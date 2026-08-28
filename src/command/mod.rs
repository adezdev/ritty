//! Command construction, execution, lifecycle, and lazy subcommands.
//!
//! This module contains [`Command`], handler [`CommandContext`], type-erased
//! [`CommandOutput`], reusable [`Plugin`] lifecycle hooks, and the common
//! [`HandlerResult`] and [`BoxError`] aliases. Import from here when using the
//! logical module API instead of Ritty's root facade or prelude.

mod parser;
mod usage;

use std::sync::{Arc, OnceLock};

use self::parser::{OptionArity, probe_long, probe_short, single_char};
use crate::argument::{Arg, EnumOption, Flag, StringOption};
use crate::error::{ParseError, RunError};
use crate::matches::Matches;

/// A resolved CLI-facing help request, or none (fall through to version
/// detection, then ordinary execution). Produced by `Command::resolve_help`.
///
/// Version is not part of this tree walk: per Citty, automatic version is a
/// root-only, single-token request, checked separately by `run_cli_from`
/// after confirming no help request matched anywhere in argv.
enum Builtin<'a> {
    /// A help request targeting `command`, reachable from the root as
    /// `display_name` (space-separated, canonical names only).
    Help {
        command: &'a Command,
        display_name: String,
        inherited_version: Option<&'a str>,
    },
    /// No built-in was requested.
    None,
}

/// The outcome of `Command::run_cli_from`: a rendered built-in response
/// ready to print, or confirmation that ordinary execution already ran.
#[derive(Debug)]
enum CliAction {
    Help(String),
    Version(String),
    Ran,
}
/// The context handed to a command's handler when it runs.
///
/// `matches()` is the selected command's own parsed matches; `root_matches()`
/// is the complete top-level parse result, so a nested handler can still
/// inspect parent/global options without Ritty flattening match ownership.
///
/// # Example
///
/// ```
/// use ritty::{Arg, Command, Flag};
///
/// let command = Command::new("tool")
///     .flag(Flag::new("verbose"))
///     .command(Command::new("show").arg(Arg::new("item")).handler(|ctx| {
///         Ok((
///             ctx.command().name().to_owned(),
///             ctx.matches().argument("item").unwrap().to_owned(),
///             ctx.root_matches().flag("verbose"),
///         ))
///     }));
///
/// let output = command.run_from(["--verbose", "show", "config"])?;
/// assert_eq!(
///     output.downcast::<(String, String, bool)>().unwrap(),
///     ("show".to_owned(), "config".to_owned(), true)
/// );
/// # Ok::<(), ritty::RunError>(())
/// ```
#[derive(Debug)]
pub struct CommandContext<'a> {
    command: &'a Command,
    matches: &'a Matches,
    root_matches: &'a Matches,
}

impl<'a> CommandContext<'a> {
    /// Returns the command whose handler is executing.
    pub fn command(&self) -> &Command {
        self.command
    }

    /// Returns the executing command's own parsed matches.
    pub fn matches(&self) -> &Matches {
        self.matches
    }

    /// Returns the complete top-level parsed result.
    pub fn root_matches(&self) -> &Matches {
        self.root_matches
    }
}

/// A boxed error, as returned by a failing handler.
pub type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

/// The result returned by a command handler. Lifecycle hooks (setup,
/// cleanup, plugin setup/cleanup) use the default `T = ()`; command handlers
/// may return any `'static` `T`, which `Command::handler` erases into a
/// [`CommandOutput`].
pub type HandlerResult<T = ()> = Result<T, BoxError>;

/// A type-erased handler success value, returned by [`Command::run_from`].
///
/// Handlers may return any `'static` Rust value; `CommandOutput` stores it
/// behind `Box<dyn Any>` without requiring `Clone`, `Debug`, `Send`, or
/// `Sync` from the contained type. Recover the value with [`Self::downcast`]
/// or inspect it with [`Self::is`]/[`Self::downcast_ref`].
///
/// # Example
///
/// ```
/// use std::rc::Rc;
/// use ritty::Command;
///
/// let command = Command::new("value").handler(|_| Ok(Rc::new(String::from("local"))));
/// let output = command.run_from([] as [&str; 0])?;
///
/// assert!(output.is::<Rc<String>>());
/// assert_eq!(output.downcast_ref::<Rc<String>>().unwrap().as_str(), "local");
/// assert_eq!(output.downcast::<Rc<String>>().unwrap().as_str(), "local");
/// # Ok::<(), ritty::RunError>(())
/// ```
pub struct CommandOutput {
    value: Box<dyn std::any::Any>,
    type_name: &'static str,
}

impl CommandOutput {
    /// Wraps `value`, recording its type name for `Debug`/`type_name`.
    pub fn new<T: 'static>(value: T) -> Self {
        Self {
            value: Box::new(value),
            type_name: std::any::type_name::<T>(),
        }
    }

    /// Returns whether the contained value is of type `T`.
    pub fn is<T: 'static>(&self) -> bool {
        self.value.is::<T>()
    }

    /// Returns a reference to the contained value if it is of type `T`.
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.value.downcast_ref::<T>()
    }

    /// Consumes `self`, returning the contained value if it is of type `T`,
    /// or the original `CommandOutput` unchanged if it is not.
    pub fn downcast<T: 'static>(self) -> Result<T, Self> {
        match self.value.downcast::<T>() {
            Ok(value) => Ok(*value),
            Err(value) => Err(Self {
                value,
                type_name: self.type_name,
            }),
        }
    }

    /// Returns the type name of the contained value, as recorded by
    /// [`Self::new`].
    pub fn type_name(&self) -> &'static str {
        self.type_name
    }
}

impl std::fmt::Debug for CommandOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CommandOutput")
            .field("type_name", &self.type_name)
            .finish()
    }
}

type HandlerFn =
    dyn for<'a> Fn(&CommandContext<'a>) -> Result<CommandOutput, BoxError> + Send + Sync;

/// A shared, cloneable command handler. Returns a type-erased
/// [`CommandOutput`] so `Command` does not need to be generic; wrapped so
/// `Command` can derive a meaningful `Debug` without trying to print closure
/// internals.
#[derive(Clone)]
struct Handler(Arc<HandlerFn>);

impl std::fmt::Debug for Handler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Handler(..)")
    }
}

/// A shared, cloneable callable used for setup and cleanup hooks (command
/// and plugin alike). Unlike `Handler`, these remain strictly
/// unit-returning: lifecycle hooks do not contribute to `CommandOutput`.
#[derive(Clone)]
struct LifecycleHook(Arc<dyn for<'a> Fn(&CommandContext<'a>) -> HandlerResult + Send + Sync>);

impl std::fmt::Debug for LifecycleHook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("LifecycleHook(..)")
    }
}

type LoaderFn = dyn Fn() -> Command + Send + Sync;

/// A lazy subcommand's loader plus its one-time resolution cache. Shared
/// (via `Arc`) across clones of the placeholder `Command` that carries it,
/// so a clone never re-runs the loader independently of its origin — every
/// clone observes the same cached resolution.
#[derive(Clone)]
struct Lazy {
    loader: Arc<LoaderFn>,
    cache: Arc<OnceLock<Command>>,
}

impl std::fmt::Debug for Lazy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Lazy(..)")
    }
}
/// A reusable, named lifecycle participant attached to a `Command` via
/// [`Command::plugin`]. A plugin has no handler of its own — it only
/// contributes setup/cleanup hooks that run alongside the command's own
/// lifecycle. See [`Command::plugin`] for exact ordering.
///
/// A `Plugin` is a concrete, cloneable value: because its lifecycle hooks
/// are Arc-backed, cloning a plugin to attach it to multiple commands
/// shares its captured closure state rather than duplicating it.
///
/// # Example
///
/// ```
/// use std::sync::{Arc, Mutex};
/// use ritty::{Command, Plugin};
///
/// let events = Arc::new(Mutex::new(Vec::new()));
/// let plugin = Plugin::new("audit")
///     .setup({
///         let events = Arc::clone(&events);
///         move |_| {
///             events.lock().unwrap().push("plugin setup");
///             Ok(())
///         }
///     })
///     .cleanup({
///         let events = Arc::clone(&events);
///         move |_| {
///             events.lock().unwrap().push("plugin cleanup");
///             Ok(())
///         }
///     });
/// let command = Command::new("tool").plugin(plugin).handler({
///     let events = Arc::clone(&events);
///     move |_| {
///         events.lock().unwrap().push("handler");
///         Ok(())
///     }
/// });
///
/// command.run_from([] as [&str; 0])?;
/// assert_eq!(
///     *events.lock().unwrap(),
///     ["plugin setup", "handler", "plugin cleanup"]
/// );
/// # Ok::<(), ritty::RunError>(())
/// ```
#[derive(Debug, Clone)]
pub struct Plugin {
    name: String,
    setup: Option<LifecycleHook>,
    cleanup: Option<LifecycleHook>,
}

impl Plugin {
    /// Creates a new named plugin with no setup or cleanup hooks.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            setup: None,
            cleanup: None,
        }
    }

    /// Returns the plugin's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the plugin's setup hook, run as part of the owning command's
    /// setup phase. Ordinary captured closures are supported.
    pub fn setup<F>(mut self, setup: F) -> Self
    where
        F: for<'a> Fn(&CommandContext<'a>) -> HandlerResult + Send + Sync + 'static,
    {
        self.setup = Some(LifecycleHook(Arc::new(setup)));
        self
    }

    /// Returns whether the plugin has a setup hook set.
    pub fn has_setup(&self) -> bool {
        self.setup.is_some()
    }

    /// Sets the plugin's cleanup hook, run as part of the owning command's
    /// cleanup phase. Ordinary captured closures are supported.
    pub fn cleanup<F>(mut self, cleanup: F) -> Self
    where
        F: for<'a> Fn(&CommandContext<'a>) -> HandlerResult + Send + Sync + 'static,
    {
        self.cleanup = Some(LifecycleHook(Arc::new(cleanup)));
        self
    }

    /// Returns whether the plugin has a cleanup hook set.
    pub fn has_cleanup(&self) -> bool {
        self.cleanup.is_some()
    }
}

/// A command in a Ritty CLI application.
///
/// Builder methods define metadata, inputs, child commands, lifecycle hooks,
/// and the selected leaf handler. A command can be parsed with
/// [`Self::parse_from`], executed programmatically with [`Self::run_from`], or
/// run against process argv with [`Self::run`].
///
/// # Example
///
/// ```
/// use ritty::{Arg, Command};
///
/// let command = Command::new("echo")
///     .description("Echo a value")
///     .arg(Arg::new("value").required())
///     .handler(|ctx| Ok(ctx.matches().argument("value").unwrap().to_owned()));
///
/// let output = command.run_from(["hello"])?;
/// assert_eq!(output.downcast::<String>().unwrap(), "hello");
/// # Ok::<(), ritty::RunError>(())
/// ```
#[derive(Debug, Clone)]
pub struct Command {
    name: String,
    aliases: Vec<String>,
    description: Option<String>,
    version: Option<String>,
    subcommands: Vec<Command>,
    arguments: Vec<Arg>,
    flags: Vec<Flag>,
    options: Vec<StringOption>,
    enum_options: Vec<EnumOption>,
    default_subcommand: Option<String>,
    hidden: bool,
    handler: Option<Handler>,
    setup: Option<LifecycleHook>,
    cleanup: Option<LifecycleHook>,
    plugins: Vec<Plugin>,
    lazy: Option<Lazy>,
}

impl Command {
    /// Creates a new command.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            aliases: Vec::new(),
            description: None,
            version: None,
            subcommands: Vec::new(),
            arguments: Vec::new(),
            flags: Vec::new(),
            options: Vec::new(),
            enum_options: Vec::new(),
            default_subcommand: None,
            hidden: false,
            handler: None,
            setup: None,
            cleanup: None,
            plugins: Vec::new(),
            lazy: None,
        }
    }

    /// Adds a lazily-resolved subcommand. `loader` is an ordinary
    /// synchronous closure producing the child `Command`; it runs at most
    /// once for this shared command tree (and every clone of it — the
    /// resolution cache is `Arc`-shared, not duplicated by `Clone`), the
    /// first time the child is actually needed: selected during parsing or
    /// execution, or resolved for usage/help metadata (description,
    /// aliases, hidden state). Constructing or cloning the parent never
    /// invokes `loader`.
    ///
    /// `name` is the subcommand's canonical identity, known without ever
    /// invoking `loader` — this is what lets a direct canonical-name match
    /// select this child, or detect it colliding with a sibling, without
    /// resolving it or any unrelated lazy sibling. If the command `loader`
    /// returns has a different own name, `name` wins: the resolved
    /// command's name is overwritten to match the declared identity, so
    /// `Matches::subcommand()` always reports `name` for a selection of
    /// this child.
    ///
    /// A lazy child's own aliases are part of its loaded metadata, not its
    /// declared identity, so matching an alias (rather than the canonical
    /// name) against this child does require resolving it — and, mirroring
    /// Citty, may also resolve other not-yet-resolved lazy siblings while
    /// searching. A direct canonical-name match never pays this cost.
    ///
    /// # Example
    ///
    /// ```
    /// use ritty::{Arg, Command};
    ///
    /// let command = Command::new("ritty").lazy_command("build", || {
    ///     Command::new("build")
    ///         .description("Build the project")
    ///         .arg(Arg::new("package").default("workspace"))
    ///         .handler(|ctx| Ok(ctx.matches().argument("package").unwrap().to_owned()))
    /// });
    ///
    /// let output = command.run_from(["build", "core"])?;
    /// assert_eq!(output.downcast::<String>().unwrap(), "core");
    /// # Ok::<(), ritty::RunError>(())
    /// ```
    pub fn lazy_command<F>(mut self, name: impl Into<String>, loader: F) -> Self
    where
        F: Fn() -> Command + Send + Sync + 'static,
    {
        let mut placeholder = Command::new(name.into());
        placeholder.lazy = Some(Lazy {
            loader: Arc::new(loader),
            cache: Arc::new(OnceLock::new()),
        });
        self.subcommands.push(placeholder);
        self
    }

    /// Returns the resolved command backing `self`: `self` unchanged for an
    /// ordinary (non-lazy) command, or the cached result of running the
    /// loader for a lazy placeholder — running it first if this is the
    /// first time. The declared canonical `name` always wins over whatever
    /// name the loader's returned command carries.
    fn resolved(&self) -> &Command {
        match &self.lazy {
            None => self,
            Some(lazy) => lazy.cache.get_or_init(|| {
                let mut resolved = (lazy.loader)();
                resolved.name = self.name.clone();
                resolved
            }),
        }
    }

    /// Aliases already known without invoking a lazy loader: an ordinary
    /// command's own aliases, or — for a lazy child — its aliases if it has
    /// already been resolved for some other reason, `false` otherwise.
    /// Never triggers resolution itself.
    fn has_known_alias(&self, name: &str) -> bool {
        match &self.lazy {
            None => self.aliases.iter().any(|a| a == name),
            Some(lazy) => lazy
                .cache
                .get()
                .is_some_and(|resolved| resolved.aliases.iter().any(|a| a == name)),
        }
    }

    /// Sets the command's handler, invoked when this command is selected for
    /// execution by `run_from`. Ordinary captured closures are supported.
    ///
    /// The handler may return any `'static` success value `T` — including
    /// `()`, as in `.handler(|_ctx| Ok(()))` — which `run_from` reports back
    /// as a type-erased [`CommandOutput`]. The value itself is never
    /// required to be `Clone`, `Debug`, `Send`, or `Sync`; only the handler
    /// closure carries those bounds.
    pub fn handler<F, T>(mut self, handler: F) -> Self
    where
        T: 'static,
        F: for<'a> Fn(&CommandContext<'a>) -> HandlerResult<T> + Send + Sync + 'static,
    {
        self.handler = Some(Handler(Arc::new(move |ctx| {
            handler(ctx).map(CommandOutput::new)
        })));
        self
    }

    /// Returns whether the command has a handler set. For a lazy
    /// subcommand this resolves it.
    pub fn has_handler(&self) -> bool {
        self.resolved().handler.is_some()
    }

    /// Sets the command's setup hook, run before its selected child (if any)
    /// or its own handler. Setup hooks run root-to-leaf along the selected
    /// path. If setup fails, this command's child is not entered and its
    /// handler is not invoked, but its cleanup hook (and every ancestor's
    /// cleanup hook already entered) still runs.
    pub fn setup<F>(mut self, setup: F) -> Self
    where
        F: for<'a> Fn(&CommandContext<'a>) -> HandlerResult + Send + Sync + 'static,
    {
        self.setup = Some(LifecycleHook(Arc::new(setup)));
        self
    }

    /// Returns whether the command has a setup hook set. For a lazy
    /// subcommand this resolves it.
    pub fn has_setup(&self) -> bool {
        self.resolved().setup.is_some()
    }

    /// Sets the command's cleanup hook. Cleanup is attempted for every
    /// entered command even when setup, nested execution, or its selected
    /// handler returns an error; cleanup hooks run leaf-to-root along the
    /// selected path. A cleanup failure never overwrites an earlier
    /// setup/handler/nested-execution failure.
    pub fn cleanup<F>(mut self, cleanup: F) -> Self
    where
        F: for<'a> Fn(&CommandContext<'a>) -> HandlerResult + Send + Sync + 'static,
    {
        self.cleanup = Some(LifecycleHook(Arc::new(cleanup)));
        self
    }

    /// Returns whether the command has a cleanup hook set. For a lazy
    /// subcommand this resolves it.
    pub fn has_cleanup(&self) -> bool {
        self.resolved().cleanup.is_some()
    }

    /// Attaches a plugin to the command. Repeated calls append; declaration
    /// order determines setup order (and, reversed, cleanup order). Plugins
    /// are not deduplicated by name — two plugins sharing a name are kept as
    /// distinct entries.
    ///
    /// Lifecycle order for one command:
    ///
    /// ```text
    /// plugin setups (declaration order)
    /// → command setup
    /// → selected work (handler or child)
    /// → command cleanup
    /// → plugin cleanups (reverse declaration order)
    /// ```
    ///
    /// If a plugin's setup fails, later plugin setups and the command's own
    /// setup/work are skipped — but command cleanup and every registered
    /// plugin's cleanup are still attempted, including plugins whose setup
    /// never ran. The first failure established (setup or cleanup) remains
    /// the primary `RunError`; later cleanup failures never replace it.
    pub fn plugin(mut self, plugin: Plugin) -> Self {
        self.plugins.push(plugin);
        self
    }

    /// Returns the command's attached plugins, in declaration order. For a
    /// lazy subcommand this resolves it.
    pub fn plugins(&self) -> &[Plugin] {
        &self.resolved().plugins
    }

    /// Marks the command as hidden from generated usage/help listings.
    /// Hidden is presentation-only: it has no effect on parsing — a hidden
    /// subcommand remains selectable by name, alias, or as a default subcommand.
    pub fn hidden(mut self) -> Self {
        self.hidden = true;
        self
    }

    /// Returns whether the command is hidden from usage/help listings. For
    /// a lazy subcommand this resolves it — hidden is loaded metadata, like
    /// description and aliases.
    pub fn is_hidden(&self) -> bool {
        self.resolved().hidden
    }

    /// Adds a subcommand alias. Aliases are exact and case-sensitive.
    pub fn alias(mut self, alias: impl Into<String>) -> Self {
        self.aliases.push(alias.into());
        self
    }

    /// Returns the command's aliases, in insertion order. For a lazy
    /// subcommand this resolves it, since aliases are loaded metadata, not
    /// part of its declared identity — see [`Command::lazy_command`].
    pub fn aliases(&self) -> &[String] {
        &self.resolved().aliases
    }

    /// Sets the command description.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the command version.
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Adds a subcommand.
    pub fn command(mut self, command: Command) -> Self {
        self.subcommands.push(command);
        self
    }

    /// Returns the command's subcommands. Entries for not-yet-selected lazy
    /// subcommands are returned as unresolved placeholders — their own
    /// `name()` is always the declared canonical identity, but other
    /// metadata (aliases, description, nested subcommands, ...) resolves
    /// lazily the first time it's asked for.
    pub fn subcommands(&self) -> &[Command] {
        &self.resolved().subcommands
    }

    /// Sets the subcommand to select automatically when no explicit child
    /// is chosen. May name a child's canonical name or one of its aliases;
    /// resolved at parse time.
    pub fn default_subcommand(mut self, name: impl Into<String>) -> Self {
        self.default_subcommand = Some(name.into());
        self
    }

    /// Returns the configured default-subcommand spelling, if any.
    pub fn get_default_subcommand(&self) -> Option<&str> {
        self.resolved().default_subcommand.as_deref()
    }

    /// Returns subcommands eligible for usage/help listings, in declaration
    /// order. Called only while rendering usage for an already-resolved
    /// command, so resolving each child here (via `is_hidden()`) to read
    /// its real hidden state is the same trade Citty's usage rendering
    /// makes — it needs the metadata regardless.
    fn visible_subcommands(&self) -> impl Iterator<Item = &Command> {
        self.subcommands.iter().filter(|c| !c.is_hidden())
    }

    /// Returns every subcommand whose canonical name or an alias matches
    /// `name`, erring on the side of never resolving an unrelated lazy
    /// sibling when a free (no-loader) match already answers the question.
    ///
    /// Every subcommand's canonical `name` is always known without
    /// resolving it (declared identity), so a direct name match is always
    /// free. Aliases are declared identity for an ordinary command but
    /// loaded metadata for a lazy one, so:
    ///
    /// - if any sibling matches by canonical name, or by an alias already
    ///   known without resolving (an ordinary command's own aliases, or a
    ///   lazy sibling already resolved for another reason), that's the
    ///   answer — no further resolution happens;
    /// - only when nothing matches for free do we fall back to resolving
    ///   every not-yet-resolved lazy sibling to search their aliases too,
    ///   mirroring Citty: alias lookup lives in child metadata, so it may
    ///   resolve children a direct canonical-name match never would.
    fn subcommands_matching(&self, name: &str) -> Vec<&Command> {
        let free: Vec<&Command> = self
            .subcommands
            .iter()
            .filter(|command| command.name() == name || command.has_known_alias(name))
            .collect();

        if !free.is_empty() {
            return free;
        }

        self.subcommands
            .iter()
            .filter(|command| command.resolved().aliases.iter().any(|a| a == name))
            .collect()
    }

    /// Adds a positional argument.
    pub fn arg(mut self, arg: Arg) -> Self {
        self.arguments.push(arg);
        self
    }

    /// Returns the command's positional arguments. For a lazy subcommand
    /// this resolves it.
    pub fn arguments(&self) -> &[Arg] {
        &self.resolved().arguments
    }

    /// Adds a flag.
    pub fn flag(mut self, flag: Flag) -> Self {
        self.flags.push(flag);
        self
    }

    /// Returns the command's flags. For a lazy subcommand this resolves it.
    pub fn flags(&self) -> &[Flag] {
        &self.resolved().flags
    }

    /// Adds a string option.
    pub fn option(mut self, option: StringOption) -> Self {
        self.options.push(option);
        self
    }

    /// Returns the command's string options. For a lazy subcommand this
    /// resolves it.
    pub fn options(&self) -> &[StringOption] {
        &self.resolved().options
    }

    /// Adds an enum option.
    pub fn enum_option(mut self, option: EnumOption) -> Self {
        self.enum_options.push(option);
        self
    }

    /// Returns the command's enum options. For a lazy subcommand this
    /// resolves it.
    pub fn enum_options(&self) -> &[EnumOption] {
        &self.resolved().enum_options
    }
    /// Parses `args` and executes the selected command's handler.
    ///
    /// Parsing runs exactly once; execution then traverses `Command` and the
    /// resulting `Matches` tree together, following the canonical subcommand
    /// selection parsing already made (explicit, aliased, or default) rather
    /// than re-examining argv. Only the selected leaf's handler runs — a
    /// parent's handler is not invoked when a child is selected, and its
    /// [`CommandOutput`] propagates back through every ancestor unchanged to
    /// become the result of this call. An empty command with no handler, or
    /// a selected leaf with no handler and no subcommands, succeeds with a
    /// unit `CommandOutput`.
    ///
    /// This is a synchronous, programmatic API: it does not read
    /// `std::env::args`, print, exit, render usage, or special-case
    /// help/version.
    ///
    /// Divergence from Citty v0.2.2: upstream's `runCommand` recurses into a
    /// selected subcommand without assigning its returned `result`, so a
    /// parent's programmatic result is always `undefined` when a child ran.
    /// Ritty's execution model invokes only the selected leaf's handler, so
    /// the coherent behavior is to propagate that leaf's output through
    /// every parent frame — this is a deliberate divergence, not an
    /// oversight.
    ///
    /// # Example
    ///
    /// ```
    /// use ritty::{Arg, Command};
    ///
    /// let command = Command::new("length")
    ///     .arg(Arg::new("value").required())
    ///     .handler(|ctx| Ok(ctx.matches().argument("value").unwrap().len()));
    ///
    /// let output = command.run_from(["ritty"])?;
    /// assert_eq!(output.downcast::<usize>().unwrap(), 5);
    /// # Ok::<(), ritty::RunError>(())
    /// ```
    pub fn run_from<I, S>(&self, args: I) -> Result<CommandOutput, RunError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let matches = self.parse_from(args)?;
        self.execute(&matches, &matches)
    }

    /// Whether the long spelling `--{long_name}` is available as a CLI
    /// built-in for this command. Declaring `long_name` itself (canonical or
    /// alias) disables both the long and short built-in spellings, so this
    /// only checks long ownership.
    fn builtin_long_enabled(&self, long_name: &str) -> bool {
        !self.owns_long(long_name)
    }

    /// Whether the short spelling `-{short}` is available as a CLI built-in
    /// for this command. Disabled by owning either the long name itself or
    /// the short spelling.
    fn builtin_short_enabled(&self, long_name: &str, short: char) -> bool {
        !self.owns_long(long_name) && !self.owns_short(short)
    }

    /// Walks `args` against this command tree looking for a built-in
    /// `--help`/`-h` request, without running the ordinary parser (which
    /// would reject an undeclared `--help` as an unknown option). Reuses
    /// `probe_long`/`probe_short`'s option-arity knowledge to skip over
    /// parent option values correctly, and follows only explicit subcommand
    /// tokens — never a default-subcommand chain — so a bare `--help` never
    /// silently targets a default child.
    ///
    /// Version is not resolved here: Citty's automatic version is a
    /// root-only, single-token request, evaluated by `run_cli_from` only
    /// after this walk finds no help request.
    fn resolve_help<'a>(&'a self, args: &[String]) -> Result<Builtin<'a>, ParseError> {
        let mut command = self.resolved();
        let mut display_name = self.name().to_owned();
        let mut inherited_version: Option<&'a str> = None;
        let mut terminated = false;
        let mut index = 0;

        while index < args.len() {
            let arg = args[index].as_str();

            if !terminated && arg == "--" {
                terminated = true;
                index += 1;
                continue;
            }

            if terminated {
                index += 1;
                continue;
            }

            if arg == "--help" && command.builtin_long_enabled("help") {
                return Ok(Builtin::Help {
                    command,
                    display_name,
                    inherited_version,
                });
            }

            if arg == "-h" && command.builtin_short_enabled("help", 'h') {
                return Ok(Builtin::Help {
                    command,
                    display_name,
                    inherited_version,
                });
            }

            if let Some(rest) = arg.strip_prefix("--") {
                if rest.contains('=') {
                    index += 1;
                    continue;
                }

                index += match probe_long(command, rest)? {
                    Some(OptionArity::Value) => 2,
                    Some(OptionArity::Flag) | None => 1,
                };
                continue;
            }

            if let Some(rest) = arg.strip_prefix('-') {
                if rest.contains('=') {
                    index += 1;
                    continue;
                }

                let arity = single_char(rest)
                    .map(|short| probe_short(command, short))
                    .transpose()?
                    .flatten();

                index += match arity {
                    Some(OptionArity::Value) => 2,
                    Some(OptionArity::Flag) | None => 1,
                };
                continue;
            }

            let candidates = command.subcommands_matching(arg);
            if let [next] = candidates[..] {
                inherited_version = command.version.as_deref().or(inherited_version);
                display_name.push(' ');
                display_name.push_str(next.name());
                // Resolve now: the walk continues probing option arity
                // against this child's own schema, which for a lazy child
                // only exists once resolved. `.name()` above is unaffected.
                command = next.resolved();
            }

            index += 1;
        }

        Ok(Builtin::None)
    }

    /// Whether `token` is this (root) command's enabled automatic-version
    /// spelling. Only meaningful as the sole element of argv — see
    /// `run_cli_from`.
    fn builtin_version_token(&self, token: &str) -> bool {
        (token == "--version" && self.builtin_long_enabled("version"))
            || (token == "-v" && self.builtin_short_enabled("version", 'v'))
    }

    /// Resolves built-ins against `args`, falling through to literal
    /// `run_from`-equivalent execution when none apply. Kept separate from
    /// `run()` so the CLI dispatcher is testable without touching
    /// `std::env::args`.
    ///
    /// Matches Citty's dispatch precedence: a help request anywhere in argv
    /// wins first; only when none is found does the exact-one-token,
    /// root-only automatic version rule apply; everything else falls
    /// through to ordinary parsing/execution.
    fn run_cli_from<I, S>(&self, args: I) -> Result<CliAction, RunError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let args: Vec<String> = args
            .into_iter()
            .map(|arg| arg.as_ref().to_owned())
            .collect();

        match self.resolve_help(&args)? {
            Builtin::Help {
                command,
                display_name,
                inherited_version,
            } => Ok(CliAction::Help(
                command.render_usage_named(&display_name, inherited_version),
            )),
            Builtin::None => {
                let this = self.resolved();
                if let [token] = args.as_slice()
                    && this.builtin_version_token(token)
                {
                    return match this.version.as_deref() {
                        Some(version) => Ok(CliAction::Version(version.to_owned())),
                        None => Err(RunError::NoVersion),
                    };
                }

                // The programmatic result is a `run_from` concern, not CLI
                // stdout — discard it here.
                self.run_from(args)?;
                Ok(CliAction::Ran)
            }
        }
    }

    /// Runs the command against the current process's argv (`argv[1..]`),
    /// intercepting built-in `--help`/`-h`/`--version`/`-v` requests before
    /// ordinary parsing/execution begins — so a successful built-in request
    /// runs zero handler, setup, cleanup, or plugin hooks. `run_from` remains
    /// the literal, programmatic counterpart and never special-cases these
    /// spellings.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ritty::{Command, RunError};
    ///
    /// fn main() -> Result<(), RunError> {
    ///     Command::new("tool")
    ///         .version("0.1.0")
    ///         .handler(|_| Ok(()))
    ///         .run()
    /// }
    /// ```
    pub fn run(&self) -> Result<(), RunError> {
        let args: Vec<String> = std::env::args().skip(1).collect();

        match self.run_cli_from(args)? {
            CliAction::Help(text) | CliAction::Version(text) => {
                use std::io::Write;
                writeln!(std::io::stdout(), "{text}").map_err(RunError::Io)
            }
            CliAction::Ran => Ok(()),
        }
    }

    /// Executes the command selected by `matches`, recursing into a selected
    /// child by its canonical name rather than re-resolving aliases.
    ///
    /// Plugin setups (declaration order) run before this command's own
    /// setup, which runs before entering a selected child or invoking this
    /// command's own handler (root-to-leaf along the selected path); cleanup
    /// is always attempted afterward, even if plugin setup, setup, the
    /// child, or the handler failed — command cleanup first, then plugin
    /// cleanups in reverse declaration order (leaf-to-root across nesting).
    /// A cleanup failure never overwrites an earlier primary failure.
    fn execute<'a>(
        &'a self,
        matches: &'a Matches,
        root_matches: &'a Matches,
    ) -> Result<CommandOutput, RunError> {
        // Resolve once up front: a lazy leaf's real lifecycle (plugins,
        // setup/cleanup, handler) is what must run, never a placeholder's
        // (empty) one. Cached, so this is a no-op after the first call for
        // a given lazy child, and a no-op always for an ordinary command.
        let this = self.resolved();

        let context = CommandContext {
            command: this,
            matches,
            root_matches,
        };

        let mut primary: Result<CommandOutput, RunError> = Ok(CommandOutput::new(()));

        for plugin in &this.plugins {
            if let Some(setup) = &plugin.setup
                && let Err(err) = (setup.0)(&context)
            {
                primary = Err(RunError::PluginSetup {
                    plugin: plugin.name.clone(),
                    source: err,
                });
                break;
            }
        }

        if primary.is_ok()
            && let Some(setup) = &this.setup
            && let Err(err) = (setup.0)(&context)
        {
            primary = Err(RunError::Setup(err));
        }

        if primary.is_ok() {
            primary = this.execute_work(matches, root_matches, &context);
        }

        if let Some(cleanup) = &this.cleanup
            && let Err(err) = (cleanup.0)(&context)
            && primary.is_ok()
        {
            primary = Err(RunError::Cleanup(err));
        }

        for plugin in this.plugins.iter().rev() {
            if let Some(cleanup) = &plugin.cleanup
                && let Err(err) = (cleanup.0)(&context)
                && primary.is_ok()
            {
                primary = Err(RunError::PluginCleanup {
                    plugin: plugin.name.clone(),
                    source: err,
                });
            }
        }

        primary
    }

    /// Runs the work phase for an entered command whose setup succeeded:
    /// recurse into a selected child, run this command's own handler, or
    /// report `NoCommand`/no-op as appropriate.
    fn execute_work<'a>(
        &'a self,
        matches: &'a Matches,
        root_matches: &'a Matches,
        context: &CommandContext<'a>,
    ) -> Result<CommandOutput, RunError> {
        if let Some(name) = matches.subcommand() {
            let child = self
                .subcommands
                .iter()
                .find(|child| child.name() == name)
                .expect("a parser-selected canonical subcommand always exists in the command tree");
            let child_matches = matches
                .subcommand_matches()
                .expect("a selected subcommand always carries its own matches");
            return child.execute(child_matches, root_matches);
        }

        if let Some(handler) = &self.handler {
            return (handler.0)(context).map_err(RunError::Handler);
        }

        if self.subcommands.is_empty() {
            return Ok(CommandOutput::new(()));
        }

        Err(RunError::NoCommand)
    }

    /// Returns the command name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the command description. For a lazy subcommand this
    /// resolves it, since the description is loaded metadata.
    pub fn get_description(&self) -> Option<&str> {
        self.resolved().description.as_deref()
    }

    /// Returns the command version. For a lazy subcommand this resolves
    /// it, since the version is loaded metadata.
    pub fn get_version(&self) -> Option<&str> {
        self.resolved().version.as_deref()
    }
}

#[cfg(test)]
mod tests;

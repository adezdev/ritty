use std::collections::HashSet;
use std::iter::once;

use super::Command;
use super::parser::single_char;
use crate::argument::{Arg, EnumOption, Flag, StringOption};

/// A two-column usage row: left-hand label and right-hand detail. An empty
/// detail means the row has no second column.
type UsageRow = (String, String);

/// Renders rows as a deterministic, left-aligned two-column block, indented
/// two spaces, with no trailing whitespace on rows that lack a detail.
fn render_rows(rows: &[UsageRow]) -> String {
    let width = rows
        .iter()
        .map(|(label, _)| label.chars().count())
        .max()
        .unwrap_or(0);
    rows.iter()
        .map(|(label, detail)| {
            if detail.is_empty() {
                format!("  {label}")
            } else {
                format!("  {label:width$}  {detail}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn dedup_strings(items: Vec<String>) -> Vec<String> {
    let mut seen = HashSet::new();
    items
        .into_iter()
        .filter(|item| seen.insert(item.clone()))
        .collect()
}

fn dedup_chars(items: Vec<char>) -> Vec<char> {
    let mut seen = HashSet::new();
    items
        .into_iter()
        .filter(|item| seen.insert(*item))
        .collect()
}

/// Single-character aliases, which the parser also accepts as short options.
fn short_aliases(aliases: &[String]) -> Vec<char> {
    aliases
        .iter()
        .filter_map(|a| single_char(a.as_str()))
        .collect()
}

/// Multi-character aliases, which the parser treats as long-option spellings.
fn long_aliases(aliases: &[String]) -> Vec<&str> {
    aliases
        .iter()
        .filter(|a| a.chars().count() > 1)
        .map(String::as_str)
        .collect()
}

/// Long spellings in display order: aliases first (insertion order), then
/// the canonical name, deduplicated.
fn long_spellings(canonical: &str, aliases: &[String]) -> Vec<String> {
    let names = long_aliases(aliases)
        .into_iter()
        .map(String::from)
        .chain(once(canonical.to_string()))
        .collect();
    dedup_strings(names)
}

/// Joins short and long spellings into one display string, e.g.
/// `-o, --output=<dir>`. Only the canonical long spelling carries the value
/// marker; aliases are shown bare.
fn format_spellings(
    shorts: &[char],
    longs: &[String],
    value_marker: Option<&str>,
    canonical: &str,
) -> String {
    let mut parts: Vec<String> = shorts.iter().map(|c| format!("-{c}")).collect();
    parts.extend(longs.iter().map(|name| match value_marker {
        Some(marker) if name.as_str() == canonical => format!("--{name}={marker}"),
        _ => format!("--{name}"),
    }));
    parts.join(", ")
}

/// `(Required)` when required and unsatisfied by a default; `(Default: x)`
/// when a default exists. A default always satisfies the requirement, so the
/// two markers are mutually exclusive.
fn required_or_default_annotation(required: bool, default: Option<&str>) -> Option<String> {
    match default {
        Some(default) => Some(format!("(Default: {default})")),
        None if required => Some("(Required)".to_string()),
        None => None,
    }
}

fn combine_description(description: Option<&str>, annotation: Option<String>) -> String {
    match (description, annotation) {
        (Some(d), Some(a)) => format!("{d} {a}"),
        (Some(d), None) => d.to_string(),
        (None, Some(a)) => a,
        (None, None) => String::new(),
    }
}

fn render_argument_row(arg: &Arg) -> UsageRow {
    let mut label = arg.name().to_uppercase();
    if let Some(hint) = arg.get_value_hint() {
        label.push_str(&format!(" <{hint}>"));
    }
    let annotation = required_or_default_annotation(arg.is_required(), arg.default_value());
    (
        label,
        combine_description(arg.get_description(), annotation),
    )
}

fn render_string_option_row(option: &StringOption) -> UsageRow {
    let shorts = dedup_chars(short_aliases(option.aliases()));
    let longs = long_spellings(option.name(), option.aliases());
    let marker = format!("<{}>", option.get_value_hint().unwrap_or(option.name()));
    let label = format_spellings(&shorts, &longs, Some(&marker), option.name());
    let annotation = required_or_default_annotation(option.is_required(), option.default_value());
    (
        label,
        combine_description(option.get_description(), annotation),
    )
}

fn render_enum_option_row(option: &EnumOption) -> UsageRow {
    let shorts = dedup_chars(short_aliases(option.aliases()));
    let longs = long_spellings(option.name(), option.aliases());
    let marker = format!("<{}>", option.values().join("|"));
    let label = format_spellings(&shorts, &longs, Some(&marker), option.name());
    let annotation = required_or_default_annotation(option.is_required(), option.default_value());
    (
        label,
        combine_description(option.get_description(), annotation),
    )
}

/// Whether a `--no-*` row should be rendered for this flag: it must be
/// parseable (the canonical name doesn't already read as a negation) and
/// either default to `true` or carry an explicit negative description.
fn negative_row_eligible(flag: &Flag) -> bool {
    (flag.default_value() == Some(true) || flag.get_negative_description().is_some())
        && !flag.name().starts_with("no-")
}

/// `--no-*` spellings the parser actually accepts: `--no-` plus every alias
/// (the parser's long-option matching does not distinguish alias length for
/// negation, so even a one-character alias yields a valid `--no-x`) and the
/// canonical name. A dedicated `.short()` is not an alias and so does not by
/// itself produce a `--no-*` spelling.
fn negative_spellings(flag: &Flag) -> Vec<String> {
    let names = flag
        .aliases()
        .iter()
        .cloned()
        .chain(once(flag.name().to_string()))
        .collect();
    dedup_strings(names)
        .into_iter()
        .map(|n| format!("--no-{n}"))
        .collect()
}

fn render_flag_rows(flag: &Flag) -> Vec<UsageRow> {
    let shorts = dedup_chars(
        flag.short_name()
            .into_iter()
            .chain(short_aliases(flag.aliases()))
            .collect(),
    );
    let longs = long_spellings(flag.name(), flag.aliases());
    let label = format_spellings(&shorts, &longs, None, flag.name());
    let default_str = flag.default_value().map(|b| b.to_string());
    let annotation = required_or_default_annotation(flag.is_required(), default_str.as_deref());

    let mut rows = vec![(
        label,
        combine_description(flag.get_description(), annotation),
    )];

    if negative_row_eligible(flag) {
        let neg_label = negative_spellings(flag).join(", ");
        let neg_detail = flag.get_negative_description().unwrap_or("").to_string();
        rows.push((neg_label, neg_detail));
    }

    rows
}

fn render_command_rows(command: &Command) -> Vec<UsageRow> {
    command
        .visible_subcommands()
        .map(|c| {
            let mut label = c.name().to_string();
            for alias in c.aliases() {
                label.push_str(", ");
                label.push_str(alias);
            }
            (label, c.get_description().unwrap_or("").to_string())
        })
        .collect()
}

/// `{description} ({name} v{version})`, degrading cleanly when either piece
/// is absent; `None` when neither is present.
fn render_header(description: Option<&str>, name: &str, version: Option<&str>) -> Option<String> {
    match (description, version) {
        (Some(d), Some(v)) => Some(format!("{d} ({name} v{v})")),
        (Some(d), None) => Some(d.to_string()),
        (None, Some(v)) => Some(format!("{name} v{v}")),
        (None, None) => None,
    }
}

fn synopsis_required_options(command: &Command) -> Vec<String> {
    let mut items = Vec::new();

    for flag in command.flags() {
        if flag.is_required() && flag.default_value().is_none() {
            items.push(format!("--{}", flag.name()));
        }
    }

    for option in command.options() {
        if option.is_required() && option.default_value().is_none() {
            let marker = option.get_value_hint().unwrap_or(option.name());
            items.push(format!("--{}=<{}>", option.name(), marker));
        }
    }

    for option in command.enum_options() {
        if option.is_required() && option.default_value().is_none() {
            items.push(format!(
                "--{}=<{}>",
                option.name(),
                option.values().join("|")
            ));
        }
    }

    items
}

fn synopsis_positionals(command: &Command) -> Vec<String> {
    command
        .arguments()
        .iter()
        .map(|arg| {
            let name = arg.name().to_uppercase();
            if arg.is_required() && arg.default_value().is_none() {
                format!("<{name}>")
            } else {
                format!("[{name}]")
            }
        })
        .collect()
}

/// Flattens every visible subcommand's canonical name and aliases into one
/// pipe-joined alternative expression, e.g. `build|b|test|t`.
fn synopsis_command_alternatives(command: &Command) -> Option<String> {
    let tokens: Vec<String> = command
        .visible_subcommands()
        .flat_map(|c| once(c.name().to_string()).chain(c.aliases().iter().cloned()))
        .collect();

    if tokens.is_empty() {
        None
    } else {
        Some(tokens.join("|"))
    }
}

fn render_usage_line(command: &Command, display_name: &str) -> String {
    let mut parts = vec!["USAGE".to_string(), display_name.to_string()];

    let has_options = !command.flags().is_empty()
        || !command.options().is_empty()
        || !command.enum_options().is_empty();
    if has_options {
        parts.push("[OPTIONS]".to_string());
    }

    parts.extend(synopsis_required_options(command));
    parts.extend(synopsis_positionals(command));

    if let Some(alternatives) = synopsis_command_alternatives(command) {
        parts.push(alternatives);
    }

    parts.join(" ")
}

impl Command {
    /// Renders usage for this command under `display_name`, falling back to
    /// `inherited_version` when this command declares none of its own. The
    /// `display_name`/`inherited_version` split exists so a future nested
    /// help traversal can render e.g. `root remote add` without redesigning
    /// the renderer.
    pub(super) fn render_usage_named(
        &self,
        display_name: &str,
        inherited_version: Option<&str>,
    ) -> String {
        let effective_version = self.version.as_deref().or(inherited_version);
        let mut sections = Vec::new();

        if let Some(header) =
            render_header(self.description.as_deref(), display_name, effective_version)
        {
            sections.push(header);
        }

        sections.push(render_usage_line(self, display_name));

        if !self.arguments.is_empty() {
            let rows: Vec<UsageRow> = self.arguments.iter().map(render_argument_row).collect();
            sections.push(format!("ARGUMENTS\n\n{}", render_rows(&rows)));
        }

        let mut option_rows: Vec<UsageRow> = Vec::new();
        for flag in &self.flags {
            option_rows.extend(render_flag_rows(flag));
        }
        for option in &self.options {
            option_rows.push(render_string_option_row(option));
        }
        for option in &self.enum_options {
            option_rows.push(render_enum_option_row(option));
        }
        if !option_rows.is_empty() {
            sections.push(format!("OPTIONS\n\n{}", render_rows(&option_rows)));
        }

        let command_rows = render_command_rows(self);
        if !command_rows.is_empty() {
            sections.push(format!("COMMANDS\n\n{}", render_rows(&command_rows)));
        }

        sections.join("\n\n")
    }

    /// Renders deterministic, plain-text usage for this command. Hidden
    /// subcommands are omitted from the listing and the synopsis, but remain
    /// fully parseable — hidden is presentation-only.
    ///
    /// Rendering can resolve lazy children because their descriptions,
    /// aliases, and hidden state are loaded metadata; the resolved command is
    /// then retained in its shared cache.
    ///
    /// # Example
    ///
    /// ```
    /// use ritty::{Arg, Command, Flag};
    ///
    /// let command = Command::new("greet")
    ///     .description("Print a greeting")
    ///     .arg(Arg::new("name").default("world"))
    ///     .flag(Flag::new("excited").short('e'));
    /// let usage = command.render_usage();
    ///
    /// assert!(usage.contains("USAGE greet [OPTIONS] [NAME]"));
    /// assert!(usage.contains("-e, --excited"));
    /// ```
    pub fn render_usage(&self) -> String {
        let this = self.resolved();
        this.render_usage_named(this.name(), None)
    }

    /// Writes the rendered usage to stdout, propagating any I/O error rather
    /// than panicking.
    ///
    /// ```no_run
    /// use ritty::Command;
    ///
    /// Command::new("tool").show_usage()?;
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn show_usage(&self) -> std::io::Result<()> {
        use std::io::Write;
        writeln!(std::io::stdout(), "{}", self.render_usage())
    }
}

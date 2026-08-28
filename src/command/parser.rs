use super::Command;
use crate::argument::{EnumOption, Flag, StringOption};
use crate::error::{ArgumentErrorKind, ParseError, ParseErrorKind};
use crate::matches::Matches;

/// Resolution of a bare long-option token to either a flag (with its
/// effective positive/negative value) or a string option.
enum LongMatch<'a> {
    Flag(&'a Flag, bool),
    Option(&'a StringOption),
    EnumOption(&'a EnumOption),
}

/// Resolution of a bare short-option token to a flag, string option, or enum option.
enum ShortMatch<'a> {
    Flag(&'a Flag),
    Option(&'a StringOption),
    EnumOption(&'a EnumOption),
}

/// Whether a recognized option consumes a following token as its value.
pub(super) enum OptionArity {
    Flag,
    Value,
}

/// The schema entry (if any) owning an `=value` spelling: a boolean flag,
/// a string option, or an enum option.
type EqualsCandidates<'a> = (
    Option<&'a Flag>,
    Option<&'a StringOption>,
    Option<&'a EnumOption>,
);

/// Determines whether `name` is recognized as a bare long option by
/// `command`, or transitively by its default-subcommand chain, and if so
/// with what arity. Used to decide, at the level currently being parsed, how
/// many raw tokens to hold back for a default child without fully resolving
/// the option there — the child (or its own default chain) re-resolves and
/// consumes the held-back tokens itself.
pub(super) fn probe_long(command: &Command, name: &str) -> Result<Option<OptionArity>, ParseError> {
    if let Some(m) = command.resolve_long(name)? {
        return Ok(Some(match m {
            LongMatch::Flag(_, _) => OptionArity::Flag,
            LongMatch::Option(_) | LongMatch::EnumOption(_) => OptionArity::Value,
        }));
    }

    match command.resolve_default_child()? {
        Some(next) => probe_long(next, name),
        None => Ok(None),
    }
}

/// Short-option counterpart to `probe_long`.
pub(super) fn probe_short(
    command: &Command,
    short: char,
) -> Result<Option<OptionArity>, ParseError> {
    if let Some(m) = command.resolve_short(short)? {
        return Ok(Some(match m {
            ShortMatch::Flag(_) => OptionArity::Flag,
            ShortMatch::Option(_) | ShortMatch::EnumOption(_) => OptionArity::Value,
        }));
    }

    match command.resolve_default_child()? {
        Some(next) => probe_short(next, short),
        None => Ok(None),
    }
}

/// Determines whether `--name=value` is recognized by `command`, or
/// transitively by its default-subcommand chain.
fn probe_long_equals(command: &Command, name: &str) -> Result<bool, ParseError> {
    let (flag_match, string_match, enum_match) = command.long_equals_candidates(name)?;
    if flag_match.is_some() || string_match.is_some() || enum_match.is_some() {
        return Ok(true);
    }

    match command.resolve_default_child()? {
        Some(next) => probe_long_equals(next, name),
        None => Ok(false),
    }
}

/// Short-option counterpart to `probe_long_equals`.
fn probe_short_equals(command: &Command, short: char) -> Result<bool, ParseError> {
    let (flag_match, string_match, enum_match) = command.short_equals_candidates(short)?;
    if flag_match.is_some() || string_match.is_some() || enum_match.is_some() {
        return Ok(true);
    }

    match command.resolve_default_child()? {
        Some(next) => probe_short_equals(next, short),
        None => Ok(false),
    }
}

/// Parses an explicit `=value` boolean literal. Ritty accepts only the exact
/// lowercase strings `true`/`false` (unlike Citty v0.2.2, which treats any
/// string other than `"false"` as true) so a typo is reported rather than
/// silently coerced to `true`.
fn parse_bool_literal(spelling: &str, value: &str) -> Result<bool, ParseError> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(ParseError::new(
            ParseErrorKind::Argument(ArgumentErrorKind::InvalidOptionValue),
            format!("invalid value for option: {spelling}: {value} (expected true or false)"),
        )),
    }
}

/// Validates an enum option's effective value against its allowed values.
/// An empty allowed-value list means there is no restriction.
fn validate_enum_value(option: &EnumOption, value: &str) -> Result<(), ParseError> {
    if option.values().is_empty() || option.values().iter().any(|allowed| allowed == value) {
        return Ok(());
    }

    Err(ParseError::new(
        ParseErrorKind::Argument(ArgumentErrorKind::InvalidOptionValue),
        format!(
            "invalid value for option: --{}: {} (expected one of: {})",
            option.name(),
            value,
            option.values().join(", ")
        ),
    ))
}

/// Extracts `value` as a single Unicode scalar, the definition of one
/// short-option character shared by parsing and usage rendering.
pub(super) fn single_char(value: &str) -> Option<char> {
    let mut chars = value.chars();
    let first = chars.next()?;
    if chars.next().is_some() {
        return None;
    }
    Some(first)
}

impl Command {
    /// Returns every string option whose canonical name or an alias matches `name`.
    fn options_matching_long(&self, name: &str) -> Vec<&StringOption> {
        self.options
            .iter()
            .filter(|option| option.name() == name || option.aliases().iter().any(|a| a == name))
            .collect()
    }

    /// Returns every string option that declares `short` as a single-character alias.
    fn options_matching_short(&self, short: char) -> Vec<&StringOption> {
        self.options
            .iter()
            .filter(|option| {
                option
                    .aliases()
                    .iter()
                    .any(|alias| single_char(alias) == Some(short))
            })
            .collect()
    }

    /// Returns every enum option whose canonical name or an alias matches `name`.
    fn enum_options_matching_long(&self, name: &str) -> Vec<&EnumOption> {
        self.enum_options
            .iter()
            .filter(|option| option.name() == name || option.aliases().iter().any(|a| a == name))
            .collect()
    }

    /// Returns every enum option that declares `short` as a single-character alias.
    fn enum_options_matching_short(&self, short: char) -> Vec<&EnumOption> {
        self.enum_options
            .iter()
            .filter(|option| {
                option
                    .aliases()
                    .iter()
                    .any(|alias| single_char(alias) == Some(short))
            })
            .collect()
    }

    /// Returns every flag whose canonical name or an alias matches `name`.
    fn flags_matching_long(&self, name: &str) -> Vec<&Flag> {
        self.flags
            .iter()
            .filter(|flag| flag.name() == name || flag.aliases().iter().any(|a| a == name))
            .collect()
    }

    /// Returns every flag that declares `short` as its dedicated short name
    /// or as a single-character alias.
    fn flags_matching_short(&self, short: char) -> Vec<&Flag> {
        self.flags
            .iter()
            .filter(|flag| {
                flag.short_name() == Some(short)
                    || flag
                        .aliases()
                        .iter()
                        .any(|alias| single_char(alias) == Some(short))
            })
            .collect()
    }

    /// Whether this command's own schema (a flag, string option, or enum
    /// option — canonical name or alias) already claims the long spelling
    /// `--{name}`, independent of arity. Used to decide whether a built-in
    /// like `--help` would collide with a user-declared option.
    pub(super) fn owns_long(&self, name: &str) -> bool {
        !self.flags_matching_long(name).is_empty()
            || !self.options_matching_long(name).is_empty()
            || !self.enum_options_matching_long(name).is_empty()
    }

    /// Short-option counterpart to `owns_long`.
    pub(super) fn owns_short(&self, short: char) -> bool {
        !self.flags_matching_short(short).is_empty()
            || !self.options_matching_short(short).is_empty()
            || !self.enum_options_matching_short(short).is_empty()
    }

    /// Resolves a bare long-option token (no `--` prefix, no `=`) against
    /// declared flags (positive and `no-*` negation) and string options,
    /// erroring on ambiguity rather than silently preferring one schema.
    fn resolve_long(&self, name: &str) -> Result<Option<LongMatch<'_>>, ParseError> {
        let positive_flags = self.flags_matching_long(name);
        let string_options = self.options_matching_long(name);
        let enum_options = self.enum_options_matching_long(name);
        let negative_flags = name
            .strip_prefix("no-")
            .map(|base| self.flags_matching_long(base))
            .unwrap_or_default();

        if positive_flags.len() + string_options.len() + enum_options.len() + negative_flags.len()
            > 1
        {
            return Err(ParseError::new(
                ParseErrorKind::Argument(ArgumentErrorKind::AmbiguousOption),
                format!("ambiguous option: --{name}"),
            ));
        }

        if let Some(flag) = positive_flags.first() {
            return Ok(Some(LongMatch::Flag(flag, true)));
        }

        if let Some(option) = string_options.first() {
            return Ok(Some(LongMatch::Option(option)));
        }

        if let Some(option) = enum_options.first() {
            return Ok(Some(LongMatch::EnumOption(option)));
        }

        if let Some(flag) = negative_flags.first() {
            return Ok(Some(LongMatch::Flag(flag, false)));
        }

        Ok(None)
    }

    /// Resolves a bare short-option token (no `=`) against declared flags,
    /// string options, and enum options, erroring on ambiguity.
    fn resolve_short(&self, short: char) -> Result<Option<ShortMatch<'_>>, ParseError> {
        let flag_candidates = self.flags_matching_short(short);
        let option_candidates = self.options_matching_short(short);
        let enum_candidates = self.enum_options_matching_short(short);

        if flag_candidates.len() + option_candidates.len() + enum_candidates.len() > 1 {
            return Err(ParseError::new(
                ParseErrorKind::Argument(ArgumentErrorKind::AmbiguousOption),
                format!("ambiguous option: -{short}"),
            ));
        }

        if let Some(flag) = flag_candidates.first() {
            return Ok(Some(ShortMatch::Flag(flag)));
        }

        if let Some(option) = option_candidates.first() {
            return Ok(Some(ShortMatch::Option(option)));
        }

        if let Some(option) = enum_candidates.first() {
            return Ok(Some(ShortMatch::EnumOption(option)));
        }

        Ok(None)
    }

    /// Resolves a `--name=value` long-option token to whichever schema entry
    /// owns `name`: a boolean flag, or a value-bearing string/enum option.
    /// A `--no-*` negation participates only in the ambiguity count — an
    /// explicit value is never valid for the negative spelling, so it is
    /// never returned as a flag candidate here.
    fn long_equals_candidates(&self, name: &str) -> Result<EqualsCandidates<'_>, ParseError> {
        let positive_flags = self.flags_matching_long(name);
        let negative_flags = name
            .strip_prefix("no-")
            .map(|base| self.flags_matching_long(base))
            .unwrap_or_default();
        let string_candidates = self.options_matching_long(name);
        let enum_candidates = self.enum_options_matching_long(name);

        if positive_flags.len()
            + negative_flags.len()
            + string_candidates.len()
            + enum_candidates.len()
            > 1
        {
            return Err(ParseError::new(
                ParseErrorKind::Argument(ArgumentErrorKind::AmbiguousOption),
                format!("ambiguous option: --{name}"),
            ));
        }

        Ok((
            positive_flags.first().copied(),
            string_candidates.first().copied(),
            enum_candidates.first().copied(),
        ))
    }

    /// Resolves a `-x=value` short-option token, mirroring `long_equals_candidates`.
    fn short_equals_candidates(&self, short: char) -> Result<EqualsCandidates<'_>, ParseError> {
        let flag_candidates = self.flags_matching_short(short);
        let string_candidates = self.options_matching_short(short);
        let enum_candidates = self.enum_options_matching_short(short);

        if flag_candidates.len() + string_candidates.len() + enum_candidates.len() > 1 {
            return Err(ParseError::new(
                ParseErrorKind::Argument(ArgumentErrorKind::AmbiguousOption),
                format!("ambiguous option: -{short}"),
            ));
        }

        Ok((
            flag_candidates.first().copied(),
            string_candidates.first().copied(),
            enum_candidates.first().copied(),
        ))
    }

    /// Resolves the configured default subcommand to its concrete child,
    /// re-checking alias/name collisions at each call site that needs it.
    /// Returns `Ok(None)` when no default subcommand is configured.
    fn resolve_default_child(&self) -> Result<Option<&Command>, ParseError> {
        let Some(default_name) = &self.default_subcommand else {
            return Ok(None);
        };

        let candidates = self.subcommands_matching(default_name);

        if candidates.len() > 1 {
            return Err(ParseError::new(
                ParseErrorKind::AmbiguousCommand,
                format!("ambiguous command: {default_name}"),
            ));
        }

        let child = candidates.first().copied().ok_or_else(|| {
            ParseError::new(
                ParseErrorKind::DefaultSubcommandNotFound,
                format!("default subcommand not found: {default_name}"),
            )
        })?;

        // Every caller immediately treats the default child as "the command
        // now being parsed/probed" — hand back its resolved form so a lazy
        // default child's real schema is what gets consulted.
        Ok(Some(child.resolved()))
    }

    /// Parses command-line arguments.
    pub fn parse_from<I, S>(&self, args: I) -> Result<Matches, ParseError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let args: Vec<String> = args
            .into_iter()
            .map(|arg| arg.as_ref().to_owned())
            .collect();
        self.resolved().parse_tokens(&args)
    }
    /// Parses a slice of already-collected argv tokens against this command,
    /// recursing into a selected subcommand's own tokens once one is found.
    ///
    /// When no explicit child is selected and a default subcommand is
    /// configured, tokens this command's own schema does not recognize are
    /// held back into `child_tokens` and handed to the default child's own
    /// `parse_tokens` afterward, rather than being replayed verbatim the way
    /// Citty's `strict: false` parser does. This keeps unknown-input errors
    /// meaningful (a token accepted by neither command still errors) while
    /// letting the default child receive real argv instead of only defaults.
    fn parse_tokens(&self, args: &[String]) -> Result<Matches, ParseError> {
        let mut matches = Matches::new();
        let mut positional = 0;
        let mut terminated = false;
        let mut child_tokens: Vec<String> = Vec::new();
        let mut child_terminator_sent = false;

        let mut index = 0;

        while index < args.len() {
            let arg = args[index].as_str();

            if !terminated && arg == "--" {
                terminated = true;
                index += 1;
                continue;
            }

            if !terminated {
                if let Some(rest) = arg.strip_prefix("--") {
                    if let Some((name, value)) = rest.split_once('=') {
                        let (flag_match, string_match, enum_match) =
                            self.long_equals_candidates(name)?;

                        if let Some(flag) = flag_match {
                            let parsed = parse_bool_literal(&format!("--{name}"), value)?;
                            matches.set_flag(flag.name(), parsed);
                            index += 1;
                            continue;
                        }

                        if let Some(option) = string_match {
                            matches.push_option(option.name().to_owned(), value.to_owned());
                            index += 1;
                            continue;
                        }

                        if let Some(option) = enum_match {
                            matches.push_enum_option(option.name().to_owned(), value.to_owned());
                            index += 1;
                            continue;
                        }

                        if let Some(child) = self.resolve_default_child()?
                            && probe_long_equals(child, name)?
                        {
                            child_tokens.push(arg.to_owned());
                            index += 1;
                            continue;
                        }

                        return Err(ParseError::new(
                            ParseErrorKind::Argument(ArgumentErrorKind::UnknownOption),
                            format!("unknown flag: --{name}"),
                        ));
                    }

                    let name = rest;

                    match self.resolve_long(name)? {
                        Some(LongMatch::Flag(flag, value)) => {
                            matches.set_flag(flag.name(), value);
                            index += 1;
                            continue;
                        }
                        Some(LongMatch::Option(option)) => {
                            let value = args.get(index + 1).ok_or_else(|| {
                                ParseError::new(
                                    ParseErrorKind::Argument(ArgumentErrorKind::MissingOptionValue),
                                    format!("missing value for option: --{name}"),
                                )
                            })?;
                            matches.push_option(option.name().to_owned(), value.to_owned());
                            index += 2;
                            continue;
                        }
                        Some(LongMatch::EnumOption(option)) => {
                            let value = args.get(index + 1).ok_or_else(|| {
                                ParseError::new(
                                    ParseErrorKind::Argument(ArgumentErrorKind::MissingOptionValue),
                                    format!("missing value for option: --{name}"),
                                )
                            })?;
                            matches.push_enum_option(option.name().to_owned(), value.to_owned());
                            index += 2;
                            continue;
                        }
                        None => {
                            if let Some(child) = self.resolve_default_child()? {
                                match probe_long(child, name)? {
                                    Some(OptionArity::Flag) => {
                                        child_tokens.push(arg.to_owned());
                                        index += 1;
                                        continue;
                                    }
                                    Some(OptionArity::Value) => {
                                        child_tokens.push(arg.to_owned());
                                        if let Some(value) = args.get(index + 1) {
                                            child_tokens.push(value.clone());
                                            index += 2;
                                        } else {
                                            index += 1;
                                        }
                                        continue;
                                    }
                                    None => {}
                                }
                            }

                            return Err(ParseError::new(
                                ParseErrorKind::Argument(ArgumentErrorKind::UnknownOption),
                                format!("unknown flag: --{name}"),
                            ));
                        }
                    }
                }

                if let Some(rest) = arg.strip_prefix('-') {
                    if let Some((name, value)) = rest.split_once('=') {
                        if let Some(short) = single_char(name) {
                            let (flag_match, string_match, enum_match) =
                                self.short_equals_candidates(short)?;

                            if let Some(flag) = flag_match {
                                let parsed = parse_bool_literal(&format!("-{short}"), value)?;
                                matches.set_flag(flag.name(), parsed);
                                index += 1;
                                continue;
                            }

                            if let Some(option) = string_match {
                                matches.push_option(option.name().to_owned(), value.to_owned());
                                index += 1;
                                continue;
                            }

                            if let Some(option) = enum_match {
                                matches
                                    .push_enum_option(option.name().to_owned(), value.to_owned());
                                index += 1;
                                continue;
                            }

                            if let Some(child) = self.resolve_default_child()?
                                && probe_short_equals(child, short)?
                            {
                                child_tokens.push(arg.to_owned());
                                index += 1;
                                continue;
                            }
                        }

                        return Err(ParseError::new(
                            ParseErrorKind::Argument(ArgumentErrorKind::UnknownOption),
                            format!("unknown flag: -{rest}"),
                        ));
                    }

                    if let Some(short) = single_char(rest) {
                        match self.resolve_short(short)? {
                            Some(ShortMatch::Flag(flag)) => {
                                matches.set_flag(flag.name(), true);
                                index += 1;
                                continue;
                            }
                            Some(ShortMatch::Option(option)) => {
                                let value = args.get(index + 1).ok_or_else(|| {
                                    ParseError::new(
                                        ParseErrorKind::Argument(
                                            ArgumentErrorKind::MissingOptionValue,
                                        ),
                                        format!("missing value for option: -{short}"),
                                    )
                                })?;
                                matches.push_option(option.name().to_owned(), value.to_owned());
                                index += 2;
                                continue;
                            }
                            Some(ShortMatch::EnumOption(option)) => {
                                let value = args.get(index + 1).ok_or_else(|| {
                                    ParseError::new(
                                        ParseErrorKind::Argument(
                                            ArgumentErrorKind::MissingOptionValue,
                                        ),
                                        format!("missing value for option: -{short}"),
                                    )
                                })?;
                                matches
                                    .push_enum_option(option.name().to_owned(), value.to_owned());
                                index += 2;
                                continue;
                            }
                            None => {
                                if let Some(child) = self.resolve_default_child()? {
                                    match probe_short(child, short)? {
                                        Some(OptionArity::Flag) => {
                                            child_tokens.push(arg.to_owned());
                                            index += 1;
                                            continue;
                                        }
                                        Some(OptionArity::Value) => {
                                            child_tokens.push(arg.to_owned());
                                            if let Some(value) = args.get(index + 1) {
                                                child_tokens.push(value.clone());
                                                index += 2;
                                            } else {
                                                index += 1;
                                            }
                                            continue;
                                        }
                                        None => {}
                                    }
                                }
                            }
                        }
                    }

                    return Err(ParseError::new(
                        ParseErrorKind::Argument(ArgumentErrorKind::UnknownOption),
                        format!("unknown flag: -{rest}"),
                    ));
                }

                let candidates = self.subcommands_matching(arg);

                if candidates.len() > 1 {
                    return Err(ParseError::new(
                        ParseErrorKind::AmbiguousCommand,
                        format!("ambiguous command: {arg}"),
                    ));
                }

                if let Some(child) = candidates.first() {
                    // Resolve now: a lazy child's own schema (flags,
                    // options, positionals, nested subcommands) is what
                    // must parse its remaining tokens, never a
                    // placeholder's empty one. `.name()` is unaffected —
                    // it's the declared canonical identity either way.
                    let child = child.resolved();
                    self.finalize(&mut matches, positional)?;
                    let child_matches = child.parse_tokens(&args[index + 1..])?;
                    matches.select_subcommand(child.name().to_owned(), child_matches);
                    return Ok(matches);
                }
            }

            if let Some(argument) = self.arguments.get(positional) {
                matches.push_argument(argument.name().to_owned(), arg.to_owned());
                positional += 1;
                index += 1;
                continue;
            }

            // A bare token neither self nor an explicit child claims falls
            // through to the default child, whatever its own positional
            // capacity (or further default chain) turns out to be — the
            // child's own `parse_tokens` call is the one that decides
            // whether it fits or is itself excess.
            if self.resolve_default_child()?.is_some() {
                if terminated && !child_terminator_sent {
                    child_tokens.push("--".to_owned());
                    child_terminator_sent = true;
                }
                child_tokens.push(arg.to_owned());
                index += 1;
                continue;
            }

            if !self.subcommands.is_empty() {
                return Err(ParseError::new(
                    ParseErrorKind::UnknownCommand,
                    format!("unknown command: {arg}"),
                ));
            }

            return Err(ParseError::new(
                ParseErrorKind::Argument(ArgumentErrorKind::UnexpectedArgument),
                format!("unexpected argument: {arg}"),
            ));
        }

        self.finalize(&mut matches, positional)?;

        if self.default_subcommand.is_some() {
            let child = self
                .resolve_default_child()?
                .expect("resolve_default_child returns Some or errors when configured");

            let child_matches = child.parse_tokens(&child_tokens)?;
            matches.select_subcommand(child.name().to_owned(), child_matches);
        }

        Ok(matches)
    }

    /// Applies positional/flag/option/enum defaults and required checks for
    /// this command's own schema against tokens already consumed into
    /// `matches`. Runs once the current command's own token prefix is fully
    /// parsed, before any recursion into a selected child's tokens.
    fn finalize(&self, matches: &mut Matches, positional: usize) -> Result<(), ParseError> {
        for argument in self.arguments.iter().skip(positional) {
            match argument.default_value() {
                Some(default) => {
                    matches.push_argument(argument.name().to_owned(), default.to_owned())
                }
                None if argument.is_required() => {
                    return Err(ParseError::new(
                        ParseErrorKind::Argument(ArgumentErrorKind::MissingRequiredArgument),
                        format!("missing required argument: {}", argument.name()),
                    ));
                }
                None => {}
            }
        }

        for flag in &self.flags {
            if matches.has_flag(flag.name()) {
                continue;
            }

            match flag.default_value() {
                Some(default) => matches.set_flag(flag.name(), default),
                None if flag.is_required() => {
                    return Err(ParseError::new(
                        ParseErrorKind::Argument(ArgumentErrorKind::MissingRequiredOption),
                        format!("missing required option: --{}", flag.name()),
                    ));
                }
                None => {}
            }
        }

        for option in &self.options {
            if matches.option(option.name()).is_some() {
                continue;
            }

            match option.default_value() {
                Some(default) => matches.push_option(option.name().to_owned(), default.to_owned()),
                None if option.is_required() => {
                    return Err(ParseError::new(
                        ParseErrorKind::Argument(ArgumentErrorKind::MissingRequiredOption),
                        format!("missing required option: --{}", option.name()),
                    ));
                }
                None => {}
            }
        }

        for option in &self.enum_options {
            if matches.enum_option(option.name()).is_some() {
                continue;
            }

            match option.default_value() {
                Some(default) => {
                    matches.push_enum_option(option.name().to_owned(), default.to_owned())
                }
                None if option.is_required() => {
                    return Err(ParseError::new(
                        ParseErrorKind::Argument(ArgumentErrorKind::MissingRequiredOption),
                        format!("missing required option: --{}", option.name()),
                    ));
                }
                None => {}
            }
        }

        for option in &self.enum_options {
            if let Some(value) = matches.enum_option(option.name()) {
                validate_enum_value(option, value)?;
            }
        }

        Ok(())
    }
}

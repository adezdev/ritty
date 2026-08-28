use super::*;
use crate::{ArgumentErrorKind, ParseErrorKind};

#[test]
fn creates_command() {
    let command = Command::new("ritty");

    assert_eq!(command.name(), "ritty");
    assert_eq!(command.get_description(), None);
    assert_eq!(command.get_version(), None);
}

#[test]
fn configures_command_metadata() {
    let command = Command::new("ritty")
        .description("Elegant CLI builder for Rust")
        .version("0.1.0");

    assert_eq!(command.name(), "ritty");
    assert_eq!(
        command.get_description(),
        Some("Elegant CLI builder for Rust")
    );
    assert_eq!(command.get_version(), Some("0.1.0"));
}

#[test]
fn adds_subcommand() {
    let command = Command::new("ritty").command(Command::new("build"));

    assert_eq!(command.subcommands().len(), 1);
    assert_eq!(command.subcommands()[0].name(), "build");
}

#[test]
fn adds_argument() {
    let command = Command::new("ritty").arg(Arg::new("name"));

    assert_eq!(command.arguments().len(), 1);
    assert_eq!(command.arguments()[0].name(), "name");
}

#[test]
fn adds_flag() {
    let command = Command::new("ritty").flag(Flag::new("verbose").short('v'));

    assert_eq!(command.flags().len(), 1);
    assert_eq!(command.flags()[0].name(), "verbose");
    assert_eq!(command.flags()[0].short_name(), Some('v'));
}

#[test]
fn parses_long_flag() {
    let command = Command::new("ritty").flag(Flag::new("verbose"));

    let matches = command.parse_from(["--verbose"]).unwrap();

    assert!(matches.flag("verbose"));
}

#[test]
fn parses_short_flag() {
    let command = Command::new("ritty").flag(Flag::new("verbose").short('v'));

    let matches = command.parse_from(["-v"]).unwrap();

    assert!(matches.flag("verbose"));
}

#[test]
fn parses_positional_argument() {
    let command = Command::new("ritty").arg(Arg::new("name"));

    let matches = command.parse_from(["world"]).unwrap();

    assert_eq!(matches.argument("name"), Some("world"));
}

#[test]
fn rejects_missing_required_argument() {
    let command = Command::new("ritty").arg(Arg::new("name").required());

    let error = command.parse_from([] as [&str; 0]).unwrap_err();

    assert_eq!(error.message(), "missing required argument: name");
}

#[test]
fn accepts_required_argument_when_supplied() {
    let command = Command::new("ritty").arg(Arg::new("name").required());

    let matches = command.parse_from(["world"]).unwrap();

    assert_eq!(matches.argument("name"), Some("world"));
}

#[test]
fn parses_multiple_positional_arguments_in_order() {
    let command = Command::new("ritty")
        .arg(Arg::new("first").required())
        .arg(Arg::new("second").required());

    let matches = command.parse_from(["one", "two"]).unwrap();

    assert_eq!(matches.argument("first"), Some("one"));
    assert_eq!(matches.argument("second"), Some("two"));
}

#[test]
fn rejects_missing_later_required_argument() {
    let command = Command::new("ritty")
        .arg(Arg::new("first"))
        .arg(Arg::new("second").required());

    let error = command.parse_from(["one"]).unwrap_err();

    assert_eq!(error.message(), "missing required argument: second");
}

#[test]
fn flag_does_not_satisfy_required_argument() {
    let command = Command::new("ritty")
        .arg(Arg::new("name").required())
        .flag(Flag::new("verbose"));

    let error = command.parse_from(["--verbose"]).unwrap_err();

    assert_eq!(error.message(), "missing required argument: name");
}

#[test]
fn subcommand_does_not_satisfy_required_argument() {
    let command = Command::new("ritty")
        .arg(Arg::new("name").required())
        .command(Command::new("build"));

    let error = command.parse_from(["build"]).unwrap_err();

    assert_eq!(error.message(), "missing required argument: name");
}

#[test]
fn missing_optional_positional_uses_default() {
    let command = Command::new("ritty").arg(Arg::new("name").default("world"));

    let matches = command.parse_from([] as [&str; 0]).unwrap();

    assert_eq!(matches.argument("name"), Some("world"));
}

#[test]
fn explicit_input_overrides_default() {
    let command = Command::new("ritty").arg(Arg::new("name").default("world"));

    let matches = command.parse_from(["alice"]).unwrap();

    assert_eq!(matches.argument("name"), Some("alice"));
}

#[test]
fn multiple_defaults_apply_independently() {
    let command = Command::new("ritty")
        .arg(Arg::new("first").default("a"))
        .arg(Arg::new("second").default("b"));

    let matches = command.parse_from([] as [&str; 0]).unwrap();

    assert_eq!(matches.argument("first"), Some("a"));
    assert_eq!(matches.argument("second"), Some("b"));
}

#[test]
fn mixed_explicit_and_default_values_bind_in_order() {
    let command = Command::new("ritty")
        .arg(Arg::new("first").default("a"))
        .arg(Arg::new("second").default("b"));

    let matches = command.parse_from(["x"]).unwrap();

    assert_eq!(matches.argument("first"), Some("x"));
    assert_eq!(matches.argument("second"), Some("b"));
}

#[test]
fn required_argument_with_default_is_satisfied_when_omitted() {
    let command = Command::new("ritty").arg(Arg::new("name").required().default("world"));

    let matches = command.parse_from([] as [&str; 0]).unwrap();

    assert_eq!(matches.argument("name"), Some("world"));
}

#[test]
fn flag_does_not_suppress_positional_default() {
    let command = Command::new("ritty")
        .flag(Flag::new("verbose"))
        .arg(Arg::new("name").default("world"));

    let matches = command.parse_from(["--verbose"]).unwrap();

    assert!(matches.flag("verbose"));
    assert_eq!(matches.argument("name"), Some("world"));
}

#[test]
fn subcommand_does_not_suppress_positional_default() {
    let command = Command::new("ritty")
        .command(Command::new("build"))
        .arg(Arg::new("name").default("world"));

    let matches = command.parse_from(["build"]).unwrap();

    assert_eq!(matches.subcommand(), Some("build"));
    assert_eq!(matches.argument("name"), Some("world"));
}

#[test]
fn argument_metadata_defaults_to_none() {
    let arg = Arg::new("name");

    assert_eq!(arg.get_description(), None);
    assert_eq!(arg.get_value_hint(), None);
}

#[test]
fn configures_argument_metadata() {
    let arg = Arg::new("output")
        .description("Output directory")
        .value_hint("dir")
        .required()
        .default(".");

    assert_eq!(arg.name(), "output");
    assert_eq!(arg.get_description(), Some("Output directory"));
    assert_eq!(arg.get_value_hint(), Some("dir"));
    assert!(arg.is_required());
    assert_eq!(arg.default_value(), Some("."));
}

#[test]
fn argument_metadata_does_not_affect_parsing() {
    let command = Command::new("ritty").arg(
        Arg::new("output")
            .description("Output directory")
            .value_hint("dir"),
    );

    let matches = command.parse_from(["build"]).unwrap();

    assert_eq!(matches.argument("output"), Some("build"));
}

#[test]
fn parses_subcommand() {
    let command = Command::new("ritty").command(Command::new("build"));

    let matches = command.parse_from(["build"]).unwrap();

    assert_eq!(matches.subcommand(), Some("build"));
}

#[test]
fn adds_string_option() {
    let command = Command::new("ritty").option(StringOption::new("name"));

    assert_eq!(command.options().len(), 1);
    assert_eq!(command.options()[0].name(), "name");
}

#[test]
fn parses_string_option_separate_token() {
    let command = Command::new("ritty").option(StringOption::new("name"));

    let matches = command.parse_from(["--name", "alice"]).unwrap();

    assert_eq!(matches.option("name"), Some("alice"));
}

#[test]
fn parses_string_option_equals_syntax() {
    let command = Command::new("ritty").option(StringOption::new("name"));

    let matches = command.parse_from(["--name=alice"]).unwrap();

    assert_eq!(matches.option("name"), Some("alice"));
}

#[test]
fn preserves_exact_string_option_value() {
    let command = Command::new("ritty").option(StringOption::new("name"));

    let matches = command.parse_from(["--name", "Alice-Smith"]).unwrap();

    assert_eq!(matches.option("name"), Some("Alice-Smith"));
}

#[test]
fn rejects_missing_string_option_value() {
    let command = Command::new("ritty").option(StringOption::new("name"));

    let error = command.parse_from(["--name"]).unwrap_err();

    assert_eq!(error.message(), "missing value for option: --name");
}

#[test]
fn string_option_consumes_hyphen_prefixed_value() {
    let command = Command::new("ritty").option(StringOption::new("pattern"));

    let matches = command.parse_from(["--pattern", "-foo"]).unwrap();

    assert_eq!(matches.option("pattern"), Some("-foo"));
}

#[test]
fn string_option_consumes_double_hyphen_prefixed_value() {
    let command = Command::new("ritty").option(StringOption::new("pattern"));

    let matches = command.parse_from(["--pattern", "--literal"]).unwrap();

    assert_eq!(matches.option("pattern"), Some("--literal"));
}

#[test]
fn string_option_value_is_not_mistaken_for_subcommand() {
    let command = Command::new("ritty")
        .option(StringOption::new("target"))
        .command(Command::new("build"));

    let matches = command.parse_from(["--target", "build"]).unwrap();

    assert_eq!(matches.option("target"), Some("build"));
    assert_eq!(matches.subcommand(), None);
}

#[test]
fn subcommand_resolves_after_string_option_value() {
    let command = Command::new("ritty")
        .option(StringOption::new("target"))
        .command(Command::new("build"));

    let matches = command
        .parse_from(["--target", "release", "build"])
        .unwrap();

    assert_eq!(matches.option("target"), Some("release"));
    assert_eq!(matches.subcommand(), Some("build"));
}

#[test]
fn string_option_and_boolean_flag_coexist() {
    let command = Command::new("ritty")
        .flag(Flag::new("verbose"))
        .option(StringOption::new("name"));

    let matches = command
        .parse_from(["--verbose", "--name", "alice"])
        .unwrap();

    assert!(matches.flag("verbose"));
    assert_eq!(matches.option("name"), Some("alice"));
}

#[test]
fn string_option_value_does_not_advance_positional_cursor() {
    let command = Command::new("ritty")
        .option(StringOption::new("name"))
        .arg(Arg::new("target"));

    let matches = command.parse_from(["--name", "alice", "world"]).unwrap();

    assert_eq!(matches.option("name"), Some("alice"));
    assert_eq!(matches.argument("target"), Some("world"));
}

#[test]
fn rejects_unknown_long_option() {
    let command = Command::new("ritty");

    let error = command.parse_from(["--wat"]).unwrap_err();

    assert_eq!(error.message(), "unknown flag: --wat");
}

#[test]
fn string_option_metadata_defaults_to_none() {
    let option = StringOption::new("output");

    assert!(!option.is_required());
    assert_eq!(option.default_value(), None);
    assert_eq!(option.get_description(), None);
    assert_eq!(option.get_value_hint(), None);
}

#[test]
fn configures_string_option_metadata() {
    let option = StringOption::new("output")
        .description("Output directory")
        .value_hint("dir")
        .required()
        .default(".");

    assert_eq!(option.name(), "output");
    assert_eq!(option.get_description(), Some("Output directory"));
    assert_eq!(option.get_value_hint(), Some("dir"));
    assert!(option.is_required());
    assert_eq!(option.default_value(), Some("."));
}

#[test]
fn missing_optional_string_option_remains_absent() {
    let command = Command::new("ritty").option(StringOption::new("name"));

    let matches = command.parse_from([] as [&str; 0]).unwrap();

    assert_eq!(matches.option("name"), None);
}

#[test]
fn missing_string_option_uses_default() {
    let command = Command::new("ritty").option(StringOption::new("name").default("world"));

    let matches = command.parse_from([] as [&str; 0]).unwrap();

    assert_eq!(matches.option("name"), Some("world"));
}

#[test]
fn explicit_string_option_value_overrides_default() {
    let command = Command::new("ritty").option(StringOption::new("name").default("world"));

    let separate = command.parse_from(["--name", "alice"]).unwrap();
    let equals = command.parse_from(["--name=alice"]).unwrap();

    assert_eq!(separate.option("name"), Some("alice"));
    assert_eq!(equals.option("name"), Some("alice"));
}

#[test]
fn rejects_missing_required_string_option() {
    let command = Command::new("ritty").option(StringOption::new("name").required());

    let error = command.parse_from([] as [&str; 0]).unwrap_err();

    assert_eq!(error.message(), "missing required option: --name");
}

#[test]
fn accepts_required_string_option_when_supplied() {
    let command = Command::new("ritty").option(StringOption::new("name").required());

    let matches = command.parse_from(["--name", "alice"]).unwrap();

    assert_eq!(matches.option("name"), Some("alice"));
}

#[test]
fn required_string_option_with_default_is_satisfied_when_omitted() {
    let command =
        Command::new("ritty").option(StringOption::new("name").required().default("world"));

    let matches = command.parse_from([] as [&str; 0]).unwrap();

    assert_eq!(matches.option("name"), Some("world"));
}

#[test]
fn multiple_string_option_defaults_apply_independently() {
    let command = Command::new("ritty")
        .option(StringOption::new("first").default("a"))
        .option(StringOption::new("second").default("b"));

    let matches = command.parse_from([] as [&str; 0]).unwrap();

    assert_eq!(matches.option("first"), Some("a"));
    assert_eq!(matches.option("second"), Some("b"));
}

#[test]
fn mixed_explicit_and_default_string_options() {
    let command = Command::new("ritty")
        .option(StringOption::new("first").default("a"))
        .option(StringOption::new("second").default("b"));

    let matches = command.parse_from(["--first", "x"]).unwrap();

    assert_eq!(matches.option("first"), Some("x"));
    assert_eq!(matches.option("second"), Some("b"));
}

#[test]
fn hyphen_prefixed_explicit_value_overrides_string_option_default() {
    let command = Command::new("ritty").option(StringOption::new("pattern").default("default"));

    let matches = command.parse_from(["--pattern", "--literal"]).unwrap();

    assert_eq!(matches.option("pattern"), Some("--literal"));
}

#[test]
fn flag_does_not_satisfy_required_string_option() {
    let command = Command::new("ritty")
        .option(StringOption::new("name").required())
        .flag(Flag::new("verbose"));

    let error = command.parse_from(["--verbose"]).unwrap_err();

    assert_eq!(error.message(), "missing required option: --name");
}

#[test]
fn subcommand_does_not_satisfy_required_string_option() {
    let command = Command::new("ritty")
        .option(StringOption::new("name").required())
        .command(Command::new("build"));

    let error = command.parse_from(["build"]).unwrap_err();

    assert_eq!(error.message(), "missing required option: --name");
}

#[test]
fn string_option_default_does_not_affect_positional_state() {
    let command = Command::new("ritty")
        .option(StringOption::new("name").default("world"))
        .arg(Arg::new("target").required());

    let matches = command.parse_from(["value"]).unwrap();

    assert_eq!(matches.option("name"), Some("world"));
    assert_eq!(matches.argument("target"), Some("value"));
}

#[test]
fn canonical_string_option_repeated_uses_first_occurrence() {
    // Established parser behavior: `Matches::option` looks up the
    // first stored occurrence, so the first explicit value wins.
    let command = Command::new("ritty").option(StringOption::new("output"));

    let matches = command
        .parse_from(["--output", "first", "--output", "second"])
        .unwrap();

    assert_eq!(matches.option("output"), Some("first"));
}

#[test]
fn string_option_aliases_default_to_empty() {
    let option = StringOption::new("output");

    assert!(option.aliases().is_empty());
}

#[test]
fn string_option_retains_aliases_in_order() {
    let option = StringOption::new("output").alias("o").alias("out");

    assert_eq!(option.aliases(), ["o", "out"]);
}

#[test]
fn parses_short_string_option_alias() {
    let command = Command::new("ritty").option(StringOption::new("output").alias("o"));

    let matches = command.parse_from(["-o", "dist"]).unwrap();

    assert_eq!(matches.option("output"), Some("dist"));
}

#[test]
fn parses_short_string_option_alias_with_equals() {
    let command = Command::new("ritty").option(StringOption::new("output").alias("o"));

    let matches = command.parse_from(["-o=dist"]).unwrap();

    assert_eq!(matches.option("output"), Some("dist"));
}

#[test]
fn unicode_scalar_string_option_alias_works_separate_and_equals() {
    let command = Command::new("ritty").option(StringOption::new("output").alias("é"));

    let separate = command.parse_from(["-é", "dist"]).unwrap();
    let equals = command.parse_from(["-é=dist"]).unwrap();

    assert_eq!(separate.option("output"), Some("dist"));
    assert_eq!(equals.option("output"), Some("dist"));
}

#[test]
fn multi_scalar_unicode_string_option_alias_is_not_a_short_option() {
    let command = Command::new("ritty").option(StringOption::new("output").alias("日本"));

    assert!(command.parse_from(["-日", "dist"]).is_err());
    assert!(command.parse_from(["-日=dist"]).is_err());
}

#[test]
fn short_string_option_alias_equals_value_preserves_extra_equals() {
    let command = Command::new("ritty").option(StringOption::new("output").alias("o"));

    let matches = command.parse_from(["-o=a=b"]).unwrap();

    assert_eq!(matches.option("output"), Some("a=b"));
}

#[test]
fn short_string_option_alias_equals_empty_value_is_explicit() {
    let command = Command::new("ritty").option(StringOption::new("output").alias("o"));

    let matches = command.parse_from(["-o="]).unwrap();

    assert_eq!(matches.option("output"), Some(""));
}

#[test]
fn canonical_long_option_equals_empty_value_is_explicit() {
    let command = Command::new("ritty").option(StringOption::new("name"));

    let matches = command.parse_from(["--name="]).unwrap();

    assert_eq!(matches.option("name"), Some(""));
}

#[test]
fn short_string_option_alias_consumes_hyphen_prefixed_value() {
    let command = Command::new("ritty").option(StringOption::new("output").alias("o"));

    let matches = command.parse_from(["-o", "--literal"]).unwrap();

    assert_eq!(matches.option("output"), Some("--literal"));
}

#[test]
fn parses_long_string_option_alias() {
    let command = Command::new("ritty").option(StringOption::new("output").alias("destination"));

    let matches = command.parse_from(["--destination", "dist"]).unwrap();

    assert_eq!(matches.option("output"), Some("dist"));
}

#[test]
fn parses_long_string_option_alias_with_equals() {
    let command = Command::new("ritty").option(StringOption::new("output").alias("destination"));

    let matches = command.parse_from(["--destination=dist"]).unwrap();

    assert_eq!(matches.option("output"), Some("dist"));
}

#[test]
fn all_alias_spellings_resolve_to_canonical_name() {
    let command =
        Command::new("ritty").option(StringOption::new("output").alias("o").alias("destination"));

    for args in [
        &["--output", "dist"][..],
        &["-o", "dist"][..],
        &["-o=dist"][..],
        &["--destination", "dist"][..],
        &["--destination=dist"][..],
    ] {
        let matches = command.parse_from(args.to_vec()).unwrap();
        assert_eq!(matches.option("output"), Some("dist"));
    }
}

#[test]
fn short_string_option_alias_overrides_default() {
    let command =
        Command::new("ritty").option(StringOption::new("output").alias("o").default("default"));

    let matches = command.parse_from(["-o", "explicit"]).unwrap();

    assert_eq!(matches.option("output"), Some("explicit"));
}

#[test]
fn short_string_option_alias_equals_overrides_default() {
    let command =
        Command::new("ritty").option(StringOption::new("output").alias("o").default("default"));

    let matches = command.parse_from(["-o=explicit"]).unwrap();

    assert_eq!(matches.option("output"), Some("explicit"));
}

#[test]
fn required_string_option_satisfied_through_short_alias() {
    let command = Command::new("ritty").option(StringOption::new("output").required().alias("o"));

    let matches = command.parse_from(["-o", "dist"]).unwrap();

    assert_eq!(matches.option("output"), Some("dist"));
}

#[test]
fn string_option_alias_value_is_not_mistaken_for_subcommand() {
    let command = Command::new("ritty")
        .option(StringOption::new("target").alias("t"))
        .command(Command::new("build"));

    let matches = command.parse_from(["-t", "build"]).unwrap();

    assert_eq!(matches.option("target"), Some("build"));
    assert_eq!(matches.subcommand(), None);
}

#[test]
fn subcommand_resolves_after_string_option_alias_value() {
    let command = Command::new("ritty")
        .option(StringOption::new("target").alias("t"))
        .command(Command::new("build"));

    let matches = command.parse_from(["-t", "release", "build"]).unwrap();

    assert_eq!(matches.option("target"), Some("release"));
    assert_eq!(matches.subcommand(), Some("build"));
}

#[test]
fn string_option_alias_value_does_not_advance_positional_cursor() {
    let command = Command::new("ritty")
        .option(StringOption::new("output").alias("o"))
        .arg(Arg::new("target"));

    let matches = command.parse_from(["-o", "dist", "world"]).unwrap();

    assert_eq!(matches.option("output"), Some("dist"));
    assert_eq!(matches.argument("target"), Some("world"));
}

#[test]
fn rejects_unknown_short_string_option_alias() {
    let command = Command::new("ritty");

    let error = command.parse_from(["-x"]).unwrap_err();

    assert_eq!(error.message(), "unknown flag: -x");
}

#[test]
fn rejects_unknown_long_string_option_alias() {
    let command = Command::new("ritty");

    let error = command.parse_from(["--destination"]).unwrap_err();

    assert_eq!(error.message(), "unknown flag: --destination");
}

#[test]
fn boolean_short_flag_still_works_alongside_string_option_aliases() {
    let command = Command::new("ritty")
        .flag(Flag::new("verbose").short('v'))
        .option(StringOption::new("output").alias("o"));

    let matches = command.parse_from(["-v", "-o", "dist"]).unwrap();

    assert!(matches.flag("verbose"));
    assert_eq!(matches.option("output"), Some("dist"));
}

#[test]
fn boolean_short_flag_and_string_option_alias_collision_errors() {
    let command = Command::new("ritty")
        .flag(Flag::new("verbose").short('v'))
        .option(StringOption::new("value").alias("v"));

    let error = command.parse_from(["-v"]).unwrap_err();

    assert_eq!(error.message(), "ambiguous option: -v");
}

#[test]
fn duplicate_string_option_alias_collision_errors() {
    let command = Command::new("ritty")
        .option(StringOption::new("first").alias("x"))
        .option(StringOption::new("second").alias("x"));

    let error = command.parse_from(["-x", "value"]).unwrap_err();

    assert_eq!(error.message(), "ambiguous option: -x");
}

#[test]
fn flag_metadata_defaults_to_none() {
    let flag = Flag::new("color");

    assert!(flag.aliases().is_empty());
    assert_eq!(flag.get_description(), None);
    assert_eq!(flag.get_negative_description(), None);
    assert_eq!(flag.get_value_hint(), None);
    assert!(!flag.is_required());
    assert_eq!(flag.default_value(), None);
}

#[test]
fn configures_flag_metadata() {
    let flag = Flag::new("color")
        .short('c')
        .alias("colour")
        .alias("colors")
        .description("Enable color output")
        .negative_description("Disable color output")
        .value_hint("bool")
        .required()
        .default(true);

    assert_eq!(flag.name(), "color");
    assert_eq!(flag.short_name(), Some('c'));
    assert_eq!(flag.aliases(), ["colour", "colors"]);
    assert_eq!(flag.get_description(), Some("Enable color output"));
    assert_eq!(
        flag.get_negative_description(),
        Some("Disable color output")
    );
    assert_eq!(flag.get_value_hint(), Some("bool"));
    assert!(flag.is_required());
    assert_eq!(flag.default_value(), Some(true));
}

#[test]
fn dedicated_short_still_works_alongside_aliases() {
    let command = Command::new("ritty").flag(Flag::new("verbose").short('v').alias("chatty"));

    let matches = command.parse_from(["-v"]).unwrap();

    assert!(matches.flag("verbose"));
}

#[test]
fn parses_long_flag_alias() {
    let command = Command::new("ritty").flag(Flag::new("verbose").alias("chatty"));

    let matches = command.parse_from(["--chatty"]).unwrap();

    assert!(matches.flag("verbose"));
}

#[test]
fn single_char_flag_alias_works_as_short_and_long() {
    let command = Command::new("ritty").flag(Flag::new("verbose").alias("q"));

    let short = command.parse_from(["-q"]).unwrap();
    let long = command.parse_from(["--q"]).unwrap();

    assert!(short.flag("verbose"));
    assert!(long.flag("verbose"));
}

#[test]
fn dedicated_short_accepts_multi_byte_unicode_scalar() {
    let command = Command::new("ritty").flag(Flag::new("verbose").short('é'));

    let matches = command.parse_from(["-é"]).unwrap();

    assert!(matches.flag("verbose"));
}

#[test]
fn dedicated_short_multi_byte_unicode_matches_usage_spelling() {
    let command = Command::new("ritty").flag(Flag::new("verbose").short('é'));

    assert_eq!(
        command.render_usage(),
        "USAGE ritty [OPTIONS]\n\nOPTIONS\n\n  -é, --verbose"
    );
}

#[test]
fn unicode_scalar_flag_alias_works_as_short() {
    let command = Command::new("ritty").flag(Flag::new("verbose").alias("é"));

    let matches = command.parse_from(["-é"]).unwrap();

    assert!(matches.flag("verbose"));
}

#[test]
fn multi_scalar_unicode_alias_is_not_a_short_option() {
    let command = Command::new("ritty").flag(Flag::new("verbose").alias("日本"));

    let result = command.parse_from(["-日"]);

    assert!(result.is_err());
}

#[test]
fn flag_default_true_applies_when_absent() {
    let command = Command::new("ritty").flag(Flag::new("color").default(true));

    let matches = command.parse_from([] as [&str; 0]).unwrap();

    assert!(matches.flag("color"));
}

#[test]
fn flag_default_false_applies_when_absent() {
    let command = Command::new("ritty").flag(Flag::new("color").default(false));

    let matches = command.parse_from([] as [&str; 0]).unwrap();

    assert!(!matches.flag("color"));
}

#[test]
fn explicit_positive_overrides_false_default() {
    let command = Command::new("ritty").flag(Flag::new("color").default(false));

    let matches = command.parse_from(["--color"]).unwrap();

    assert!(matches.flag("color"));
}

#[test]
fn negation_overrides_true_default() {
    let command = Command::new("ritty").flag(Flag::new("color").default(true));

    let matches = command.parse_from(["--no-color"]).unwrap();

    assert!(!matches.flag("color"));
}

#[test]
fn parses_canonical_negation() {
    let command = Command::new("ritty").flag(Flag::new("color"));

    let matches = command.parse_from(["--no-color"]).unwrap();

    assert!(!matches.flag("color"));
}

#[test]
fn parses_long_alias_negation() {
    let command = Command::new("ritty").flag(Flag::new("color").alias("colour"));

    let matches = command.parse_from(["--no-colour"]).unwrap();

    assert!(!matches.flag("color"));
}

#[test]
fn parses_single_char_alias_negation() {
    let command = Command::new("ritty").flag(Flag::new("color").alias("c"));

    let matches = command.parse_from(["--no-c"]).unwrap();

    assert!(!matches.flag("color"));
}

#[test]
fn dedicated_short_does_not_support_negation() {
    let command = Command::new("ritty").flag(Flag::new("color").short('c'));

    let error = command.parse_from(["--no-c"]).unwrap_err();

    assert_eq!(error.message(), "unknown flag: --no-c");
}

// --- Boolean `=value` coercion ---

#[test]
fn canonical_long_boolean_equals_true() {
    let command = Command::new("ritty").flag(Flag::new("force"));

    let matches = command.parse_from(["--force=true"]).unwrap();

    assert!(matches.flag("force"));
}

#[test]
fn canonical_long_boolean_equals_false() {
    let command = Command::new("ritty").flag(Flag::new("force"));

    let matches = command.parse_from(["--force=false"]).unwrap();

    assert!(!matches.flag("force"));
}

#[test]
fn long_alias_boolean_equals_normalizes_to_canonical() {
    let command = Command::new("ritty").flag(Flag::new("verbose").alias("chatty"));

    let matches = command.parse_from(["--chatty=true"]).unwrap();

    assert!(matches.flag("verbose"));

    let matches = command.parse_from(["--chatty=false"]).unwrap();

    assert!(!matches.flag("verbose"));
}

#[test]
fn dedicated_short_boolean_equals_true_and_false() {
    let command = Command::new("ritty").flag(Flag::new("verbose").short('v'));

    let matches = command.parse_from(["-v=true"]).unwrap();
    assert!(matches.flag("verbose"));

    let matches = command.parse_from(["-v=false"]).unwrap();
    assert!(!matches.flag("verbose"));
}

#[test]
fn single_char_alias_boolean_equals_true_and_false() {
    let command = Command::new("ritty").flag(Flag::new("verbose").alias("q"));

    let matches = command.parse_from(["-q=true"]).unwrap();
    assert!(matches.flag("verbose"));

    let matches = command.parse_from(["-q=false"]).unwrap();
    assert!(!matches.flag("verbose"));
}

#[test]
fn explicit_boolean_equals_overrides_true_default() {
    let command = Command::new("ritty").flag(Flag::new("install").default(true));

    let matches = command.parse_from(["--install=false"]).unwrap();

    assert!(!matches.flag("install"));
}

#[test]
fn explicit_boolean_equals_overrides_false_default() {
    let command = Command::new("ritty").flag(Flag::new("install").default(false));

    let matches = command.parse_from(["--install=true"]).unwrap();

    assert!(matches.flag("install"));
}

#[test]
fn boolean_equals_false_then_bare_positive_yields_positive() {
    let command = Command::new("ritty").flag(Flag::new("force"));

    let matches = command.parse_from(["--force=false", "--force"]).unwrap();

    assert!(matches.flag("force"));
}

#[test]
fn bare_positive_then_boolean_equals_false_yields_negative() {
    let command = Command::new("ritty").flag(Flag::new("force"));

    let matches = command.parse_from(["--force", "--force=false"]).unwrap();

    assert!(!matches.flag("force"));
}

#[test]
fn negation_then_boolean_equals_true_yields_positive() {
    let command = Command::new("ritty").flag(Flag::new("force"));

    let matches = command.parse_from(["--no-force", "--force=true"]).unwrap();

    assert!(matches.flag("force"));
}

#[test]
fn boolean_equals_true_then_negation_yields_negative() {
    let command = Command::new("ritty").flag(Flag::new("force"));

    let matches = command.parse_from(["--force=true", "--no-force"]).unwrap();

    assert!(!matches.flag("force"));
}

#[test]
fn boolean_equals_rejects_yes() {
    let command = Command::new("ritty").flag(Flag::new("force"));

    let error = command.parse_from(["--force=yes"]).unwrap_err();

    assert_eq!(
        error.kind(),
        ParseErrorKind::Argument(ArgumentErrorKind::InvalidOptionValue)
    );
    assert_eq!(
        error.message(),
        "invalid value for option: --force: yes (expected true or false)"
    );
}

#[test]
fn boolean_equals_rejects_1() {
    let command = Command::new("ritty").flag(Flag::new("force"));

    let error = command.parse_from(["--force=1"]).unwrap_err();

    assert_eq!(
        error.kind(),
        ParseErrorKind::Argument(ArgumentErrorKind::InvalidOptionValue)
    );
}

#[test]
fn boolean_equals_rejects_uppercase_true() {
    let command = Command::new("ritty").flag(Flag::new("force"));

    let error = command.parse_from(["--force=TRUE"]).unwrap_err();

    assert_eq!(
        error.kind(),
        ParseErrorKind::Argument(ArgumentErrorKind::InvalidOptionValue)
    );
}

#[test]
fn boolean_equals_rejects_empty_value() {
    let command = Command::new("ritty").flag(Flag::new("force"));

    let error = command.parse_from(["--force="]).unwrap_err();

    assert_eq!(
        error.kind(),
        ParseErrorKind::Argument(ArgumentErrorKind::InvalidOptionValue)
    );
}

#[test]
fn short_boolean_equals_invalid_value_reports_short_spelling() {
    let command = Command::new("ritty").flag(Flag::new("verbose").short('v'));

    let error = command.parse_from(["-v=yes"]).unwrap_err();

    assert_eq!(
        error.kind(),
        ParseErrorKind::Argument(ArgumentErrorKind::InvalidOptionValue)
    );
    assert_eq!(
        error.message(),
        "invalid value for option: -v: yes (expected true or false)"
    );
}

#[test]
fn negated_boolean_equals_does_not_become_valid() {
    let command = Command::new("ritty").flag(Flag::new("force"));

    let error = command.parse_from(["--no-force=true"]).unwrap_err();

    assert_eq!(
        error.kind(),
        ParseErrorKind::Argument(ArgumentErrorKind::UnknownOption)
    );
    assert_eq!(error.message(), "unknown flag: --no-force");
}

#[test]
fn negated_boolean_equals_false_does_not_become_valid() {
    let command = Command::new("ritty").flag(Flag::new("force"));

    let error = command.parse_from(["--no-force=false"]).unwrap_err();

    assert_eq!(
        error.kind(),
        ParseErrorKind::Argument(ArgumentErrorKind::UnknownOption)
    );
}

#[test]
fn boolean_equals_spelling_collides_with_string_option_remains_ambiguous() {
    let command = Command::new("ritty")
        .flag(Flag::new("mode"))
        .option(StringOption::new("mode"));

    let error = command.parse_from(["--mode=true"]).unwrap_err();

    assert_eq!(
        error.kind(),
        ParseErrorKind::Argument(ArgumentErrorKind::AmbiguousOption)
    );
}

#[test]
fn boolean_equals_spelling_collides_with_enum_option_remains_ambiguous() {
    let command = Command::new("ritty")
        .flag(Flag::new("mode"))
        .enum_option(EnumOption::new("mode", ["a", "b"]));

    let error = command.parse_from(["--mode=true"]).unwrap_err();

    assert_eq!(
        error.kind(),
        ParseErrorKind::Argument(ArgumentErrorKind::AmbiguousOption)
    );
}

#[test]
fn boolean_equals_reaches_direct_default_subcommand() {
    let command = Command::new("root")
        .default_subcommand("run")
        .command(Command::new("run").flag(Flag::new("force")));

    let matches = command.parse_from(["--force=false"]).unwrap();

    assert!(!matches.subcommand_matches().unwrap().flag("force"));
}

#[test]
fn boolean_equals_reaches_nested_default_subcommand_chain() {
    let command = Command::new("root").default_subcommand("run").command(
        Command::new("run")
            .default_subcommand("fast")
            .command(Command::new("fast").flag(Flag::new("force"))),
    );

    let matches = command.parse_from(["--force=true"]).unwrap();

    assert!(
        matches
            .subcommand_matches()
            .unwrap()
            .subcommand_matches()
            .unwrap()
            .flag("force")
    );
}

#[test]
fn long_alias_boolean_equals_reaches_default_subcommand() {
    let command = Command::new("root")
        .default_subcommand("run")
        .command(Command::new("run").flag(Flag::new("verbose").alias("chatty")));

    let matches = command.parse_from(["--chatty=false"]).unwrap();

    assert!(!matches.subcommand_matches().unwrap().flag("verbose"));
}

#[test]
fn short_boolean_equals_reaches_default_subcommand() {
    let command = Command::new("root")
        .default_subcommand("run")
        .command(Command::new("run").flag(Flag::new("verbose").short('v')));

    let matches = command.parse_from(["-v=true"]).unwrap();

    assert!(matches.subcommand_matches().unwrap().flag("verbose"));
}

#[test]
fn builtin_help_skips_boolean_long_equals_before_child() {
    let command = Command::new("ritty")
        .flag(Flag::new("force"))
        .command(Command::new("remote"));

    let action = command
        .run_cli_from(["--force=false", "remote", "--help"])
        .unwrap();

    match action {
        CliAction::Help(text) => assert!(text.contains("ritty remote")),
        _ => panic!("expected CliAction::Help"),
    }
}

#[test]
fn builtin_help_skips_boolean_alias_long_equals_before_child() {
    let command = Command::new("ritty")
        .flag(Flag::new("force").alias("f"))
        .command(Command::new("remote"));

    let action = command
        .run_cli_from(["--f=false", "remote", "--help"])
        .unwrap();

    match action {
        CliAction::Help(text) => assert!(text.contains("ritty remote")),
        _ => panic!("expected CliAction::Help"),
    }
}

#[test]
fn string_option_equals_syntax_still_works_alongside_boolean_equals() {
    let command = Command::new("ritty")
        .flag(Flag::new("force"))
        .option(StringOption::new("name"));

    let matches = command
        .parse_from(["--force=true", "--name=value"])
        .unwrap();

    assert!(matches.flag("force"));
    assert_eq!(matches.option("name"), Some("value"));
}

#[test]
fn short_option_equals_syntax_still_works_alongside_boolean_equals() {
    let command = Command::new("ritty")
        .flag(Flag::new("force").short('f'))
        .option(StringOption::new("name").alias("n"));

    let matches = command.parse_from(["-f=true", "-n=value"]).unwrap();

    assert!(matches.flag("force"));
    assert_eq!(matches.option("name"), Some("value"));
}

#[test]
fn missing_string_option_value_remains_strict_error() {
    let command = Command::new("ritty").option(StringOption::new("output"));

    let error = command.parse_from(["--output"]).unwrap_err();

    assert_eq!(
        error.kind(),
        ParseErrorKind::Argument(ArgumentErrorKind::MissingOptionValue)
    );
}

#[test]
fn missing_short_string_option_value_remains_strict_error() {
    let command = Command::new("ritty").option(StringOption::new("output").alias("o"));

    let error = command.parse_from(["-o"]).unwrap_err();

    assert_eq!(
        error.kind(),
        ParseErrorKind::Argument(ArgumentErrorKind::MissingOptionValue)
    );
}

#[test]
fn rejects_missing_required_flag() {
    let command = Command::new("ritty").flag(Flag::new("confirm").required());

    let error = command.parse_from([] as [&str; 0]).unwrap_err();

    assert_eq!(error.message(), "missing required option: --confirm");
}

#[test]
fn required_flag_satisfied_by_positive() {
    let command = Command::new("ritty").flag(Flag::new("confirm").required());

    let matches = command.parse_from(["--confirm"]).unwrap();

    assert!(matches.flag("confirm"));
}

#[test]
fn required_flag_satisfied_by_negation() {
    let command = Command::new("ritty").flag(Flag::new("confirm").required());

    let matches = command.parse_from(["--no-confirm"]).unwrap();

    assert!(!matches.flag("confirm"));
}

#[test]
fn required_flag_satisfied_by_default() {
    let command = Command::new("ritty").flag(Flag::new("confirm").required().default(false));

    let matches = command.parse_from([] as [&str; 0]).unwrap();

    assert!(!matches.flag("confirm"));
}

#[test]
fn repeated_positive_then_negative_yields_negative() {
    let command = Command::new("ritty").flag(Flag::new("verbose"));

    let matches = command.parse_from(["--verbose", "--no-verbose"]).unwrap();

    assert!(!matches.flag("verbose"));
}

#[test]
fn repeated_negative_then_positive_yields_positive() {
    let command = Command::new("ritty").flag(Flag::new("verbose"));

    let matches = command.parse_from(["--no-verbose", "--verbose"]).unwrap();

    assert!(matches.flag("verbose"));
}

#[test]
fn rejects_unknown_negation() {
    let command = Command::new("ritty").flag(Flag::new("color"));

    let error = command.parse_from(["--no-verbose"]).unwrap_err();

    assert_eq!(error.message(), "unknown flag: --no-verbose");
}

#[test]
fn two_boolean_long_aliases_colliding_errors() {
    let command = Command::new("ritty")
        .flag(Flag::new("first").alias("x"))
        .flag(Flag::new("second").alias("x"));

    let error = command.parse_from(["--x"]).unwrap_err();

    assert_eq!(error.message(), "ambiguous option: --x");
}

#[test]
fn two_boolean_short_spellings_colliding_errors() {
    let command = Command::new("ritty")
        .flag(Flag::new("first").short('x'))
        .flag(Flag::new("second").short('x'));

    let error = command.parse_from(["-x"]).unwrap_err();

    assert_eq!(error.message(), "ambiguous option: -x");
}

#[test]
fn boolean_long_and_string_long_alias_collision_errors() {
    let command = Command::new("ritty")
        .flag(Flag::new("verbose").alias("mode"))
        .option(StringOption::new("output").alias("mode"));

    let error = command.parse_from(["--mode"]).unwrap_err();

    assert_eq!(error.message(), "ambiguous option: --mode");
}

#[test]
fn flag_named_no_cache_parses_as_exact_positive() {
    let command = Command::new("ritty").flag(Flag::new("no-cache"));

    let matches = command.parse_from(["--no-cache"]).unwrap();

    assert!(matches.flag("no-cache"));
}

#[test]
fn no_cache_ambiguous_between_positive_and_negation() {
    let command = Command::new("ritty")
        .flag(Flag::new("cache"))
        .flag(Flag::new("no-cache"));

    let error = command.parse_from(["--no-cache"]).unwrap_err();

    assert_eq!(error.message(), "ambiguous option: --no-cache");
}

#[test]
fn boolean_negation_and_string_option_exact_collision_errors() {
    let command = Command::new("ritty")
        .flag(Flag::new("cache"))
        .option(StringOption::new("no-cache"));

    let error = command.parse_from(["--no-cache"]).unwrap_err();

    assert_eq!(error.message(), "ambiguous option: --no-cache");
}

#[test]
fn exact_string_option_no_cache_works_without_boolean_collision() {
    let command = Command::new("ritty").option(StringOption::new("no-cache"));

    let matches = command.parse_from(["--no-cache", "value"]).unwrap();

    assert_eq!(matches.option("no-cache"), Some("value"));
}

#[test]
fn boolean_positive_followed_by_subcommand() {
    let command = Command::new("ritty")
        .flag(Flag::new("verbose"))
        .command(Command::new("build"));

    let matches = command.parse_from(["--verbose", "build"]).unwrap();

    assert!(matches.flag("verbose"));
    assert_eq!(matches.subcommand(), Some("build"));
}

#[test]
fn boolean_negative_followed_by_subcommand() {
    let command = Command::new("ritty")
        .flag(Flag::new("verbose"))
        .command(Command::new("build"));

    let matches = command.parse_from(["--no-verbose", "build"]).unwrap();

    assert!(!matches.flag("verbose"));
    assert_eq!(matches.subcommand(), Some("build"));
}

#[test]
fn boolean_negation_does_not_advance_positional_cursor() {
    let command = Command::new("ritty")
        .flag(Flag::new("verbose"))
        .arg(Arg::new("target"));

    let matches = command.parse_from(["--no-verbose", "world"]).unwrap();

    assert!(!matches.flag("verbose"));
    assert_eq!(matches.argument("target"), Some("world"));
}

#[test]
fn flag_metadata_does_not_affect_parsing() {
    let command = Command::new("ritty").flag(
        Flag::new("color")
            .description("Enable colors")
            .negative_description("Disable colors")
            .value_hint("bool"),
    );

    let matches = command.parse_from(["--no-color"]).unwrap();

    assert!(!matches.flag("color"));
}

#[test]
fn adds_enum_option_and_retains_allowed_values_in_order() {
    let command = Command::new("ritty")
        .enum_option(EnumOption::new("level", ["debug", "info", "warn", "error"]));

    assert_eq!(command.enum_options().len(), 1);
    assert_eq!(command.enum_options()[0].name(), "level");
    assert_eq!(
        command.enum_options()[0].values(),
        ["debug", "info", "warn", "error"]
    );
}

#[test]
fn enum_option_metadata_defaults_to_none() {
    let option = EnumOption::new("level", ["debug", "info"]);

    assert!(option.aliases().is_empty());
    assert_eq!(option.get_description(), None);
    assert_eq!(option.get_value_hint(), None);
    assert!(!option.is_required());
    assert_eq!(option.default_value(), None);
}

#[test]
fn configures_enum_option_metadata() {
    let option = EnumOption::new("level", ["debug", "info", "warn", "error"])
        .alias("l")
        .alias("log-level")
        .description("Logging level")
        .value_hint("level")
        .required()
        .default("info");

    assert_eq!(option.name(), "level");
    assert_eq!(option.aliases(), ["l", "log-level"]);
    assert_eq!(option.get_description(), Some("Logging level"));
    assert_eq!(option.get_value_hint(), Some("level"));
    assert!(option.is_required());
    assert_eq!(option.default_value(), Some("info"));
}

#[test]
fn parses_enum_option_separate_token_and_equals() {
    let command = Command::new("ritty").enum_option(EnumOption::new("level", ["debug", "info"]));

    let separate = command.parse_from(["--level", "info"]).unwrap();
    let equals = command.parse_from(["--level=info"]).unwrap();

    assert_eq!(separate.enum_option("level"), Some("info"));
    assert_eq!(equals.enum_option("level"), Some("info"));
}

#[test]
fn rejects_invalid_enum_value() {
    let command = Command::new("ritty")
        .enum_option(EnumOption::new("level", ["debug", "info", "warn", "error"]));

    let error = command.parse_from(["--level", "verbose"]).unwrap_err();

    assert_eq!(
        error.message(),
        "invalid value for option: --level: verbose (expected one of: debug, info, warn, error)"
    );
}

#[test]
fn enum_validation_is_case_sensitive() {
    let command = Command::new("ritty").enum_option(EnumOption::new("level", ["info"]));

    let error = command.parse_from(["--level", "INFO"]).unwrap_err();

    assert_eq!(
        error.message(),
        "invalid value for option: --level: INFO (expected one of: info)"
    );
}

#[test]
fn empty_allowed_value_list_accepts_anything() {
    let command = Command::new("ritty").enum_option(EnumOption::new("level", [] as [&str; 0]));

    let matches = command.parse_from(["--level", "anything"]).unwrap();

    assert_eq!(matches.enum_option("level"), Some("anything"));
}

#[test]
fn missing_optional_enum_option_remains_absent() {
    let command = Command::new("ritty").enum_option(EnumOption::new("level", ["debug", "info"]));

    let matches = command.parse_from([] as [&str; 0]).unwrap();

    assert_eq!(matches.enum_option("level"), None);
}

#[test]
fn missing_enum_option_uses_default() {
    let command = Command::new("ritty")
        .enum_option(EnumOption::new("level", ["debug", "info"]).default("info"));

    let matches = command.parse_from([] as [&str; 0]).unwrap();

    assert_eq!(matches.enum_option("level"), Some("info"));
}

#[test]
fn explicit_enum_value_overrides_default() {
    let command = Command::new("ritty")
        .enum_option(EnumOption::new("level", ["debug", "info"]).default("info"));

    let separate = command.parse_from(["--level", "debug"]).unwrap();

    assert_eq!(separate.enum_option("level"), Some("debug"));
}

#[test]
fn explicit_equals_enum_value_overrides_default() {
    let command = Command::new("ritty")
        .enum_option(EnumOption::new("level", ["debug", "info"]).default("info"));

    let matches = command.parse_from(["--level=debug"]).unwrap();

    assert_eq!(matches.enum_option("level"), Some("debug"));
}

#[test]
fn invalid_effective_enum_default_errors() {
    let command = Command::new("ritty")
        .enum_option(EnumOption::new("level", ["debug", "info"]).default("verbose"));

    let error = command.parse_from([] as [&str; 0]).unwrap_err();

    assert_eq!(
        error.message(),
        "invalid value for option: --level: verbose (expected one of: debug, info)"
    );
}

#[test]
fn valid_explicit_enum_value_beats_invalid_unused_default() {
    let command = Command::new("ritty")
        .enum_option(EnumOption::new("level", ["debug", "info"]).default("verbose"));

    let matches = command.parse_from(["--level", "info"]).unwrap();

    assert_eq!(matches.enum_option("level"), Some("info"));
}

#[test]
fn rejects_missing_required_enum_option() {
    let command =
        Command::new("ritty").enum_option(EnumOption::new("level", ["debug", "info"]).required());

    let error = command.parse_from([] as [&str; 0]).unwrap_err();

    assert_eq!(error.message(), "missing required option: --level");
}

#[test]
fn required_enum_option_satisfied_by_separate_explicit() {
    let command =
        Command::new("ritty").enum_option(EnumOption::new("level", ["debug", "info"]).required());

    let matches = command.parse_from(["--level", "info"]).unwrap();

    assert_eq!(matches.enum_option("level"), Some("info"));
}

#[test]
fn required_enum_option_satisfied_by_equals_explicit() {
    let command =
        Command::new("ritty").enum_option(EnumOption::new("level", ["debug", "info"]).required());

    let matches = command.parse_from(["--level=info"]).unwrap();

    assert_eq!(matches.enum_option("level"), Some("info"));
}

#[test]
fn required_enum_option_satisfied_by_valid_default() {
    let command = Command::new("ritty").enum_option(
        EnumOption::new("level", ["debug", "info"])
            .required()
            .default("info"),
    );

    let matches = command.parse_from([] as [&str; 0]).unwrap();

    assert_eq!(matches.enum_option("level"), Some("info"));
}

#[test]
fn required_enum_option_with_invalid_default_errors() {
    let command = Command::new("ritty").enum_option(
        EnumOption::new("level", ["debug", "info"])
            .required()
            .default("verbose"),
    );

    let error = command.parse_from([] as [&str; 0]).unwrap_err();

    assert_eq!(
        error.message(),
        "invalid value for option: --level: verbose (expected one of: debug, info)"
    );
}

#[test]
fn parses_short_enum_option_alias_separate_and_equals() {
    let command =
        Command::new("ritty").enum_option(EnumOption::new("level", ["debug", "info"]).alias("l"));

    let separate = command.parse_from(["-l", "info"]).unwrap();
    let equals = command.parse_from(["-l=info"]).unwrap();

    assert_eq!(separate.enum_option("level"), Some("info"));
    assert_eq!(equals.enum_option("level"), Some("info"));
}

#[test]
fn unicode_scalar_enum_option_alias_works_separate_and_equals() {
    let command =
        Command::new("ritty").enum_option(EnumOption::new("level", ["debug", "info"]).alias("é"));

    let separate = command.parse_from(["-é", "info"]).unwrap();
    let equals = command.parse_from(["-é=info"]).unwrap();

    assert_eq!(separate.enum_option("level"), Some("info"));
    assert_eq!(equals.enum_option("level"), Some("info"));
}

#[test]
fn unicode_scalar_enum_option_alias_still_validates_value() {
    let command =
        Command::new("ritty").enum_option(EnumOption::new("level", ["debug", "info"]).alias("é"));

    let error = command.parse_from(["-é", "verbose"]).unwrap_err();

    assert_eq!(
        error.message(),
        "invalid value for option: --level: verbose (expected one of: debug, info)"
    );
}

#[test]
fn parses_long_enum_option_alias_separate_and_equals() {
    let command = Command::new("ritty")
        .enum_option(EnumOption::new("level", ["debug", "info"]).alias("log-level"));

    let separate = command.parse_from(["--log-level", "info"]).unwrap();
    let equals = command.parse_from(["--log-level=info"]).unwrap();

    assert_eq!(separate.enum_option("level"), Some("info"));
    assert_eq!(equals.enum_option("level"), Some("info"));
}

#[test]
fn all_enum_alias_spellings_resolve_to_canonical_name() {
    let command = Command::new("ritty").enum_option(
        EnumOption::new("level", ["debug", "info"])
            .alias("l")
            .alias("log-level"),
    );

    for args in [
        &["--level", "info"][..],
        &["-l", "info"][..],
        &["-l=info"][..],
        &["--log-level", "info"][..],
        &["--log-level=info"][..],
    ] {
        let matches = command.parse_from(args.to_vec()).unwrap();
        assert_eq!(matches.enum_option("level"), Some("info"));
    }
}

#[test]
fn enum_alias_value_overrides_default() {
    let command = Command::new("ritty").enum_option(
        EnumOption::new("level", ["debug", "info"])
            .alias("l")
            .default("info"),
    );

    let matches = command.parse_from(["-l", "debug"]).unwrap();

    assert_eq!(matches.enum_option("level"), Some("debug"));
}

#[test]
fn enum_option_consumes_hyphen_prefixed_allowed_value() {
    let command = Command::new("ritty").enum_option(EnumOption::new("mode", ["-fast", "--safe"]));

    let fast = command.parse_from(["--mode", "-fast"]).unwrap();
    let safe = command.parse_from(["--mode", "--safe"]).unwrap();

    assert_eq!(fast.enum_option("mode"), Some("-fast"));
    assert_eq!(safe.enum_option("mode"), Some("--safe"));
}

#[test]
fn enum_value_matching_subcommand_is_not_mistaken_for_subcommand() {
    let command = Command::new("ritty")
        .enum_option(EnumOption::new("target", ["build", "test"]))
        .command(Command::new("build"));

    let matches = command.parse_from(["--target", "build"]).unwrap();

    assert_eq!(matches.enum_option("target"), Some("build"));
    assert_eq!(matches.subcommand(), None);
}

#[test]
fn subcommand_resolves_after_consumed_enum_value() {
    let command = Command::new("ritty")
        .enum_option(EnumOption::new("target", ["build", "test"]))
        .command(Command::new("build"));

    let matches = command.parse_from(["--target", "test", "build"]).unwrap();

    assert_eq!(matches.enum_option("target"), Some("test"));
    assert_eq!(matches.subcommand(), Some("build"));
}

#[test]
fn enum_option_value_does_not_advance_positional_cursor() {
    let command = Command::new("ritty")
        .enum_option(EnumOption::new("level", ["debug", "info"]))
        .arg(Arg::new("target"));

    let matches = command.parse_from(["--level", "info", "world"]).unwrap();

    assert_eq!(matches.enum_option("level"), Some("info"));
    assert_eq!(matches.argument("target"), Some("world"));
}

#[test]
fn canonical_enum_option_repeated_uses_first_occurrence() {
    let command = Command::new("ritty").enum_option(EnumOption::new("level", ["debug", "info"]));

    let matches = command
        .parse_from(["--level", "info", "--level", "debug"])
        .unwrap();

    assert_eq!(matches.enum_option("level"), Some("info"));
}

#[test]
fn string_option_and_enum_option_long_collision_errors() {
    let command = Command::new("ritty")
        .option(StringOption::new("mode"))
        .enum_option(EnumOption::new("mode", ["fast", "safe"]));

    let error = command.parse_from(["--mode"]).unwrap_err();

    assert_eq!(error.message(), "ambiguous option: --mode");
}

#[test]
fn flag_and_enum_option_long_collision_errors() {
    let command = Command::new("ritty")
        .flag(Flag::new("mode"))
        .enum_option(EnumOption::new("mode", ["fast", "safe"]));

    let bare = command.parse_from(["--mode"]).unwrap_err();
    let equals = command.parse_from(["--mode=fast"]).unwrap_err();

    assert_eq!(bare.message(), "ambiguous option: --mode");
    assert_eq!(equals.message(), "ambiguous option: --mode");
}

#[test]
fn boolean_negation_and_enum_option_no_prefix_collision_errors() {
    let command = Command::new("ritty")
        .flag(Flag::new("cache"))
        .enum_option(EnumOption::new("no-cache", ["on", "off"]));

    let error = command.parse_from(["--no-cache"]).unwrap_err();

    assert_eq!(error.message(), "ambiguous option: --no-cache");
}

#[test]
fn string_option_and_enum_option_short_alias_collision_errors() {
    let command = Command::new("ritty")
        .option(StringOption::new("output").alias("o"))
        .enum_option(EnumOption::new("format", ["json", "text"]).alias("o"));

    let error = command.parse_from(["-o", "x"]).unwrap_err();

    assert_eq!(error.message(), "ambiguous option: -o");
}

#[test]
fn boolean_and_enum_option_short_collision_errors() {
    let command = Command::new("ritty")
        .flag(Flag::new("verbose").short('v'))
        .enum_option(EnumOption::new("value", ["a", "b"]).alias("v"));

    let error = command.parse_from(["-v"]).unwrap_err();

    assert_eq!(error.message(), "ambiguous option: -v");
}

#[test]
fn multiple_enum_options_sharing_alias_collision_errors() {
    let command = Command::new("ritty")
        .enum_option(EnumOption::new("first", ["a"]).alias("x"))
        .enum_option(EnumOption::new("second", ["a"]).alias("x"));

    let error = command.parse_from(["-x", "a"]).unwrap_err();

    assert_eq!(error.message(), "ambiguous option: -x");
}

#[test]
fn enum_option_equals_value_preserves_extra_equals() {
    let command = Command::new("ritty").enum_option(EnumOption::new("mode", ["a=b"]).alias("m"));

    let long = command.parse_from(["--mode=a=b"]).unwrap();
    let short = command.parse_from(["-m=a=b"]).unwrap();

    assert_eq!(long.enum_option("mode"), Some("a=b"));
    assert_eq!(short.enum_option("mode"), Some("a=b"));
}

#[test]
fn enum_option_explicit_empty_value_is_validated_not_treated_as_absent() {
    let allowed = Command::new("ritty").enum_option(EnumOption::new("mode", ["", "a"]));
    let matches = allowed.parse_from(["--mode="]).unwrap();
    assert_eq!(matches.enum_option("mode"), Some(""));

    let disallowed = Command::new("ritty").enum_option(EnumOption::new("mode", ["a"]));
    let error = disallowed.parse_from(["--mode="]).unwrap_err();
    assert_eq!(
        error.message(),
        "invalid value for option: --mode:  (expected one of: a)"
    );
}

// --- Subcommand aliases and recursive parsing ---

#[test]
fn command_aliases_default_to_empty() {
    let command = Command::new("build");

    assert!(command.aliases().is_empty());
}

#[test]
fn command_retains_aliases_in_order() {
    let command = Command::new("install").alias("i").alias("add");

    assert_eq!(command.aliases(), ["i", "add"]);
}

#[test]
fn subcommand_alias_canonicalizes_to_name() {
    let command = Command::new("root").command(Command::new("install").alias("i").alias("add"));

    for token in ["install", "i", "add"] {
        let matches = command.parse_from([token]).unwrap();
        assert_eq!(matches.subcommand(), Some("install"));
    }
}

#[test]
fn duplicate_alias_on_one_command_does_not_self_collide() {
    let command = Command::new("root").command(Command::new("build").alias("b").alias("b"));

    let matches = command.parse_from(["b"]).unwrap();

    assert_eq!(matches.subcommand(), Some("build"));
}

#[test]
fn two_child_aliases_colliding_errors() {
    let command = Command::new("root")
        .command(Command::new("install").alias("x"))
        .command(Command::new("inspect").alias("x"));

    let error = command.parse_from(["x"]).unwrap_err();

    assert_eq!(error.message(), "ambiguous command: x");
}

#[test]
fn canonical_name_vs_sibling_alias_collision_errors() {
    let command = Command::new("root")
        .command(Command::new("build"))
        .command(Command::new("deploy").alias("build"));

    let error = command.parse_from(["build"]).unwrap_err();

    assert_eq!(error.message(), "ambiguous command: build");
}

#[test]
fn parent_string_option_before_subcommand() {
    let command = Command::new("root")
        .option(StringOption::new("profile"))
        .command(Command::new("build").option(StringOption::new("target")));

    let matches = command
        .parse_from(["--profile", "release", "build", "--target", "wasm"])
        .unwrap();

    assert_eq!(matches.option("profile"), Some("release"));
    assert_eq!(matches.subcommand(), Some("build"));
    assert_eq!(
        matches.subcommand_matches().unwrap().option("target"),
        Some("wasm")
    );
}

#[test]
fn parent_string_short_alias_before_subcommand() {
    let command = Command::new("root")
        .option(StringOption::new("profile").alias("p"))
        .command(Command::new("build"));

    let matches = command.parse_from(["-p", "release", "build"]).unwrap();

    assert_eq!(matches.option("profile"), Some("release"));
    assert_eq!(matches.subcommand(), Some("build"));
}

#[test]
fn parent_enum_value_matching_subcommand_does_not_select_it() {
    let command = Command::new("root")
        .enum_option(EnumOption::new("mode", ["build", "run"]))
        .command(Command::new("deploy"));

    let matches = command.parse_from(["--mode", "build", "deploy"]).unwrap();

    assert_eq!(matches.enum_option("mode"), Some("build"));
    assert_eq!(matches.subcommand(), Some("deploy"));
}

#[test]
fn parent_boolean_positive_before_subcommand() {
    let command = Command::new("root")
        .flag(Flag::new("verbose"))
        .command(Command::new("build"));

    let matches = command.parse_from(["--verbose", "build"]).unwrap();

    assert!(matches.flag("verbose"));
    assert_eq!(matches.subcommand(), Some("build"));
}

#[test]
fn parent_boolean_negation_before_subcommand() {
    let command = Command::new("root")
        .flag(Flag::new("verbose"))
        .command(Command::new("build"));

    let matches = command.parse_from(["--no-verbose", "build"]).unwrap();

    assert!(!matches.flag("verbose"));
    assert_eq!(matches.subcommand(), Some("build"));
}

#[test]
fn child_string_option_after_subcommand() {
    let command =
        Command::new("root").command(Command::new("build").option(StringOption::new("target")));

    let matches = command.parse_from(["build", "--target", "wasm"]).unwrap();

    assert_eq!(
        matches.subcommand_matches().unwrap().option("target"),
        Some("wasm")
    );
}

#[test]
fn child_enum_option_after_subcommand() {
    let command = Command::new("root")
        .command(Command::new("build").enum_option(EnumOption::new("mode", ["debug", "release"])));

    let matches = command.parse_from(["build", "--mode", "release"]).unwrap();

    assert_eq!(
        matches.subcommand_matches().unwrap().enum_option("mode"),
        Some("release")
    );
}

#[test]
fn child_boolean_option_after_subcommand() {
    let command = Command::new("root").command(Command::new("build").flag(Flag::new("verbose")));

    let matches = command.parse_from(["build", "--verbose"]).unwrap();

    assert!(matches.subcommand_matches().unwrap().flag("verbose"));
}

#[test]
fn parent_only_option_after_child_selection_errors_against_child() {
    let command = Command::new("root")
        .option(StringOption::new("profile"))
        .command(Command::new("build"));

    let error = command
        .parse_from(["build", "--profile", "release"])
        .unwrap_err();

    assert_eq!(error.message(), "unknown flag: --profile");
}

#[test]
fn string_option_value_matching_command_is_not_command() {
    let command = Command::new("root")
        .option(StringOption::new("target"))
        .command(Command::new("build"));

    let matches = command.parse_from(["--target", "build"]).unwrap();

    assert_eq!(matches.option("target"), Some("build"));
    assert_eq!(matches.subcommand(), None);
}

#[test]
fn string_option_alias_value_matching_command_is_not_command() {
    let command = Command::new("root")
        .option(StringOption::new("target").alias("t"))
        .command(Command::new("build"));

    let matches = command.parse_from(["-t", "build"]).unwrap();

    assert_eq!(matches.option("target"), Some("build"));
    assert_eq!(matches.subcommand(), None);
}

#[test]
fn enum_option_value_matching_command_is_not_command() {
    let command = Command::new("root")
        .enum_option(EnumOption::new("mode", ["build"]))
        .command(Command::new("build"));

    let matches = command.parse_from(["--mode", "build"]).unwrap();

    assert_eq!(matches.enum_option("mode"), Some("build"));
    assert_eq!(matches.subcommand(), None);
}

#[test]
fn enum_option_alias_value_matching_command_is_not_command() {
    let command = Command::new("root")
        .enum_option(EnumOption::new("mode", ["build"]).alias("m"))
        .command(Command::new("build"));

    let matches = command.parse_from(["-m", "build"]).unwrap();

    assert_eq!(matches.enum_option("mode"), Some("build"));
    assert_eq!(matches.subcommand(), None);
}

#[test]
fn equals_string_value_matching_command_is_not_command() {
    let command = Command::new("root")
        .option(StringOption::new("target"))
        .command(Command::new("build"));

    let matches = command.parse_from(["--target=build"]).unwrap();

    assert_eq!(matches.option("target"), Some("build"));
    assert_eq!(matches.subcommand(), None);
}

#[test]
fn equals_enum_value_matching_command_is_not_command() {
    let command = Command::new("root")
        .enum_option(EnumOption::new("mode", ["build"]))
        .command(Command::new("build"));

    let matches = command.parse_from(["--mode=build"]).unwrap();

    assert_eq!(matches.enum_option("mode"), Some("build"));
    assert_eq!(matches.subcommand(), None);
}

#[test]
fn two_level_nested_subcommand_parsing() {
    let command = Command::new("root").command(Command::new("remote").command(Command::new("add")));

    let matches = command.parse_from(["remote", "add"]).unwrap();

    assert_eq!(matches.subcommand(), Some("remote"));
    assert_eq!(
        matches.subcommand_matches().unwrap().subcommand(),
        Some("add")
    );
}

#[test]
fn three_level_nested_subcommand_parsing() {
    let command = Command::new("root").command(
        Command::new("remote").command(Command::new("add").command(Command::new("verify"))),
    );

    let matches = command.parse_from(["remote", "add", "verify"]).unwrap();

    let remote = matches.subcommand_matches().unwrap();
    let add = remote.subcommand_matches().unwrap();

    assert_eq!(matches.subcommand(), Some("remote"));
    assert_eq!(remote.subcommand(), Some("add"));
    assert_eq!(add.subcommand(), Some("verify"));
}

#[test]
fn nested_subcommand_alias_canonicalizes() {
    let command = Command::new("root").command(
        Command::new("remote")
            .alias("r")
            .command(Command::new("add").alias("a")),
    );

    let matches = command.parse_from(["r", "a"]).unwrap();

    assert_eq!(matches.subcommand(), Some("remote"));
    assert_eq!(
        matches.subcommand_matches().unwrap().subcommand(),
        Some("add")
    );
}

#[test]
fn nested_child_option_parsing() {
    let command = Command::new("root").command(
        Command::new("remote").command(Command::new("add").option(StringOption::new("name"))),
    );

    let matches = command
        .parse_from(["remote", "add", "--name", "origin"])
        .unwrap();

    let add = matches
        .subcommand_matches()
        .unwrap()
        .subcommand_matches()
        .unwrap();
    assert_eq!(add.option("name"), Some("origin"));
}

#[test]
fn nested_child_required_validation() {
    let command = Command::new("root").command(
        Command::new("remote")
            .command(Command::new("add").option(StringOption::new("name").required())),
    );

    let error = command.parse_from(["remote", "add"]).unwrap_err();

    assert_eq!(error.message(), "missing required option: --name");
}

#[test]
fn parent_required_validation_still_occurs_with_subcommand() {
    let command = Command::new("root")
        .option(StringOption::new("profile").required())
        .command(Command::new("build"));

    let error = command.parse_from(["build"]).unwrap_err();

    assert_eq!(error.message(), "missing required option: --profile");
}

#[test]
fn parent_defaults_survive_child_selection() {
    let command = Command::new("root")
        .option(StringOption::new("profile").default("default-profile"))
        .command(
            Command::new("build").option(StringOption::new("target").default("default-target")),
        );

    let matches = command.parse_from(["build"]).unwrap();

    assert_eq!(matches.option("profile"), Some("default-profile"));
    assert_eq!(matches.subcommand(), Some("build"));
    assert_eq!(
        matches.subcommand_matches().unwrap().option("target"),
        Some("default-target")
    );
}

#[test]
fn child_defaults_stored_in_child_matches_only() {
    let command = Command::new("root")
        .command(Command::new("build").option(StringOption::new("target").default("wasm")));

    let matches = command.parse_from(["build"]).unwrap();

    assert_eq!(matches.option("target"), None);
    assert_eq!(
        matches.subcommand_matches().unwrap().option("target"),
        Some("wasm")
    );
}

#[test]
fn subcommand_selected_over_positional_when_both_could_match() {
    let command = Command::new("root")
        .arg(Arg::new("value"))
        .command(Command::new("build"));

    let matches = command.parse_from(["build"]).unwrap();

    assert_eq!(matches.subcommand(), Some("build"));
    assert_eq!(matches.argument("value"), None);
}

#[test]
fn non_command_bare_token_fills_positional_when_available() {
    let command = Command::new("root")
        .arg(Arg::new("value"))
        .command(Command::new("build"));

    let matches = command.parse_from(["something-else"]).unwrap();

    assert_eq!(matches.argument("value"), Some("something-else"));
    assert_eq!(matches.subcommand(), None);
}

#[test]
fn unknown_command_when_no_positional_can_accept_token() {
    let command = Command::new("root").command(Command::new("build"));

    let error = command.parse_from(["foo"]).unwrap_err();

    assert_eq!(error.message(), "unknown command: foo");
}

#[test]
fn selected_child_prevents_parent_selecting_later_sibling() {
    let command = Command::new("root")
        .command(Command::new("build").arg(Arg::new("rest")))
        .command(Command::new("test"));

    let matches = command.parse_from(["build", "test"]).unwrap();

    assert_eq!(matches.subcommand(), Some("build"));
    assert_eq!(
        matches.subcommand_matches().unwrap().argument("rest"),
        Some("test")
    );
}

#[test]
fn subcommand_matches_returns_correct_child() {
    let command = Command::new("root")
        .command(Command::new("build"))
        .command(Command::new("test"));

    let matches = command.parse_from(["test"]).unwrap();

    assert!(matches.subcommand_matches().is_some());
    assert_eq!(matches.subcommand_matches().unwrap().subcommand(), None);
}

#[test]
fn no_subcommand_selected_returns_normal_matches() {
    let command = Command::new("root").command(Command::new("build"));

    let matches = command.parse_from([] as [&str; 0]).unwrap();

    assert_eq!(matches.subcommand(), None);
    assert!(matches.subcommand_matches().is_none());
}

#[test]
fn new_command_has_no_default_subcommand() {
    let command = Command::new("root");

    assert_eq!(command.get_default_subcommand(), None);
}

#[test]
fn default_subcommand_builder_stores_exact_spelling() {
    let command = Command::new("root").default_subcommand("b");

    assert_eq!(command.get_default_subcommand(), Some("b"));
    assert_eq!(command.aliases(), &[] as &[String]);
}

#[test]
fn empty_argv_selects_default_subcommand() {
    let command = Command::new("root")
        .default_subcommand("build")
        .command(Command::new("build"));

    let matches = command.parse_from([] as [&str; 0]).unwrap();

    assert_eq!(matches.subcommand(), Some("build"));
    assert!(matches.subcommand_matches().is_some());
}

#[test]
fn default_subcommand_alias_resolves_to_canonical_name() {
    let command = Command::new("root")
        .default_subcommand("b")
        .command(Command::new("build").alias("b"));

    let matches = command.parse_from([] as [&str; 0]).unwrap();

    assert_eq!(matches.subcommand(), Some("build"));
}

#[test]
fn explicit_child_overrides_default_subcommand() {
    let command = Command::new("root")
        .default_subcommand("build")
        .command(Command::new("build"))
        .command(Command::new("test"));

    let matches = command.parse_from(["test"]).unwrap();

    assert_eq!(matches.subcommand(), Some("test"));
}

#[test]
fn explicit_child_alias_overrides_default_subcommand() {
    let command = Command::new("root")
        .default_subcommand("build")
        .command(Command::new("build"))
        .command(Command::new("test").alias("t"));

    let matches = command.parse_from(["t"]).unwrap();

    assert_eq!(matches.subcommand(), Some("test"));
}

#[test]
fn parent_boolean_option_consumed_before_default_subcommand() {
    let command = Command::new("root")
        .flag(Flag::new("verbose"))
        .default_subcommand("build")
        .command(Command::new("build"));

    let matches = command.parse_from(["--verbose"]).unwrap();

    assert!(matches.flag("verbose"));
    assert_eq!(matches.subcommand(), Some("build"));
}

#[test]
fn parent_value_option_consumed_before_default_subcommand() {
    let command = Command::new("root")
        .option(StringOption::new("profile"))
        .default_subcommand("build")
        .command(Command::new("build"));

    let matches = command.parse_from(["--profile", "release"]).unwrap();

    assert_eq!(matches.option("profile"), Some("release"));
    assert_eq!(matches.subcommand(), Some("build"));
}

#[test]
fn parent_positional_retained_before_default_subcommand() {
    let command = Command::new("root")
        .arg(Arg::new("workspace"))
        .default_subcommand("build")
        .command(Command::new("build"));

    let matches = command.parse_from(["project"]).unwrap();

    assert_eq!(matches.argument("workspace"), Some("project"));
    assert_eq!(matches.subcommand(), Some("build"));
}

#[test]
fn parent_defaults_applied_before_default_subcommand_selection() {
    let command = Command::new("root")
        .option(StringOption::new("profile").default("release"))
        .default_subcommand("build")
        .command(Command::new("build"));

    let matches = command.parse_from([] as [&str; 0]).unwrap();

    assert_eq!(matches.option("profile"), Some("release"));
    assert_eq!(matches.subcommand(), Some("build"));
}

#[test]
fn parent_required_validation_precedes_default_subcommand_selection() {
    let command = Command::new("root")
        .option(StringOption::new("profile").required())
        .default_subcommand("build")
        .command(Command::new("build"));

    let error = command.parse_from([] as [&str; 0]).unwrap_err();

    assert_eq!(error.message(), "missing required option: --profile");
}

#[test]
fn default_subcommand_child_receives_own_defaults() {
    let command = Command::new("root")
        .default_subcommand("build")
        .command(Command::new("build").option(StringOption::new("target").default("native")));

    let matches = command.parse_from([] as [&str; 0]).unwrap();

    assert_eq!(
        matches.subcommand_matches().unwrap().option("target"),
        Some("native")
    );
}

#[test]
fn default_subcommand_child_required_validation_propagates() {
    let command = Command::new("root")
        .default_subcommand("build")
        .command(Command::new("build").option(StringOption::new("target").required()));

    let error = command.parse_from([] as [&str; 0]).unwrap_err();

    assert_eq!(error.message(), "missing required option: --target");
}

#[test]
fn recursive_default_subcommand_chain_resolves_at_every_level() {
    let command = Command::new("root").default_subcommand("remote").command(
        Command::new("remote")
            .default_subcommand("status")
            .command(Command::new("status")),
    );

    let matches = command.parse_from([] as [&str; 0]).unwrap();

    assert_eq!(matches.subcommand(), Some("remote"));
    let remote_matches = matches.subcommand_matches().unwrap();
    assert_eq!(remote_matches.subcommand(), Some("status"));
    assert!(remote_matches.subcommand_matches().is_some());
}

#[test]
fn missing_configured_default_subcommand_errors() {
    let command = Command::new("root").default_subcommand("build");

    let error = command.parse_from([] as [&str; 0]).unwrap_err();

    assert_eq!(error.message(), "default subcommand not found: build");
}

#[test]
fn ambiguous_configured_default_subcommand_errors() {
    let command = Command::new("root")
        .default_subcommand("x")
        .command(Command::new("first").alias("x"))
        .command(Command::new("second").alias("x"));

    let error = command.parse_from([] as [&str; 0]).unwrap_err();

    assert_eq!(error.message(), "ambiguous command: x");
}

#[test]
fn default_canonical_vs_alias_collision_errors() {
    let command = Command::new("root")
        .default_subcommand("build")
        .command(Command::new("build"))
        .command(Command::new("deploy").alias("build"));

    let error = command.parse_from([] as [&str; 0]).unwrap_err();

    assert_eq!(error.message(), "ambiguous command: build");
}

#[test]
fn duplicate_alias_within_one_child_does_not_self_collide_as_default() {
    let command = Command::new("root")
        .default_subcommand("b")
        .command(Command::new("build").alias("b").alias("b"));

    let matches = command.parse_from([] as [&str; 0]).unwrap();

    assert_eq!(matches.subcommand(), Some("build"));
}

#[test]
fn option_owned_by_default_subcommand_is_forwarded_to_it() {
    let command = Command::new("root")
        .default_subcommand("build")
        .command(Command::new("build").option(StringOption::new("target")));

    let matches = command.parse_from(["--target", "wasm"]).unwrap();

    assert_eq!(matches.subcommand(), Some("build"));
    assert_eq!(
        matches.subcommand_matches().unwrap().option("target"),
        Some("wasm")
    );
}

#[test]
fn explicit_child_argv_behavior_unchanged_with_default_configured() {
    let command = Command::new("root")
        .default_subcommand("build")
        .command(Command::new("build").option(StringOption::new("target")))
        .command(Command::new("test"));

    let matches = command.parse_from(["build", "--target", "wasm"]).unwrap();

    assert_eq!(matches.subcommand(), Some("build"));
    assert_eq!(
        matches.subcommand_matches().unwrap().option("target"),
        Some("wasm")
    );
}

// -- Default-subcommand input: options, flags, positionals --

#[test]
fn default_subcommand_receives_canonical_long_string_option() {
    let command = Command::new("root")
        .default_subcommand("run")
        .command(Command::new("run").option(StringOption::new("format")));

    let matches = command.parse_from(["--format", "json"]).unwrap();

    assert_eq!(matches.subcommand(), Some("run"));
    assert_eq!(
        matches.subcommand_matches().unwrap().option("format"),
        Some("json")
    );
}

#[test]
fn default_subcommand_receives_short_string_option_alias() {
    let command = Command::new("root")
        .default_subcommand("run")
        .command(Command::new("run").option(StringOption::new("format").alias("f")));

    let matches = command.parse_from(["-f", "json"]).unwrap();

    assert_eq!(
        matches.subcommand_matches().unwrap().option("format"),
        Some("json")
    );
}

#[test]
fn default_subcommand_receives_unicode_scalar_short_alias() {
    let command = Command::new("root")
        .default_subcommand("run")
        .command(Command::new("run").option(StringOption::new("format").alias("é")));

    let separate = command.parse_from(["-é", "json"]).unwrap();
    let equals = command.parse_from(["-é=json"]).unwrap();

    assert_eq!(
        separate.subcommand_matches().unwrap().option("format"),
        Some("json")
    );
    assert_eq!(
        equals.subcommand_matches().unwrap().option("format"),
        Some("json")
    );
}

#[test]
fn default_subcommand_receives_name_equals_value() {
    let command = Command::new("root")
        .default_subcommand("run")
        .command(Command::new("run").option(StringOption::new("format")));

    let matches = command.parse_from(["--format=json"]).unwrap();

    assert_eq!(
        matches.subcommand_matches().unwrap().option("format"),
        Some("json")
    );
}

#[test]
fn default_subcommand_receives_enum_option() {
    let command = Command::new("root")
        .default_subcommand("run")
        .command(Command::new("run").enum_option(EnumOption::new("level", ["debug", "info"])));

    let matches = command.parse_from(["--level", "debug"]).unwrap();

    assert_eq!(
        matches.subcommand_matches().unwrap().enum_option("level"),
        Some("debug")
    );
}

#[test]
fn default_subcommand_receives_boolean_flag() {
    let command = Command::new("root")
        .default_subcommand("run")
        .command(Command::new("run").flag(Flag::new("verbose")));

    let matches = command.parse_from(["--verbose"]).unwrap();

    assert!(matches.subcommand_matches().unwrap().flag("verbose"));
}

#[test]
fn default_subcommand_receives_positional() {
    let command = Command::new("root")
        .default_subcommand("run")
        .command(Command::new("run").arg(Arg::new("file")));

    let matches = command.parse_from(["main.rs"]).unwrap();

    assert_eq!(matches.subcommand(), Some("run"));
    assert_eq!(
        matches.subcommand_matches().unwrap().argument("file"),
        Some("main.rs")
    );
}

#[test]
fn default_subcommand_required_positional_satisfied_from_argv() {
    let command = Command::new("root")
        .default_subcommand("run")
        .command(Command::new("run").arg(Arg::new("file").required()));

    let matches = command.parse_from(["main.rs"]).unwrap();

    assert_eq!(
        matches.subcommand_matches().unwrap().argument("file"),
        Some("main.rs")
    );
}

#[test]
fn default_subcommand_required_option_satisfied_from_argv() {
    let command = Command::new("root")
        .default_subcommand("run")
        .command(Command::new("run").option(StringOption::new("format").required()));

    let matches = command.parse_from(["--format", "json"]).unwrap();

    assert_eq!(
        matches.subcommand_matches().unwrap().option("format"),
        Some("json")
    );
}

#[test]
fn default_subcommand_defaults_still_apply_alongside_argv() {
    let command = Command::new("root").default_subcommand("run").command(
        Command::new("run")
            .option(StringOption::new("format"))
            .option(StringOption::new("target").default("native")),
    );

    let matches = command.parse_from(["--format", "json"]).unwrap();

    let child = matches.subcommand_matches().unwrap();
    assert_eq!(child.option("format"), Some("json"));
    assert_eq!(child.option("target"), Some("native"));
}

#[test]
fn parent_option_and_default_subcommand_option_coexist() {
    let command = Command::new("root")
        .option(StringOption::new("profile"))
        .default_subcommand("run")
        .command(Command::new("run").option(StringOption::new("format")));

    let matches = command
        .parse_from(["--profile", "release", "--format", "json"])
        .unwrap();

    assert_eq!(matches.option("profile"), Some("release"));
    assert_eq!(
        matches.subcommand_matches().unwrap().option("format"),
        Some("json")
    );
}

#[test]
fn parent_flag_and_default_subcommand_option_coexist() {
    let command = Command::new("root")
        .flag(Flag::new("quiet"))
        .default_subcommand("run")
        .command(Command::new("run").option(StringOption::new("format")));

    let matches = command.parse_from(["--quiet", "--format", "json"]).unwrap();

    assert!(matches.flag("quiet"));
    assert_eq!(
        matches.subcommand_matches().unwrap().option("format"),
        Some("json")
    );
}

#[test]
fn option_owned_by_neither_parent_nor_default_subcommand_errors() {
    let command = Command::new("root")
        .default_subcommand("run")
        .command(Command::new("run").option(StringOption::new("format")));

    let error = command.parse_from(["--bogus", "x"]).unwrap_err();

    assert_eq!(error.message(), "unknown flag: --bogus");
}

#[test]
fn nested_default_subcommand_chain_forwards_argv_at_every_level() {
    let command = Command::new("root").default_subcommand("remote").command(
        Command::new("remote")
            .default_subcommand("status")
            .command(Command::new("status").option(StringOption::new("format"))),
    );

    let matches = command.parse_from(["--format", "json"]).unwrap();

    let remote_matches = matches.subcommand_matches().unwrap();
    let status_matches = remote_matches.subcommand_matches().unwrap();
    assert_eq!(status_matches.option("format"), Some("json"));
}

// -- `--` terminator --

#[test]
fn terminator_itself_is_not_stored_as_positional() {
    let command = Command::new("root").arg(Arg::new("name"));

    let matches = command.parse_from(["--", "value"]).unwrap();

    assert_eq!(matches.argument("name"), Some("value"));
}

#[test]
fn terminator_treats_long_flag_spelling_literally() {
    let command = Command::new("root")
        .flag(Flag::new("verbose"))
        .arg(Arg::new("value"));

    let matches = command.parse_from(["--", "--verbose"]).unwrap();

    assert!(!matches.flag("verbose"));
    assert_eq!(matches.argument("value"), Some("--verbose"));
}

#[test]
fn terminator_treats_short_flag_spelling_literally() {
    let command = Command::new("root")
        .flag(Flag::new("verbose").short('x'))
        .arg(Arg::new("value"));

    let matches = command.parse_from(["--", "-x"]).unwrap();

    assert!(!matches.flag("verbose"));
    assert_eq!(matches.argument("value"), Some("-x"));
}

#[test]
fn terminator_treats_name_equals_value_literally() {
    let command = Command::new("root")
        .option(StringOption::new("name"))
        .arg(Arg::new("value"));

    let matches = command.parse_from(["--", "--name=value"]).unwrap();

    assert_eq!(matches.option("name"), None);
    assert_eq!(matches.argument("value"), Some("--name=value"));
}

#[test]
fn terminator_does_not_negate_flag_via_no_prefix() {
    let command = Command::new("root")
        .flag(Flag::new("verbose").default(true))
        .arg(Arg::new("value"));

    let matches = command.parse_from(["--", "--no-verbose"]).unwrap();

    assert!(matches.flag("verbose"));
    assert_eq!(matches.argument("value"), Some("--no-verbose"));
}

#[test]
fn terminator_prevents_child_name_spelling_from_selecting_subcommand() {
    let command = Command::new("root")
        .arg(Arg::new("value"))
        .command(Command::new("build"));

    let matches = command.parse_from(["--", "build"]).unwrap();

    assert_eq!(matches.subcommand(), None);
    assert_eq!(matches.argument("value"), Some("build"));
}

#[test]
fn explicit_child_followed_by_its_own_terminator_works() {
    let command = Command::new("root").command(
        Command::new("build")
            .flag(Flag::new("release"))
            .arg(Arg::new("value")),
    );

    let matches = command.parse_from(["build", "--", "--release"]).unwrap();

    assert_eq!(matches.subcommand(), Some("build"));
    let child = matches.subcommand_matches().unwrap();
    assert!(!child.flag("release"));
    assert_eq!(child.argument("value"), Some("--release"));
}

#[test]
fn positionals_before_and_after_terminator_bind_in_declaration_order() {
    let command = Command::new("root")
        .arg(Arg::new("first"))
        .arg(Arg::new("second"));

    let matches = command.parse_from(["one", "--", "two"]).unwrap();

    assert_eq!(matches.argument("first"), Some("one"));
    assert_eq!(matches.argument("second"), Some("two"));
}

#[test]
fn excess_positional_after_terminator_errors() {
    let command = Command::new("root").arg(Arg::new("only"));

    let error = command.parse_from(["--", "one", "two"]).unwrap_err();

    assert_eq!(error.message(), "unexpected argument: two");
}

#[test]
fn terminator_forwards_literal_positional_to_default_subcommand() {
    let command = Command::new("root")
        .default_subcommand("run")
        .command(Command::new("run").arg(Arg::new("file")));

    let matches = command.parse_from(["--", "--verbose"]).unwrap();

    assert_eq!(matches.subcommand(), Some("run"));
    assert_eq!(
        matches.subcommand_matches().unwrap().argument("file"),
        Some("--verbose")
    );
}

#[test]
fn terminator_before_explicit_child_argv_is_forwarded_intact() {
    let command = Command::new("root").command(
        Command::new("build")
            .flag(Flag::new("release"))
            .arg(Arg::new("value")),
    );

    let matches = command.parse_from(["build", "--", "--release"]).unwrap();

    assert_eq!(matches.subcommand(), Some("build"));
    let child = matches.subcommand_matches().unwrap();
    assert_eq!(child.argument("value"), Some("--release"));
}

// -- Existing error parity --

#[test]
fn unexpected_positional_errors_when_no_subcommands_declared() {
    let command = Command::new("root").arg(Arg::new("only"));

    let error = command.parse_from(["one", "two"]).unwrap_err();

    assert_eq!(error.message(), "unexpected argument: two");
}

#[test]
fn excess_positional_beyond_default_subcommand_capacity_errors() {
    let command = Command::new("root")
        .default_subcommand("run")
        .command(Command::new("run").arg(Arg::new("file")));

    let error = command.parse_from(["a.rs", "b.rs"]).unwrap_err();

    assert_eq!(error.message(), "unexpected argument: b.rs");
}

#[test]
fn terminator_before_default_subcommand_selection_still_forwards_ambiguous_spelling() {
    // Root has no explicit child; "build" only exists as the default
    // child's own name. A leading `--` still must not turn "build" into
    // subcommand recognition — it becomes a literal positional handed to
    // the default child, exactly as when no terminator is present, since
    // the default child was always going to be selected regardless.
    let command = Command::new("root")
        .default_subcommand("run")
        .command(Command::new("run").arg(Arg::new("target")));

    let matches = command.parse_from(["--", "build"]).unwrap();

    assert_eq!(matches.subcommand(), Some("run"));
    assert_eq!(
        matches.subcommand_matches().unwrap().argument("target"),
        Some("build")
    );
}

// -- Hidden subcommands --

#[test]
fn new_command_is_visible_by_default() {
    let command = Command::new("build");

    assert!(!command.is_hidden());
}

#[test]
fn hidden_marks_command_hidden() {
    let command = Command::new("internal").hidden();

    assert!(command.is_hidden());
}

#[test]
fn hidden_command_remains_explicitly_parseable() {
    let root = Command::new("root")
        .command(Command::new("public"))
        .command(Command::new("internal").hidden());

    let matches = root.parse_from(["internal"]).unwrap();

    assert_eq!(matches.subcommand(), Some("internal"));
}

#[test]
fn hidden_command_alias_remains_parseable() {
    let root = Command::new("root").command(Command::new("internal").alias("i").hidden());

    let matches = root.parse_from(["i"]).unwrap();

    assert_eq!(matches.subcommand(), Some("internal"));
}

#[test]
fn hidden_default_subcommand_still_resolves() {
    let root = Command::new("root")
        .default_subcommand("internal")
        .command(Command::new("internal").hidden());

    let matches = root.parse_from(Vec::<&str>::new()).unwrap();

    assert_eq!(matches.subcommand(), Some("internal"));
}

// -- Usage: header --

#[test]
fn usage_basic_command_name() {
    let command = Command::new("ritty");

    assert_eq!(command.render_usage(), "USAGE ritty");
}

#[test]
fn usage_description() {
    let command = Command::new("ritty").description("Elegant CLI builder");

    assert_eq!(command.render_usage(), "Elegant CLI builder\n\nUSAGE ritty");
}

#[test]
fn usage_version() {
    let command = Command::new("ritty").version("1.0.0");

    assert_eq!(command.render_usage(), "ritty v1.0.0\n\nUSAGE ritty");
}

#[test]
fn usage_description_and_version() {
    let command = Command::new("ritty")
        .description("Elegant CLI builder")
        .version("1.0.0");

    assert_eq!(
        command.render_usage(),
        "Elegant CLI builder (ritty v1.0.0)\n\nUSAGE ritty"
    );
}

// -- Usage: positionals --

#[test]
fn usage_required_positional() {
    let command = Command::new("ritty").arg(Arg::new("target").required());

    assert_eq!(
        command.render_usage(),
        "USAGE ritty <TARGET>\n\nARGUMENTS\n\n  TARGET  (Required)"
    );
}

#[test]
fn usage_optional_positional() {
    let command = Command::new("ritty").arg(Arg::new("target"));

    assert_eq!(
        command.render_usage(),
        "USAGE ritty [TARGET]\n\nARGUMENTS\n\n  TARGET"
    );
}

#[test]
fn usage_positional_default() {
    let command = Command::new("ritty").arg(Arg::new("target").required().default("main"));

    assert_eq!(
        command.render_usage(),
        "USAGE ritty [TARGET]\n\nARGUMENTS\n\n  TARGET  (Default: main)"
    );
}

#[test]
fn usage_positional_description() {
    let command = Command::new("ritty").arg(Arg::new("target").description("Build target"));

    assert_eq!(
        command.render_usage(),
        "USAGE ritty [TARGET]\n\nARGUMENTS\n\n  TARGET  Build target"
    );
}

#[test]
fn usage_positional_value_hint() {
    let command = Command::new("ritty").arg(Arg::new("target").value_hint("dir"));

    assert_eq!(
        command.render_usage(),
        "USAGE ritty [TARGET]\n\nARGUMENTS\n\n  TARGET <dir>"
    );
}

// -- Usage: string options --

#[test]
fn usage_string_option() {
    let command = Command::new("ritty").option(StringOption::new("output"));

    assert_eq!(
        command.render_usage(),
        "USAGE ritty [OPTIONS]\n\nOPTIONS\n\n  --output=<output>"
    );
}

#[test]
fn usage_short_string_alias() {
    let command = Command::new("ritty").option(StringOption::new("output").alias("o"));

    assert_eq!(
        command.render_usage(),
        "USAGE ritty [OPTIONS]\n\nOPTIONS\n\n  -o, --output=<output>"
    );
}

#[test]
fn usage_unicode_scalar_string_alias_matches_parser_spelling() {
    let command = Command::new("ritty").option(StringOption::new("output").alias("é"));

    assert_eq!(
        command.render_usage(),
        "USAGE ritty [OPTIONS]\n\nOPTIONS\n\n  -é, --output=<output>"
    );
    assert_eq!(
        command.parse_from(["-é", "dist"]).unwrap().option("output"),
        Some("dist")
    );
}

#[test]
fn usage_long_string_alias() {
    let command = Command::new("ritty").option(StringOption::new("output").alias("destination"));

    assert_eq!(
        command.render_usage(),
        "USAGE ritty [OPTIONS]\n\nOPTIONS\n\n  --destination, --output=<output>"
    );
}

#[test]
fn usage_string_value_hint() {
    let command = Command::new("ritty").option(StringOption::new("output").value_hint("dir"));

    assert_eq!(
        command.render_usage(),
        "USAGE ritty [OPTIONS]\n\nOPTIONS\n\n  --output=<dir>"
    );
}

#[test]
fn usage_string_default() {
    let command = Command::new("ritty").option(StringOption::new("output").default("dist"));

    assert_eq!(
        command.render_usage(),
        "USAGE ritty [OPTIONS]\n\nOPTIONS\n\n  --output=<output>  (Default: dist)"
    );
}

#[test]
fn usage_required_string_option() {
    let command = Command::new("ritty").option(StringOption::new("output").required());

    assert_eq!(
        command.render_usage(),
        "USAGE ritty [OPTIONS] --output=<output>\n\nOPTIONS\n\n  --output=<output>  (Required)"
    );
}

// -- Usage: enum options --

#[test]
fn usage_enum_choices() {
    let command =
        Command::new("ritty").enum_option(EnumOption::new("level", ["debug", "info", "warn"]));

    assert_eq!(
        command.render_usage(),
        "USAGE ritty [OPTIONS]\n\nOPTIONS\n\n  --level=<debug|info|warn>"
    );
}

#[test]
fn usage_enum_aliases() {
    let command = Command::new("ritty").enum_option(
        EnumOption::new("level", ["debug", "info"])
            .alias("l")
            .alias("log-level"),
    );

    assert_eq!(
        command.render_usage(),
        "USAGE ritty [OPTIONS]\n\nOPTIONS\n\n  -l, --log-level, --level=<debug|info>"
    );
}

#[test]
fn usage_enum_default() {
    let command = Command::new("ritty")
        .enum_option(EnumOption::new("level", ["debug", "info"]).default("info"));

    assert_eq!(
        command.render_usage(),
        "USAGE ritty [OPTIONS]\n\nOPTIONS\n\n  --level=<debug|info>  (Default: info)"
    );
}

// -- Usage: boolean flags --

#[test]
fn usage_boolean_canonical_flag() {
    let command = Command::new("ritty").flag(Flag::new("verbose"));

    assert_eq!(
        command.render_usage(),
        "USAGE ritty [OPTIONS]\n\nOPTIONS\n\n  --verbose"
    );
}

#[test]
fn usage_dedicated_short_flag() {
    let command = Command::new("ritty").flag(Flag::new("verbose").short('v'));

    assert_eq!(
        command.render_usage(),
        "USAGE ritty [OPTIONS]\n\nOPTIONS\n\n  -v, --verbose"
    );
}

#[test]
fn usage_boolean_aliases() {
    let command = Command::new("ritty").flag(Flag::new("verbose").short('v').alias("chatty"));

    assert_eq!(
        command.render_usage(),
        "USAGE ritty [OPTIONS]\n\nOPTIONS\n\n  -v, --chatty, --verbose"
    );
}

#[test]
fn usage_boolean_default() {
    let command = Command::new("ritty").flag(Flag::new("verbose").default(false));

    assert_eq!(
        command.render_usage(),
        "USAGE ritty [OPTIONS]\n\nOPTIONS\n\n  --verbose  (Default: false)"
    );
}

#[test]
fn usage_boolean_required_marker() {
    let command = Command::new("ritty").flag(Flag::new("verbose").required());

    assert_eq!(
        command.render_usage(),
        "USAGE ritty [OPTIONS] --verbose\n\nOPTIONS\n\n  --verbose  (Required)"
    );
}

// -- Usage: negative booleans --

#[test]
fn usage_negative_boolean_from_default_true() {
    let command = Command::new("ritty").flag(Flag::new("color").default(true));

    assert_eq!(
        command.render_usage(),
        "USAGE ritty [OPTIONS]\n\nOPTIONS\n\n  --color     (Default: true)\n  --no-color"
    );
}

#[test]
fn usage_negative_boolean_from_negative_description() {
    let command =
        Command::new("ritty").flag(Flag::new("color").negative_description("Disable color"));

    assert_eq!(
        command.render_usage(),
        "USAGE ritty [OPTIONS]\n\nOPTIONS\n\n  --color\n  --no-color  Disable color"
    );
}

#[test]
fn usage_negative_boolean_from_default_true_and_negative_description() {
    let command = Command::new("ritty").flag(
        Flag::new("color")
            .default(true)
            .description("Enable color")
            .negative_description("Disable color"),
    );

    assert_eq!(
        command.render_usage(),
        "USAGE ritty [OPTIONS]\n\nOPTIONS\n\n  --color     Enable color (Default: true)\n  --no-color  Disable color"
    );
}

#[test]
fn usage_no_double_negative_when_canonical_already_negative() {
    let command = Command::new("ritty").flag(Flag::new("no-cache").default(true));

    assert_eq!(
        command.render_usage(),
        "USAGE ritty [OPTIONS]\n\nOPTIONS\n\n  --no-cache  (Default: true)"
    );
}

#[test]
fn usage_negative_boolean_does_not_advertise_unparseable_short_negation() {
    // `.short('v')` alone does not register "v" as a long alias, so
    // `--no-v` is not something the parser accepts; only `--no-verbose`
    // should be advertised.
    let command = Command::new("ritty").flag(Flag::new("verbose").short('v').default(true));

    assert_eq!(
        command.render_usage(),
        "USAGE ritty [OPTIONS]\n\nOPTIONS\n\n  -v, --verbose  (Default: true)\n  --no-verbose"
    );
}

// -- Usage: alias deduplication --

#[test]
fn usage_aliases_deduplicate_visually() {
    let command = Command::new("ritty").flag(Flag::new("verbose").short('v').alias("v"));

    assert_eq!(
        command.render_usage(),
        "USAGE ritty [OPTIONS]\n\nOPTIONS\n\n  -v, --verbose"
    );
}

// -- Usage: declaration order --

#[test]
fn usage_multiple_arguments_and_options_retain_declaration_order() {
    let command = Command::new("ritty")
        .arg(Arg::new("first"))
        .arg(Arg::new("second"))
        .flag(Flag::new("alpha"))
        .option(StringOption::new("beta"))
        .enum_option(EnumOption::new("gamma", ["x", "y"]));

    assert_eq!(
        command.render_usage(),
        "USAGE ritty [OPTIONS] [FIRST] [SECOND]\n\n\
             ARGUMENTS\n\n  FIRST\n  SECOND\n\n\
             OPTIONS\n\n  --alpha\n  --beta=<beta>\n  --gamma=<x|y>"
    );
}

// -- Usage: subcommands --

#[test]
fn usage_visible_subcommands() {
    let command = Command::new("ritty")
        .command(Command::new("build").description("Build the project"))
        .command(Command::new("test").description("Run tests"));

    assert_eq!(
        command.render_usage(),
        "USAGE ritty build|test\n\n\
             COMMANDS\n\n  build  Build the project\n  test   Run tests"
    );
}

#[test]
fn usage_subcommand_aliases() {
    let command = Command::new("ritty").command(Command::new("build").alias("b"));

    assert_eq!(
        command.render_usage(),
        "USAGE ritty build|b\n\nCOMMANDS\n\n  build, b"
    );
}

#[test]
fn usage_subcommand_descriptions() {
    let command =
        Command::new("ritty").command(Command::new("build").description("Build the project"));

    assert_eq!(
        command.render_usage(),
        "USAGE ritty build\n\nCOMMANDS\n\n  build  Build the project"
    );
}

#[test]
fn usage_hidden_subcommands_omitted() {
    let command = Command::new("ritty")
        .command(Command::new("build"))
        .command(Command::new("internal").hidden());

    assert_eq!(
        command.render_usage(),
        "USAGE ritty build\n\nCOMMANDS\n\n  build"
    );
}

#[test]
fn usage_hidden_subcommand_aliases_omitted() {
    let command = Command::new("ritty")
        .command(Command::new("build"))
        .command(Command::new("internal").alias("i").hidden());

    assert_eq!(
        command.render_usage(),
        "USAGE ritty build\n\nCOMMANDS\n\n  build"
    );
}

#[test]
fn usage_visible_command_alternatives_in_synopsis() {
    let command = Command::new("ritty")
        .command(Command::new("build").alias("b"))
        .command(Command::new("test").alias("t"));

    assert_eq!(
        command.render_usage(),
        "USAGE ritty build|b|test|t\n\nCOMMANDS\n\n  build, b\n  test, t"
    );
}

#[test]
fn usage_all_hidden_subcommands_leave_no_commands_section() {
    let command = Command::new("ritty").command(Command::new("internal").hidden());

    assert_eq!(command.render_usage(), "USAGE ritty");
}

// -- Usage: empty / clean rendering --

#[test]
fn usage_command_with_no_metadata_renders_cleanly() {
    let command = Command::new("ritty");

    assert_eq!(command.render_usage(), "USAGE ritty");
}

#[test]
fn usage_missing_descriptions_do_not_produce_artifacts() {
    let command = Command::new("ritty")
        .arg(Arg::new("target"))
        .flag(Flag::new("verbose"))
        .command(Command::new("build"));

    let rendered = command.render_usage();

    assert!(!rendered.contains("None"));
    assert!(!rendered.contains("Some("));
    assert!(!rendered.contains("undefined"));
}

#[test]
fn usage_has_no_trailing_whitespace_on_any_line() {
    let command = Command::new("ritty")
        .description("Elegant CLI builder")
        .version("1.0.0")
        .arg(Arg::new("target"))
        .flag(Flag::new("verbose").short('v'))
        .option(StringOption::new("output").default("dist"))
        .enum_option(EnumOption::new("level", ["debug", "info"]).required())
        .command(
            Command::new("build")
                .alias("b")
                .description("Build the project"),
        )
        .command(Command::new("internal").hidden());

    let rendered = command.render_usage();

    for line in rendered.lines() {
        assert_eq!(line, line.trim_end());
    }
}

#[test]
fn usage_rendering_is_deterministic_across_repeated_calls() {
    let command = Command::new("ritty")
        .description("Elegant CLI builder")
        .version("1.0.0")
        .arg(Arg::new("target").required())
        .flag(Flag::new("verbose").short('v'))
        .option(StringOption::new("output").alias("o").default("dist"))
        .command(Command::new("build").alias("b"));

    let first = command.render_usage();
    let second = command.render_usage();

    assert_eq!(first, second);
}

#[test]
fn unknown_long_option_has_unknown_option_kind() {
    let command = Command::new("ritty");

    let error = command.parse_from(["--wat"]).unwrap_err();

    assert_eq!(
        error.kind(),
        ParseErrorKind::Argument(ArgumentErrorKind::UnknownOption)
    );
}

#[test]
fn unknown_short_option_has_unknown_option_kind() {
    let command = Command::new("ritty");

    let error = command.parse_from(["-x"]).unwrap_err();

    assert_eq!(
        error.kind(),
        ParseErrorKind::Argument(ArgumentErrorKind::UnknownOption)
    );
}

#[test]
fn ambiguous_option_has_ambiguous_option_kind() {
    let command = Command::new("ritty")
        .flag(Flag::new("verbose").short('v'))
        .option(StringOption::new("value").alias("v"));

    let error = command.parse_from(["-v"]).unwrap_err();

    assert_eq!(
        error.kind(),
        ParseErrorKind::Argument(ArgumentErrorKind::AmbiguousOption)
    );
}

#[test]
fn missing_string_option_value_has_missing_option_value_kind() {
    let command = Command::new("ritty").option(StringOption::new("name"));

    let error = command.parse_from(["--name"]).unwrap_err();

    assert_eq!(
        error.kind(),
        ParseErrorKind::Argument(ArgumentErrorKind::MissingOptionValue)
    );
}

#[test]
fn missing_enum_option_value_has_missing_option_value_kind() {
    let command = Command::new("ritty").enum_option(EnumOption::new("level", ["debug", "info"]));

    let error = command.parse_from(["--level"]).unwrap_err();

    assert_eq!(
        error.kind(),
        ParseErrorKind::Argument(ArgumentErrorKind::MissingOptionValue)
    );
}

#[test]
fn invalid_enum_value_has_invalid_option_value_kind() {
    let command = Command::new("ritty").enum_option(EnumOption::new("level", ["debug", "info"]));

    let error = command.parse_from(["--level", "nope"]).unwrap_err();

    assert_eq!(
        error.kind(),
        ParseErrorKind::Argument(ArgumentErrorKind::InvalidOptionValue)
    );
}

#[test]
fn missing_required_positional_has_missing_required_argument_kind() {
    let command = Command::new("ritty").arg(Arg::new("target").required());

    let error = command.parse_from([] as [&str; 0]).unwrap_err();

    assert_eq!(
        error.kind(),
        ParseErrorKind::Argument(ArgumentErrorKind::MissingRequiredArgument)
    );
}

#[test]
fn missing_required_boolean_flag_has_missing_required_option_kind() {
    let command = Command::new("ritty").flag(Flag::new("confirm").required());

    let error = command.parse_from([] as [&str; 0]).unwrap_err();

    assert_eq!(
        error.kind(),
        ParseErrorKind::Argument(ArgumentErrorKind::MissingRequiredOption)
    );
}

#[test]
fn missing_required_string_option_has_missing_required_option_kind() {
    let command = Command::new("ritty").option(StringOption::new("name").required());

    let error = command.parse_from([] as [&str; 0]).unwrap_err();

    assert_eq!(
        error.kind(),
        ParseErrorKind::Argument(ArgumentErrorKind::MissingRequiredOption)
    );
}

#[test]
fn missing_required_enum_option_has_missing_required_option_kind() {
    let command =
        Command::new("ritty").enum_option(EnumOption::new("level", ["debug", "info"]).required());

    let error = command.parse_from([] as [&str; 0]).unwrap_err();

    assert_eq!(
        error.kind(),
        ParseErrorKind::Argument(ArgumentErrorKind::MissingRequiredOption)
    );
}

#[test]
fn unexpected_positional_has_unexpected_argument_kind() {
    let command = Command::new("ritty");

    let error = command.parse_from(["extra"]).unwrap_err();

    assert_eq!(
        error.kind(),
        ParseErrorKind::Argument(ArgumentErrorKind::UnexpectedArgument)
    );
}

#[test]
fn unknown_command_has_unknown_command_kind() {
    let command = Command::new("ritty").command(Command::new("build"));

    let error = command.parse_from(["deploy"]).unwrap_err();

    assert_eq!(error.kind(), ParseErrorKind::UnknownCommand);
}

#[test]
fn ambiguous_command_has_ambiguous_command_kind() {
    let command = Command::new("ritty")
        .command(Command::new("install").alias("x"))
        .command(Command::new("inspect").alias("x"));

    let error = command.parse_from(["x"]).unwrap_err();

    assert_eq!(error.kind(), ParseErrorKind::AmbiguousCommand);
}

#[test]
fn missing_configured_default_subcommand_has_default_subcommand_not_found_kind() {
    let command = Command::new("root").default_subcommand("build");

    let error = command.parse_from([] as [&str; 0]).unwrap_err();

    assert_eq!(error.kind(), ParseErrorKind::DefaultSubcommandNotFound);
}

#[test]
fn nested_default_child_error_retains_original_kind() {
    let command = Command::new("root")
        .default_subcommand("build")
        .command(Command::new("build").option(StringOption::new("output").required()));

    let error = command.parse_from([] as [&str; 0]).unwrap_err();

    assert_eq!(
        error.kind(),
        ParseErrorKind::Argument(ArgumentErrorKind::MissingRequiredOption)
    );
}

#[test]
fn parse_error_display_equals_message() {
    let command = Command::new("ritty");

    let error = command.parse_from(["--wat"]).unwrap_err();

    assert_eq!(error.to_string(), error.message());
}

#[test]
fn parse_error_implements_std_error() {
    fn assert_error<E: std::error::Error>(_: &E) {}

    let command = Command::new("ritty");
    let error = command.parse_from(["--wat"]).unwrap_err();

    assert_error(&error);
}

// --- Execution ---

use std::sync::Mutex;

#[derive(Debug)]
struct Boom;

impl std::fmt::Display for Boom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("boom")
    }
}

impl std::error::Error for Boom {}

#[test]
fn handler_defaults_to_absent() {
    let command = Command::new("root");

    assert!(!command.has_handler());
}

#[test]
fn root_handler_executes() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let recorded = Arc::clone(&calls);
    let command = Command::new("root").handler(move |_ctx| {
        recorded.lock().unwrap().push("root");
        Ok(())
    });

    command.run_from([] as [&str; 0]).unwrap();

    assert_eq!(*calls.lock().unwrap(), vec!["root"]);
}

#[test]
fn handler_receives_its_local_matches() {
    let command = Command::new("root").arg(Arg::new("name")).handler(|ctx| {
        assert_eq!(ctx.matches().argument("name"), Some("alice"));
        Ok(())
    });

    command.run_from(["alice"]).unwrap();
}

#[test]
fn handler_receives_root_matches() {
    let command = Command::new("root")
        .flag(Flag::new("verbose").short('v'))
        .command(Command::new("build").handler(|ctx| {
            assert!(ctx.root_matches().flag("verbose"));
            assert_eq!(ctx.root_matches().subcommand(), Some("build"));
            Ok(())
        }));

    command.run_from(["-v", "build"]).unwrap();
}

#[test]
fn captured_closure_handler_works() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let recorded = Arc::clone(&calls);
    let command = Command::new("build").handler(move |_ctx| {
        recorded.lock().unwrap().push("build");
        Ok(())
    });

    command.run_from([] as [&str; 0]).unwrap();

    assert_eq!(*calls.lock().unwrap(), vec!["build"]);
}

#[test]
fn cloned_command_retains_working_handler() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let recorded = Arc::clone(&calls);
    let command = Command::new("root").handler(move |_ctx| {
        recorded.lock().unwrap().push("root");
        Ok(())
    });

    let cloned = command.clone();
    cloned.run_from([] as [&str; 0]).unwrap();

    assert_eq!(*calls.lock().unwrap(), vec!["root"]);
}

#[test]
fn root_options_are_parsed_before_root_handler_runs() {
    let command = Command::new("root")
        .flag(Flag::new("verbose").short('v'))
        .handler(|ctx| {
            assert!(ctx.matches().flag("verbose"));
            Ok(())
        });

    command.run_from(["-v"]).unwrap();
}

#[test]
fn explicit_child_handler_executes() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let recorded = Arc::clone(&calls);
    let command = Command::new("root").command(Command::new("build").handler(move |_ctx| {
        recorded.lock().unwrap().push("build");
        Ok(())
    }));

    command.run_from(["build"]).unwrap();

    assert_eq!(*calls.lock().unwrap(), vec!["build"]);
}

#[test]
fn parent_handler_is_suppressed_when_child_selected() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let root_calls = Arc::clone(&calls);
    let build_calls = Arc::clone(&calls);
    let command = Command::new("root")
        .handler(move |_ctx| {
            root_calls.lock().unwrap().push("root");
            Ok(())
        })
        .command(Command::new("build").handler(move |_ctx| {
            build_calls.lock().unwrap().push("build");
            Ok(())
        }));

    command.run_from(["build"]).unwrap();

    assert_eq!(*calls.lock().unwrap(), vec!["build"]);
}

#[test]
fn subcommand_alias_executes_canonical_child_handler() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let recorded = Arc::clone(&calls);
    let command =
        Command::new("root").command(Command::new("build").alias("b").handler(move |_ctx| {
            recorded.lock().unwrap().push("build");
            Ok(())
        }));

    command.run_from(["b"]).unwrap();

    assert_eq!(*calls.lock().unwrap(), vec!["build"]);
}

#[test]
fn nested_leaf_handler_executes() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let recorded = Arc::clone(&calls);
    let command = Command::new("root").command(Command::new("remote").command(
        Command::new("add").handler(move |_ctx| {
            recorded.lock().unwrap().push("add");
            Ok(())
        }),
    ));

    command.run_from(["remote", "add"]).unwrap();

    assert_eq!(*calls.lock().unwrap(), vec!["add"]);
}

#[test]
fn intermediate_parent_handlers_are_not_executed() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let root_calls = Arc::clone(&calls);
    let remote_calls = Arc::clone(&calls);
    let add_calls = Arc::clone(&calls);
    let command = Command::new("root")
        .handler(move |_ctx| {
            root_calls.lock().unwrap().push("root");
            Ok(())
        })
        .command(
            Command::new("remote")
                .handler(move |_ctx| {
                    remote_calls.lock().unwrap().push("remote");
                    Ok(())
                })
                .command(Command::new("add").handler(move |_ctx| {
                    add_calls.lock().unwrap().push("add");
                    Ok(())
                })),
        );

    command.run_from(["remote", "add"]).unwrap();

    assert_eq!(*calls.lock().unwrap(), vec!["add"]);
}

#[test]
fn default_child_handler_executes_on_empty_argv() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let recorded = Arc::clone(&calls);
    let command =
        Command::new("root")
            .default_subcommand("dev")
            .command(Command::new("dev").handler(move |_ctx| {
                recorded.lock().unwrap().push("dev");
                Ok(())
            }));

    command.run_from([] as [&str; 0]).unwrap();

    assert_eq!(*calls.lock().unwrap(), vec!["dev"]);
}

#[test]
fn default_subcommand_alias_executes_canonical_child() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let recorded = Arc::clone(&calls);
    let command = Command::new("root").default_subcommand("d").command(
        Command::new("dev").alias("d").handler(move |_ctx| {
            recorded.lock().unwrap().push("dev");
            Ok(())
        }),
    );

    command.run_from([] as [&str; 0]).unwrap();

    assert_eq!(*calls.lock().unwrap(), vec!["dev"]);
}

#[test]
fn hidden_subcommand_remains_executable() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let recorded = Arc::clone(&calls);
    let command =
        Command::new("root").command(Command::new("secret").hidden().handler(move |_ctx| {
            recorded.lock().unwrap().push("secret");
            Ok(())
        }));

    command.run_from(["secret"]).unwrap();

    assert_eq!(*calls.lock().unwrap(), vec!["secret"]);
}

#[test]
fn child_handler_receives_child_matches() {
    let command =
        Command::new("root").command(Command::new("build").arg(Arg::new("target")).handler(
            |ctx| {
                assert_eq!(ctx.matches().argument("target"), Some("web"));
                Ok(())
            },
        ));

    command.run_from(["build", "web"]).unwrap();
}

#[test]
fn child_handler_can_inspect_parent_and_root_matches() {
    let command = Command::new("root")
        .flag(Flag::new("verbose").short('v'))
        .command(Command::new("build").handler(|ctx| {
            assert!(ctx.root_matches().flag("verbose"));
            assert!(ctx.matches().argument("target").is_none());
            Ok(())
        }));

    command.run_from(["-v", "build"]).unwrap();
}

#[test]
fn parse_failure_becomes_run_error_parse() {
    let command = Command::new("root");

    let error = command.run_from(["--wat"]).unwrap_err();

    assert!(matches!(error, RunError::Parse(_)));
}

#[test]
fn run_error_parse_kind_is_preserved() {
    let command = Command::new("root");

    let error = command.run_from(["--wat"]).unwrap_err();

    let RunError::Parse(parse_error) = error else {
        panic!("expected RunError::Parse");
    };
    assert_eq!(
        parse_error.kind(),
        ParseErrorKind::Argument(ArgumentErrorKind::UnknownOption)
    );
}

#[test]
fn run_error_parse_message_is_preserved() {
    let command = Command::new("root");

    let direct = command.parse_from(["--wat"]).unwrap_err();
    let error = command.run_from(["--wat"]).unwrap_err();

    let RunError::Parse(parse_error) = error else {
        panic!("expected RunError::Parse");
    };
    assert_eq!(parse_error.message(), direct.message());
}

#[test]
fn unresolved_required_child_returns_no_command() {
    let command = Command::new("root").command(Command::new("build").handler(|_ctx| Ok(())));

    let error = command.run_from([] as [&str; 0]).unwrap_err();

    assert!(matches!(error, RunError::NoCommand));
}

#[test]
fn empty_no_handler_command_succeeds_as_no_op() {
    let command = Command::new("root");

    command.run_from([] as [&str; 0]).unwrap();
}

#[test]
fn selected_leaf_without_handler_succeeds_as_no_op() {
    let command = Command::new("root").command(Command::new("build"));

    command.run_from(["build"]).unwrap();
}

#[test]
fn handler_failure_becomes_handler_error_variant() {
    let command = Command::new("root").handler(|_ctx| Err::<(), _>(Box::new(Boom) as BoxError));

    let error = command.run_from([] as [&str; 0]).unwrap_err();

    assert!(matches!(error, RunError::Handler(_)));
}

#[test]
fn handler_error_is_exposed_through_source() {
    let command = Command::new("root").handler(|_ctx| Err::<(), _>(Box::new(Boom) as BoxError));

    let error = command.run_from([] as [&str; 0]).unwrap_err();

    let source = std::error::Error::source(&error).expect("handler error has a source");
    assert_eq!(source.to_string(), "boom");
}

#[test]
fn run_error_implements_display() {
    let command = Command::new("root").command(Command::new("build").handler(|_ctx| Ok(())));

    let error = command.run_from([] as [&str; 0]).unwrap_err();

    assert_eq!(error.to_string(), "no command specified");
}

#[test]
fn run_error_implements_std_error() {
    fn assert_error<E: std::error::Error>(_: &E) {}

    let command = Command::new("root");
    let error = command.run_from(["--wat"]).unwrap_err();

    assert_error(&error);
}

#[test]
fn parse_error_prevents_any_handler_call() {
    let calls = Arc::new(Mutex::new(0));
    let recorded = Arc::clone(&calls);
    let command = Command::new("root")
        .handler(move |_ctx| {
            *recorded.lock().unwrap() += 1;
            Ok(())
        })
        .command(Command::new("build").handler(|_ctx| -> HandlerResult {
            panic!("child handler must not run on parse failure");
        }));

    let error = command.run_from(["--wat"]).unwrap_err();

    assert!(matches!(error, RunError::Parse(_)));
    assert_eq!(*calls.lock().unwrap(), 0);
}

#[test]
fn setup_defaults_to_absent() {
    let command = Command::new("root");

    assert!(!command.has_setup());
}

#[test]
fn cleanup_defaults_to_absent() {
    let command = Command::new("root");

    assert!(!command.has_cleanup());
}

#[test]
fn has_setup_reports_true_once_set() {
    let command = Command::new("root").setup(|_ctx| Ok(()));

    assert!(command.has_setup());
}

#[test]
fn has_cleanup_reports_true_once_set() {
    let command = Command::new("root").cleanup(|_ctx| Ok(()));

    assert!(command.has_cleanup());
}

#[test]
fn root_setup_runs_before_root_handler() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let setup_calls = Arc::clone(&calls);
    let handler_calls = Arc::clone(&calls);
    let command = Command::new("root")
        .setup(move |_ctx| {
            setup_calls.lock().unwrap().push("setup");
            Ok(())
        })
        .handler(move |_ctx| {
            handler_calls.lock().unwrap().push("handler");
            Ok(())
        });

    command.run_from([] as [&str; 0]).unwrap();

    assert_eq!(*calls.lock().unwrap(), vec!["setup", "handler"]);
}

#[test]
fn root_cleanup_runs_after_root_handler() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let handler_calls = Arc::clone(&calls);
    let cleanup_calls = Arc::clone(&calls);
    let command = Command::new("root")
        .handler(move |_ctx| {
            handler_calls.lock().unwrap().push("handler");
            Ok(())
        })
        .cleanup(move |_ctx| {
            cleanup_calls.lock().unwrap().push("cleanup");
            Ok(())
        });

    command.run_from([] as [&str; 0]).unwrap();

    assert_eq!(*calls.lock().unwrap(), vec!["handler", "cleanup"]);
}

#[test]
fn exact_setup_handler_cleanup_order() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let setup_calls = Arc::clone(&calls);
    let handler_calls = Arc::clone(&calls);
    let cleanup_calls = Arc::clone(&calls);
    let command = Command::new("root")
        .setup(move |_ctx| {
            setup_calls.lock().unwrap().push("setup");
            Ok(())
        })
        .handler(move |_ctx| {
            handler_calls.lock().unwrap().push("handler");
            Ok(())
        })
        .cleanup(move |_ctx| {
            cleanup_calls.lock().unwrap().push("cleanup");
            Ok(())
        });

    command.run_from([] as [&str; 0]).unwrap();

    assert_eq!(*calls.lock().unwrap(), vec!["setup", "handler", "cleanup"]);
}

#[test]
fn setup_receives_local_matches() {
    let command = Command::new("root")
        .arg(Arg::new("name"))
        .setup(|ctx| {
            assert_eq!(ctx.matches().argument("name"), Some("alice"));
            Ok(())
        })
        .handler(|_ctx| Ok(()));

    command.run_from(["alice"]).unwrap();
}

#[test]
fn cleanup_receives_local_matches() {
    let command = Command::new("root")
        .arg(Arg::new("name"))
        .handler(|_ctx| Ok(()))
        .cleanup(|ctx| {
            assert_eq!(ctx.matches().argument("name"), Some("alice"));
            Ok(())
        });

    command.run_from(["alice"]).unwrap();
}

#[test]
fn hooks_receive_root_matches() {
    let command = Command::new("root")
        .flag(Flag::new("verbose").short('v'))
        .command(
            Command::new("build")
                .setup(|ctx| {
                    assert!(ctx.root_matches().flag("verbose"));
                    Ok(())
                })
                .handler(|_ctx| Ok(()))
                .cleanup(|ctx| {
                    assert!(ctx.root_matches().flag("verbose"));
                    Ok(())
                }),
        );

    command.run_from(["-v", "build"]).unwrap();
}

#[test]
fn captured_setup_closure_works() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let recorded = Arc::clone(&calls);
    let command = Command::new("root")
        .setup(move |_ctx| {
            recorded.lock().unwrap().push("setup");
            Ok(())
        })
        .handler(|_ctx| Ok(()));

    command.run_from([] as [&str; 0]).unwrap();

    assert_eq!(*calls.lock().unwrap(), vec!["setup"]);
}

#[test]
fn captured_cleanup_closure_works() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let recorded = Arc::clone(&calls);
    let command = Command::new("root")
        .handler(|_ctx| Ok(()))
        .cleanup(move |_ctx| {
            recorded.lock().unwrap().push("cleanup");
            Ok(())
        });

    command.run_from([] as [&str; 0]).unwrap();

    assert_eq!(*calls.lock().unwrap(), vec!["cleanup"]);
}

#[test]
fn cloned_command_retains_working_setup() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let recorded = Arc::clone(&calls);
    let command = Command::new("root")
        .setup(move |_ctx| {
            recorded.lock().unwrap().push("setup");
            Ok(())
        })
        .handler(|_ctx| Ok(()));

    let cloned = command.clone();
    cloned.run_from([] as [&str; 0]).unwrap();

    assert_eq!(*calls.lock().unwrap(), vec!["setup"]);
}

#[test]
fn cloned_command_retains_working_cleanup() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let recorded = Arc::clone(&calls);
    let command = Command::new("root")
        .handler(|_ctx| Ok(()))
        .cleanup(move |_ctx| {
            recorded.lock().unwrap().push("cleanup");
            Ok(())
        });

    let cloned = command.clone();
    cloned.run_from([] as [&str; 0]).unwrap();

    assert_eq!(*calls.lock().unwrap(), vec!["cleanup"]);
}

#[test]
fn nested_setup_runs_root_to_leaf() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let root_calls = Arc::clone(&calls);
    let child_calls = Arc::clone(&calls);
    let leaf_calls = Arc::clone(&calls);
    let command = Command::new("root")
        .setup(move |_ctx| {
            root_calls.lock().unwrap().push("root");
            Ok(())
        })
        .command(
            Command::new("child")
                .setup(move |_ctx| {
                    child_calls.lock().unwrap().push("child");
                    Ok(())
                })
                .command(
                    Command::new("leaf")
                        .setup(move |_ctx| {
                            leaf_calls.lock().unwrap().push("leaf");
                            Ok(())
                        })
                        .handler(|_ctx| Ok(())),
                ),
        );

    command.run_from(["child", "leaf"]).unwrap();

    assert_eq!(*calls.lock().unwrap(), vec!["root", "child", "leaf"]);
}

#[test]
fn nested_cleanup_runs_leaf_to_root() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let root_calls = Arc::clone(&calls);
    let child_calls = Arc::clone(&calls);
    let leaf_calls = Arc::clone(&calls);
    let command = Command::new("root")
        .cleanup(move |_ctx| {
            root_calls.lock().unwrap().push("root");
            Ok(())
        })
        .command(
            Command::new("child")
                .cleanup(move |_ctx| {
                    child_calls.lock().unwrap().push("child");
                    Ok(())
                })
                .command(
                    Command::new("leaf")
                        .handler(|_ctx| Ok(()))
                        .cleanup(move |_ctx| {
                            leaf_calls.lock().unwrap().push("leaf");
                            Ok(())
                        }),
                ),
        );

    command.run_from(["child", "leaf"]).unwrap();

    assert_eq!(*calls.lock().unwrap(), vec!["leaf", "child", "root"]);
}

#[test]
fn only_leaf_handler_executes_with_full_lifecycle() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let root_setup = Arc::clone(&calls);
    let root_handler = Arc::clone(&calls);
    let leaf_handler = Arc::clone(&calls);
    let command = Command::new("root")
        .setup(move |_ctx| {
            root_setup.lock().unwrap().push("root-setup");
            Ok(())
        })
        .handler(move |_ctx| {
            root_handler.lock().unwrap().push("root-handler");
            Ok(())
        })
        .command(Command::new("leaf").handler(move |_ctx| {
            leaf_handler.lock().unwrap().push("leaf-handler");
            Ok(())
        }));

    command.run_from(["leaf"]).unwrap();

    assert_eq!(*calls.lock().unwrap(), vec!["root-setup", "leaf-handler"]);
}

#[test]
fn handlerless_intermediate_command_hooks_execute() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let root_setup = Arc::clone(&calls);
    let workspace_setup = Arc::clone(&calls);
    let workspace_cleanup = Arc::clone(&calls);
    let deploy_handler = Arc::clone(&calls);
    let command = Command::new("root")
        .setup(move |_ctx| {
            root_setup.lock().unwrap().push("root-setup");
            Ok(())
        })
        .command(
            Command::new("workspace")
                .setup(move |_ctx| {
                    workspace_setup.lock().unwrap().push("workspace-setup");
                    Ok(())
                })
                .cleanup(move |_ctx| {
                    workspace_cleanup.lock().unwrap().push("workspace-cleanup");
                    Ok(())
                })
                .command(Command::new("deploy").handler(move |_ctx| {
                    deploy_handler.lock().unwrap().push("deploy-handler");
                    Ok(())
                })),
        );

    command.run_from(["workspace", "deploy"]).unwrap();

    assert_eq!(
        *calls.lock().unwrap(),
        vec![
            "root-setup",
            "workspace-setup",
            "deploy-handler",
            "workspace-cleanup"
        ]
    );
}

#[test]
fn explicit_child_lifecycle_runs_all_hooks() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let root_setup = Arc::clone(&calls);
    let root_cleanup = Arc::clone(&calls);
    let build_setup = Arc::clone(&calls);
    let build_handler = Arc::clone(&calls);
    let build_cleanup = Arc::clone(&calls);
    let command = Command::new("root")
        .setup(move |_ctx| {
            root_setup.lock().unwrap().push("root-setup");
            Ok(())
        })
        .cleanup(move |_ctx| {
            root_cleanup.lock().unwrap().push("root-cleanup");
            Ok(())
        })
        .command(
            Command::new("build")
                .setup(move |_ctx| {
                    build_setup.lock().unwrap().push("build-setup");
                    Ok(())
                })
                .handler(move |_ctx| {
                    build_handler.lock().unwrap().push("build-handler");
                    Ok(())
                })
                .cleanup(move |_ctx| {
                    build_cleanup.lock().unwrap().push("build-cleanup");
                    Ok(())
                }),
        );

    command.run_from(["build"]).unwrap();

    assert_eq!(
        *calls.lock().unwrap(),
        vec![
            "root-setup",
            "build-setup",
            "build-handler",
            "build-cleanup",
            "root-cleanup"
        ]
    );
}

#[test]
fn alias_selected_child_lifecycle_runs() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let setup_calls = Arc::clone(&calls);
    let cleanup_calls = Arc::clone(&calls);
    let command = Command::new("root").command(
        Command::new("build")
            .alias("b")
            .setup(move |_ctx| {
                setup_calls.lock().unwrap().push("setup");
                Ok(())
            })
            .handler(|_ctx| Ok(()))
            .cleanup(move |_ctx| {
                cleanup_calls.lock().unwrap().push("cleanup");
                Ok(())
            }),
    );

    command.run_from(["b"]).unwrap();

    assert_eq!(*calls.lock().unwrap(), vec!["setup", "cleanup"]);
}

#[test]
fn default_child_lifecycle_runs() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let setup_calls = Arc::clone(&calls);
    let cleanup_calls = Arc::clone(&calls);
    let command = Command::new("root").default_subcommand("dev").command(
        Command::new("dev")
            .setup(move |_ctx| {
                setup_calls.lock().unwrap().push("setup");
                Ok(())
            })
            .handler(|_ctx| Ok(()))
            .cleanup(move |_ctx| {
                cleanup_calls.lock().unwrap().push("cleanup");
                Ok(())
            }),
    );

    command.run_from([] as [&str; 0]).unwrap();

    assert_eq!(*calls.lock().unwrap(), vec!["setup", "cleanup"]);
}

#[test]
fn hidden_child_lifecycle_runs() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let setup_calls = Arc::clone(&calls);
    let cleanup_calls = Arc::clone(&calls);
    let command = Command::new("root").command(
        Command::new("secret")
            .hidden()
            .setup(move |_ctx| {
                setup_calls.lock().unwrap().push("setup");
                Ok(())
            })
            .handler(|_ctx| Ok(()))
            .cleanup(move |_ctx| {
                cleanup_calls.lock().unwrap().push("cleanup");
                Ok(())
            }),
    );

    command.run_from(["secret"]).unwrap();

    assert_eq!(*calls.lock().unwrap(), vec!["setup", "cleanup"]);
}

#[test]
fn cleanup_runs_after_handler_failure() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let cleanup_calls = Arc::clone(&calls);
    let command = Command::new("root")
        .handler(|_ctx| Err::<(), _>(Box::new(Boom) as BoxError))
        .cleanup(move |_ctx| {
            cleanup_calls.lock().unwrap().push("cleanup");
            Ok(())
        });

    let error = command.run_from([] as [&str; 0]).unwrap_err();

    assert!(matches!(error, RunError::Handler(_)));
    assert_eq!(*calls.lock().unwrap(), vec!["cleanup"]);
}

#[test]
fn handler_error_remains_primary_when_cleanup_also_fails() {
    let command = Command::new("root")
        .handler(|_ctx| Err::<(), _>(Box::new(Boom) as BoxError))
        .cleanup(|_ctx| Err(Box::new(Boom) as BoxError));

    let error = command.run_from([] as [&str; 0]).unwrap_err();

    assert!(matches!(error, RunError::Handler(_)));
}

#[test]
fn cleanup_runs_after_setup_failure() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let cleanup_calls = Arc::clone(&calls);
    let command = Command::new("root")
        .setup(|_ctx| Err(Box::new(Boom) as BoxError))
        .cleanup(move |_ctx| {
            cleanup_calls.lock().unwrap().push("cleanup");
            Ok(())
        });

    let error = command.run_from([] as [&str; 0]).unwrap_err();

    assert!(matches!(error, RunError::Setup(_)));
    assert_eq!(*calls.lock().unwrap(), vec!["cleanup"]);
}

#[test]
fn setup_error_remains_primary_when_cleanup_also_fails() {
    let command = Command::new("root")
        .setup(|_ctx| Err(Box::new(Boom) as BoxError))
        .cleanup(|_ctx| Err(Box::new(Boom) as BoxError));

    let error = command.run_from([] as [&str; 0]).unwrap_err();

    assert!(matches!(error, RunError::Setup(_)));
}

#[test]
fn setup_failure_prevents_handler_invocation() {
    let calls = Arc::new(Mutex::new(0));
    let recorded = Arc::clone(&calls);
    let command = Command::new("root")
        .setup(|_ctx| Err(Box::new(Boom) as BoxError))
        .handler(move |_ctx| {
            *recorded.lock().unwrap() += 1;
            Ok(())
        });

    let error = command.run_from([] as [&str; 0]).unwrap_err();

    assert!(matches!(error, RunError::Setup(_)));
    assert_eq!(*calls.lock().unwrap(), 0);
}

#[test]
fn setup_failure_prevents_child_entry() {
    let calls = Arc::new(Mutex::new(0));
    let recorded = Arc::clone(&calls);
    let command = Command::new("root")
        .setup(|_ctx| Err(Box::new(Boom) as BoxError))
        .command(Command::new("build").handler(move |_ctx| {
            *recorded.lock().unwrap() += 1;
            Ok(())
        }));

    let error = command.run_from(["build"]).unwrap_err();

    assert!(matches!(error, RunError::Setup(_)));
    assert_eq!(*calls.lock().unwrap(), 0);
}

#[test]
fn parent_cleanup_runs_after_child_handler_failure() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let cleanup_calls = Arc::clone(&calls);
    let command = Command::new("root")
        .cleanup(move |_ctx| {
            cleanup_calls.lock().unwrap().push("root-cleanup");
            Ok(())
        })
        .command(Command::new("build").handler(|_ctx| Err::<(), _>(Box::new(Boom) as BoxError)));

    let error = command.run_from(["build"]).unwrap_err();

    assert!(matches!(error, RunError::Handler(_)));
    assert_eq!(*calls.lock().unwrap(), vec!["root-cleanup"]);
}

#[test]
fn child_error_remains_primary_when_parent_cleanup_fails() {
    let command = Command::new("root")
        .cleanup(|_ctx| Err(Box::new(Boom) as BoxError))
        .command(Command::new("build").handler(|_ctx| Err::<(), _>(Box::new(Boom) as BoxError)));

    let error = command.run_from(["build"]).unwrap_err();

    assert!(matches!(error, RunError::Handler(_)));
}

#[test]
fn cleanup_only_failure_becomes_run_error_cleanup() {
    let command = Command::new("root")
        .handler(|_ctx| Ok(()))
        .cleanup(|_ctx| Err(Box::new(Boom) as BoxError));

    let error = command.run_from([] as [&str; 0]).unwrap_err();

    assert!(matches!(error, RunError::Cleanup(_)));
}

#[test]
fn deepest_cleanup_failure_wins_over_later_parent_cleanup_failure() {
    let command = Command::new("root")
        .cleanup(|_ctx| Err(Box::new(Boom) as BoxError))
        .command(
            Command::new("build")
                .handler(|_ctx| Ok(()))
                .cleanup(|_ctx| Err(Box::new(Boom) as BoxError)),
        );

    let error = command.run_from(["build"]).unwrap_err();

    // Both cleanups fail with the same error type/message, but the leaf's
    // cleanup failure must be the one that establishes the primary
    // failure the parent's cleanup then fails to override.
    assert!(matches!(error, RunError::Cleanup(_)));
    let source = std::error::Error::source(&error).expect("cleanup error has a source");
    assert_eq!(source.to_string(), "boom");
}

#[test]
fn cleanup_runs_around_no_command() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let setup_calls = Arc::clone(&calls);
    let cleanup_calls = Arc::clone(&calls);
    let command = Command::new("root")
        .setup(move |_ctx| {
            setup_calls.lock().unwrap().push("setup");
            Ok(())
        })
        .cleanup(move |_ctx| {
            cleanup_calls.lock().unwrap().push("cleanup");
            Ok(())
        })
        .command(Command::new("build").handler(|_ctx| Ok(())));

    let error = command.run_from([] as [&str; 0]).unwrap_err();

    assert!(matches!(error, RunError::NoCommand));
    assert_eq!(*calls.lock().unwrap(), vec!["setup", "cleanup"]);
}

#[test]
fn empty_no_op_command_still_runs_setup_and_cleanup() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let setup_calls = Arc::clone(&calls);
    let cleanup_calls = Arc::clone(&calls);
    let command = Command::new("root")
        .setup(move |_ctx| {
            setup_calls.lock().unwrap().push("setup");
            Ok(())
        })
        .cleanup(move |_ctx| {
            cleanup_calls.lock().unwrap().push("cleanup");
            Ok(())
        });

    command.run_from([] as [&str; 0]).unwrap();

    assert_eq!(*calls.lock().unwrap(), vec!["setup", "cleanup"]);
}

#[test]
fn parse_failure_invokes_zero_lifecycle_callbacks() {
    let calls = Arc::new(Mutex::new(0));
    let setup_calls = Arc::clone(&calls);
    let handler_calls = Arc::clone(&calls);
    let cleanup_calls = Arc::clone(&calls);
    let command = Command::new("root")
        .setup(move |_ctx| {
            *setup_calls.lock().unwrap() += 1;
            Ok(())
        })
        .handler(move |_ctx| {
            *handler_calls.lock().unwrap() += 1;
            Ok(())
        })
        .cleanup(move |_ctx| {
            *cleanup_calls.lock().unwrap() += 1;
            Ok(())
        });

    let error = command.run_from(["--wat"]).unwrap_err();

    assert!(matches!(error, RunError::Parse(_)));
    assert_eq!(*calls.lock().unwrap(), 0);
}

#[test]
fn run_error_setup_exposes_source() {
    let command = Command::new("root").setup(|_ctx| Err(Box::new(Boom) as BoxError));

    let error = command.run_from([] as [&str; 0]).unwrap_err();

    let source = std::error::Error::source(&error).expect("setup error has a source");
    assert_eq!(source.to_string(), "boom");
}

#[test]
fn run_error_cleanup_exposes_source() {
    let command = Command::new("root")
        .handler(|_ctx| Ok(()))
        .cleanup(|_ctx| Err(Box::new(Boom) as BoxError));

    let error = command.run_from([] as [&str; 0]).unwrap_err();

    let source = std::error::Error::source(&error).expect("cleanup error has a source");
    assert_eq!(source.to_string(), "boom");
}

// --- plugins ---

fn recording_plugin(name: &str, log: &Arc<Mutex<Vec<String>>>) -> Plugin {
    let setup_log = Arc::clone(log);
    let cleanup_log = Arc::clone(log);
    let setup_tag = format!("{name}-setup");
    let cleanup_tag = format!("{name}-cleanup");
    Plugin::new(name)
        .setup(move |_ctx| {
            setup_log.lock().unwrap().push(setup_tag.clone());
            Ok(())
        })
        .cleanup(move |_ctx| {
            cleanup_log.lock().unwrap().push(cleanup_tag.clone());
            Ok(())
        })
}

fn failing_setup_plugin(name: &str, log: &Arc<Mutex<Vec<String>>>) -> Plugin {
    let setup_log = Arc::clone(log);
    let setup_tag = format!("{name}-setup");
    Plugin::new(name).setup(move |_ctx| {
        setup_log.lock().unwrap().push(setup_tag.clone());
        Err(Box::new(Boom) as BoxError)
    })
}

fn failing_cleanup_plugin(name: &str, log: &Arc<Mutex<Vec<String>>>) -> Plugin {
    let cleanup_log = Arc::clone(log);
    let cleanup_tag = format!("{name}-cleanup");
    Plugin::new(name).cleanup(move |_ctx| {
        cleanup_log.lock().unwrap().push(cleanup_tag.clone());
        Err(Box::new(Boom) as BoxError)
    })
}

#[test]
fn new_plugin_stores_name() {
    assert_eq!(Plugin::new("logger").name(), "logger");
}

#[test]
fn new_plugin_has_no_setup() {
    assert!(!Plugin::new("logger").has_setup());
}

#[test]
fn new_plugin_has_no_cleanup() {
    assert!(!Plugin::new("logger").has_cleanup());
}

#[test]
fn plugin_setup_builder_sets_has_setup() {
    let plugin = Plugin::new("logger").setup(|_ctx| Ok(()));
    assert!(plugin.has_setup());
    assert!(!plugin.has_cleanup());
}

#[test]
fn plugin_cleanup_builder_sets_has_cleanup() {
    let plugin = Plugin::new("logger").cleanup(|_ctx| Ok(()));
    assert!(plugin.has_cleanup());
    assert!(!plugin.has_setup());
}

#[test]
fn command_starts_with_zero_plugins() {
    assert!(Command::new("root").plugins().is_empty());
}

#[test]
fn plugin_appends() {
    let command = Command::new("root")
        .plugin(Plugin::new("a"))
        .plugin(Plugin::new("b"));
    assert_eq!(command.plugins().len(), 2);
}

#[test]
fn plugins_preserve_declaration_order() {
    let command = Command::new("root")
        .plugin(Plugin::new("a"))
        .plugin(Plugin::new("b"))
        .plugin(Plugin::new("c"));
    let names: Vec<&str> = command.plugins().iter().map(Plugin::name).collect();
    assert_eq!(names, vec!["a", "b", "c"]);
}

#[test]
fn duplicate_plugin_names_remain_distinct_entries() {
    let command = Command::new("root")
        .plugin(Plugin::new("logger"))
        .plugin(Plugin::new("logger"));
    assert_eq!(command.plugins().len(), 2);
}

#[test]
fn captured_plugin_setup_closure_runs() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let command = Command::new("root")
        .plugin(recording_plugin("logger", &calls))
        .handler(|_ctx| Ok(()));

    command.run_from([] as [&str; 0]).unwrap();

    assert!(calls.lock().unwrap().contains(&"logger-setup".to_string()));
}

#[test]
fn captured_plugin_cleanup_closure_runs() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let command = Command::new("root")
        .plugin(recording_plugin("logger", &calls))
        .handler(|_ctx| Ok(()));

    command.run_from([] as [&str; 0]).unwrap();

    assert!(
        calls
            .lock()
            .unwrap()
            .contains(&"logger-cleanup".to_string())
    );
}

#[test]
fn cloned_plugin_retains_hooks() {
    let plugin = Plugin::new("logger")
        .setup(|_ctx| Ok(()))
        .cleanup(|_ctx| Ok(()));
    let cloned = plugin.clone();
    assert!(cloned.has_setup());
    assert!(cloned.has_cleanup());
    assert_eq!(cloned.name(), "logger");
}

#[test]
fn cloned_command_retains_attached_plugins() {
    let command = Command::new("root").plugin(Plugin::new("logger"));
    let cloned = command.clone();
    assert_eq!(cloned.plugins().len(), 1);
    assert_eq!(cloned.plugins()[0].name(), "logger");
}

#[test]
fn same_cloned_plugin_can_attach_to_two_commands() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let logger = recording_plugin("logger", &calls);

    let a = Command::new("a")
        .plugin(logger.clone())
        .handler(|_ctx| Ok(()));
    let b = Command::new("b").plugin(logger).handler(|_ctx| Ok(()));

    a.run_from([] as [&str; 0]).unwrap();
    b.run_from([] as [&str; 0]).unwrap();

    let recorded = calls.lock().unwrap();
    assert_eq!(
        recorded
            .iter()
            .filter(|c| c.as_str() == "logger-setup")
            .count(),
        2
    );
}

#[test]
fn single_plugin_setup_runs_before_command_setup() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let setup_calls = Arc::clone(&calls);
    let command = Command::new("root")
        .plugin(recording_plugin("a", &calls))
        .setup(move |_ctx| {
            setup_calls
                .lock()
                .unwrap()
                .push("command-setup".to_string());
            Ok(())
        })
        .handler(|_ctx| Ok(()));

    command.run_from([] as [&str; 0]).unwrap();

    let recorded = calls.lock().unwrap();
    let setup_index = recorded.iter().position(|c| c == "a-setup").unwrap();
    let command_index = recorded.iter().position(|c| c == "command-setup").unwrap();
    assert!(setup_index < command_index);
}

#[test]
fn multiple_plugin_setups_run_in_declaration_order() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let command = Command::new("root")
        .plugin(recording_plugin("a", &calls))
        .plugin(recording_plugin("b", &calls))
        .handler(|_ctx| Ok(()));

    command.run_from([] as [&str; 0]).unwrap();

    let recorded = calls.lock().unwrap();
    assert_eq!(recorded[0], "a-setup");
    assert_eq!(recorded[1], "b-setup");
}

#[test]
fn command_setup_runs_after_all_successful_plugin_setups() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let setup_calls = Arc::clone(&calls);
    let command = Command::new("root")
        .plugin(recording_plugin("a", &calls))
        .plugin(recording_plugin("b", &calls))
        .setup(move |_ctx| {
            setup_calls
                .lock()
                .unwrap()
                .push("command-setup".to_string());
            Ok(())
        })
        .handler(|_ctx| Ok(()));

    command.run_from([] as [&str; 0]).unwrap();

    let recorded = calls.lock().unwrap();
    assert_eq!(recorded[0], "a-setup");
    assert_eq!(recorded[1], "b-setup");
    assert_eq!(recorded[2], "command-setup");
}

#[test]
fn command_cleanup_runs_before_plugin_cleanups() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let cleanup_calls = Arc::clone(&calls);
    let command = Command::new("root")
        .plugin(recording_plugin("a", &calls))
        .cleanup(move |_ctx| {
            cleanup_calls
                .lock()
                .unwrap()
                .push("command-cleanup".to_string());
            Ok(())
        })
        .handler(|_ctx| Ok(()));

    command.run_from([] as [&str; 0]).unwrap();

    let recorded = calls.lock().unwrap();
    let command_index = recorded
        .iter()
        .position(|c| c == "command-cleanup")
        .unwrap();
    let plugin_index = recorded.iter().position(|c| c == "a-cleanup").unwrap();
    assert!(command_index < plugin_index);
}

#[test]
fn multiple_plugin_cleanups_run_reverse_declaration_order() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let command = Command::new("root")
        .plugin(recording_plugin("a", &calls))
        .plugin(recording_plugin("b", &calls))
        .handler(|_ctx| Ok(()));

    command.run_from([] as [&str; 0]).unwrap();

    let recorded = calls.lock().unwrap();
    let cleanups: Vec<&String> = recorded
        .iter()
        .filter(|c| c.ends_with("-cleanup"))
        .collect();
    assert_eq!(cleanups, vec!["b-cleanup", "a-cleanup"]);
}

#[test]
fn complete_success_order_plugin_command_handler_cleanup() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let setup_calls = Arc::clone(&calls);
    let handler_calls = Arc::clone(&calls);
    let cleanup_calls = Arc::clone(&calls);
    let command = Command::new("root")
        .plugin(recording_plugin("a", &calls))
        .plugin(recording_plugin("b", &calls))
        .setup(move |_ctx| {
            setup_calls
                .lock()
                .unwrap()
                .push("command-setup".to_string());
            Ok(())
        })
        .handler(move |_ctx| {
            handler_calls.lock().unwrap().push("handler".to_string());
            Ok(())
        })
        .cleanup(move |_ctx| {
            cleanup_calls
                .lock()
                .unwrap()
                .push("command-cleanup".to_string());
            Ok(())
        });

    command.run_from([] as [&str; 0]).unwrap();

    assert_eq!(
        *calls.lock().unwrap(),
        vec![
            "a-setup",
            "b-setup",
            "command-setup",
            "handler",
            "command-cleanup",
            "b-cleanup",
            "a-cleanup",
        ]
    );
}

#[test]
fn nested_plugin_lifecycle_ordering() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let root_setup = Arc::clone(&calls);
    let root_cleanup = Arc::clone(&calls);
    let child_setup = Arc::clone(&calls);
    let child_cleanup = Arc::clone(&calls);
    let leaf_setup = Arc::clone(&calls);
    let leaf_handler = Arc::clone(&calls);
    let leaf_cleanup = Arc::clone(&calls);

    let command = Command::new("root")
        .plugin(recording_plugin("root-a", &calls))
        .plugin(recording_plugin("root-b", &calls))
        .setup(move |_ctx| {
            root_setup.lock().unwrap().push("root-setup".to_string());
            Ok(())
        })
        .cleanup(move |_ctx| {
            root_cleanup
                .lock()
                .unwrap()
                .push("root-cleanup".to_string());
            Ok(())
        })
        .command(
            Command::new("child")
                .plugin(recording_plugin("child-a", &calls))
                .plugin(recording_plugin("child-b", &calls))
                .setup(move |_ctx| {
                    child_setup.lock().unwrap().push("child-setup".to_string());
                    Ok(())
                })
                .cleanup(move |_ctx| {
                    child_cleanup
                        .lock()
                        .unwrap()
                        .push("child-cleanup".to_string());
                    Ok(())
                })
                .command(
                    Command::new("leaf")
                        .plugin(recording_plugin("leaf-a", &calls))
                        .plugin(recording_plugin("leaf-b", &calls))
                        .setup(move |_ctx| {
                            leaf_setup.lock().unwrap().push("leaf-setup".to_string());
                            Ok(())
                        })
                        .handler(move |_ctx| {
                            leaf_handler
                                .lock()
                                .unwrap()
                                .push("leaf-handler".to_string());
                            Ok(())
                        })
                        .cleanup(move |_ctx| {
                            leaf_cleanup
                                .lock()
                                .unwrap()
                                .push("leaf-cleanup".to_string());
                            Ok(())
                        }),
                ),
        );

    command.run_from(["child", "leaf"]).unwrap();

    assert_eq!(
        *calls.lock().unwrap(),
        vec![
            "root-a-setup",
            "root-b-setup",
            "root-setup",
            "child-a-setup",
            "child-b-setup",
            "child-setup",
            "leaf-a-setup",
            "leaf-b-setup",
            "leaf-setup",
            "leaf-handler",
            "leaf-cleanup",
            "leaf-b-cleanup",
            "leaf-a-cleanup",
            "child-cleanup",
            "child-b-cleanup",
            "child-a-cleanup",
            "root-cleanup",
            "root-b-cleanup",
            "root-a-cleanup",
        ]
    );
}

#[test]
fn parent_handlers_remain_suppressed_with_plugins() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let parent_handler = Arc::clone(&calls);
    let command = Command::new("root")
        .plugin(recording_plugin("root-a", &calls))
        .handler(move |_ctx| {
            parent_handler
                .lock()
                .unwrap()
                .push("root-handler".to_string());
            Ok(())
        })
        .command(Command::new("child").handler(|_ctx| Ok(())));

    command.run_from(["child"]).unwrap();

    assert!(!calls.lock().unwrap().contains(&"root-handler".to_string()));
}

#[test]
fn handlerless_intermediate_command_plugins_run() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let command = Command::new("root")
        .plugin(recording_plugin("root-a", &calls))
        .command(Command::new("child").handler(|_ctx| Ok(())));

    command.run_from(["child"]).unwrap();

    assert!(calls.lock().unwrap().contains(&"root-a-setup".to_string()));
    assert!(
        calls
            .lock()
            .unwrap()
            .contains(&"root-a-cleanup".to_string())
    );
}

#[test]
fn alias_selected_child_plugin_lifecycle() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let command = Command::new("root").command(
        Command::new("child")
            .alias("c")
            .plugin(recording_plugin("child-a", &calls))
            .handler(|_ctx| Ok(())),
    );

    command.run_from(["c"]).unwrap();

    assert_eq!(
        *calls.lock().unwrap(),
        vec!["child-a-setup", "child-a-cleanup"]
    );
}

#[test]
fn default_child_plugin_lifecycle() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let command = Command::new("root").default_subcommand("child").command(
        Command::new("child")
            .plugin(recording_plugin("child-a", &calls))
            .handler(|_ctx| Ok(())),
    );

    command.run_from([] as [&str; 0]).unwrap();

    assert_eq!(
        *calls.lock().unwrap(),
        vec!["child-a-setup", "child-a-cleanup"]
    );
}

#[test]
fn hidden_child_plugin_lifecycle() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let command = Command::new("root").command(
        Command::new("secret")
            .hidden()
            .plugin(recording_plugin("secret-a", &calls))
            .handler(|_ctx| Ok(())),
    );

    command.run_from(["secret"]).unwrap();

    assert_eq!(
        *calls.lock().unwrap(),
        vec!["secret-a-setup", "secret-a-cleanup"]
    );
}

#[test]
fn parse_failure_runs_zero_plugin_callbacks() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let command = Command::new("root")
        .plugin(recording_plugin("a", &calls))
        .handler(|_ctx| Ok(()));

    let error = command.run_from(["--wat"]).unwrap_err();

    assert!(matches!(error, RunError::Parse(_)));
    assert!(calls.lock().unwrap().is_empty());
}

#[test]
fn plugin_setup_failure_prevents_later_plugin_setups() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let command = Command::new("root")
        .plugin(recording_plugin("a", &calls))
        .plugin(failing_setup_plugin("b", &calls))
        .plugin(recording_plugin("c", &calls))
        .handler(|_ctx| Ok(()));

    command.run_from([] as [&str; 0]).unwrap_err();

    assert!(!calls.lock().unwrap().contains(&"c-setup".to_string()));
}

#[test]
fn plugin_setup_failure_prevents_command_setup() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let setup_calls = Arc::clone(&calls);
    let command = Command::new("root")
        .plugin(failing_setup_plugin("a", &calls))
        .setup(move |_ctx| {
            setup_calls
                .lock()
                .unwrap()
                .push("command-setup".to_string());
            Ok(())
        })
        .handler(|_ctx| Ok(()));

    command.run_from([] as [&str; 0]).unwrap_err();

    assert!(!calls.lock().unwrap().contains(&"command-setup".to_string()));
}

#[test]
fn plugin_setup_failure_prevents_handler() {
    let calls = Arc::new(Mutex::new(0));
    let recorded = Arc::clone(&calls);
    let plugin_calls = Arc::new(Mutex::new(Vec::new()));
    let command = Command::new("root")
        .plugin(failing_setup_plugin("a", &plugin_calls))
        .handler(move |_ctx| {
            *recorded.lock().unwrap() += 1;
            Ok(())
        });

    command.run_from([] as [&str; 0]).unwrap_err();

    assert_eq!(*calls.lock().unwrap(), 0);
}

#[test]
fn command_cleanup_runs_after_plugin_setup_failure() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let cleanup_calls = Arc::clone(&calls);
    let command = Command::new("root")
        .plugin(failing_setup_plugin("a", &calls))
        .cleanup(move |_ctx| {
            cleanup_calls
                .lock()
                .unwrap()
                .push("command-cleanup".to_string());
            Ok(())
        })
        .handler(|_ctx| Ok(()));

    command.run_from([] as [&str; 0]).unwrap_err();

    assert!(
        calls
            .lock()
            .unwrap()
            .contains(&"command-cleanup".to_string())
    );
}

#[test]
fn all_plugin_cleanups_run_after_plugin_setup_failure_including_not_yet_setup() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let command = Command::new("root")
        .plugin(recording_plugin("a", &calls))
        .plugin(failing_setup_plugin("b", &calls))
        .plugin(recording_plugin("c", &calls))
        .handler(|_ctx| Ok(()));

    command.run_from([] as [&str; 0]).unwrap_err();

    let recorded = calls.lock().unwrap();
    assert!(recorded.contains(&"c-cleanup".to_string()));
    assert!(recorded.contains(&"a-cleanup".to_string()));
    let cleanups: Vec<&String> = recorded
        .iter()
        .filter(|c| c.ends_with("-cleanup"))
        .collect();
    assert_eq!(cleanups, vec!["c-cleanup", "a-cleanup"]);
}

#[test]
fn plugin_setup_error_remains_primary_when_command_cleanup_fails() {
    let command = Command::new("root")
        .plugin(Plugin::new("a").setup(|_ctx| Err(Box::new(Boom) as BoxError)))
        .cleanup(|_ctx| Err(Box::new(Boom) as BoxError))
        .handler(|_ctx| Ok(()));

    let error = command.run_from([] as [&str; 0]).unwrap_err();

    assert!(matches!(error, RunError::PluginSetup { .. }));
}

#[test]
fn plugin_setup_error_remains_primary_when_plugin_cleanup_fails() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let command = Command::new("root")
        .plugin(failing_setup_plugin("a", &calls))
        .plugin(failing_cleanup_plugin("b", &calls))
        .handler(|_ctx| Ok(()));

    let error = command.run_from([] as [&str; 0]).unwrap_err();

    assert!(matches!(error, RunError::PluginSetup { plugin, .. } if plugin == "a"));
}

#[test]
fn command_setup_failure_still_runs_reverse_plugin_cleanup() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let command = Command::new("root")
        .plugin(recording_plugin("a", &calls))
        .plugin(recording_plugin("b", &calls))
        .setup(|_ctx| Err(Box::new(Boom) as BoxError))
        .handler(|_ctx| Ok(()));

    command.run_from([] as [&str; 0]).unwrap_err();

    let recorded = calls.lock().unwrap();
    let cleanups: Vec<&String> = recorded
        .iter()
        .filter(|c| c.ends_with("-cleanup"))
        .collect();
    assert_eq!(cleanups, vec!["b-cleanup", "a-cleanup"]);
}

#[test]
fn command_setup_error_remains_primary_over_plugin_cleanup_errors() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let command = Command::new("root")
        .plugin(failing_cleanup_plugin("a", &calls))
        .setup(|_ctx| Err(Box::new(Boom) as BoxError))
        .handler(|_ctx| Ok(()));

    let error = command.run_from([] as [&str; 0]).unwrap_err();

    assert!(matches!(error, RunError::Setup(_)));
}

#[test]
fn handler_failure_still_runs_command_and_plugin_cleanup() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let cleanup_calls = Arc::clone(&calls);
    let command = Command::new("root")
        .plugin(recording_plugin("a", &calls))
        .cleanup(move |_ctx| {
            cleanup_calls
                .lock()
                .unwrap()
                .push("command-cleanup".to_string());
            Ok(())
        })
        .handler(|_ctx| Err::<(), _>(Box::new(Boom) as BoxError));

    command.run_from([] as [&str; 0]).unwrap_err();

    let recorded = calls.lock().unwrap();
    assert!(recorded.contains(&"command-cleanup".to_string()));
    assert!(recorded.contains(&"a-cleanup".to_string()));
}

#[test]
fn handler_error_remains_primary_with_plugins() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let command = Command::new("root")
        .plugin(failing_cleanup_plugin("a", &calls))
        .cleanup(|_ctx| Err(Box::new(Boom) as BoxError))
        .handler(|_ctx| Err::<(), _>(Box::new(Boom) as BoxError));

    let error = command.run_from([] as [&str; 0]).unwrap_err();

    assert!(matches!(error, RunError::Handler(_)));
}

#[test]
fn child_failure_still_causes_parent_plugin_cleanup() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let command = Command::new("root")
        .plugin(recording_plugin("root-a", &calls))
        .command(Command::new("build").handler(|_ctx| Err::<(), _>(Box::new(Boom) as BoxError)));

    command.run_from(["build"]).unwrap_err();

    assert!(
        calls
            .lock()
            .unwrap()
            .contains(&"root-a-cleanup".to_string())
    );
}

#[test]
fn child_failure_remains_primary_with_plugins() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let command = Command::new("root")
        .plugin(failing_cleanup_plugin("root-a", &calls))
        .command(Command::new("build").handler(|_ctx| Err::<(), _>(Box::new(Boom) as BoxError)));

    let error = command.run_from(["build"]).unwrap_err();

    assert!(matches!(error, RunError::Handler(_)));
}

#[test]
fn command_cleanup_error_takes_precedence_over_later_plugin_cleanup_failures() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let command = Command::new("root")
        .plugin(failing_cleanup_plugin("a", &calls))
        .plugin(failing_cleanup_plugin("b", &calls))
        .cleanup(|_ctx| Err(Box::new(Boom) as BoxError))
        .handler(|_ctx| Ok(()));

    let error = command.run_from([] as [&str; 0]).unwrap_err();

    assert!(matches!(error, RunError::Cleanup(_)));
    // both plugin cleanups still attempted despite command cleanup already failing
    let recorded = calls.lock().unwrap();
    assert!(recorded.contains(&"a-cleanup".to_string()));
    assert!(recorded.contains(&"b-cleanup".to_string()));
}

#[test]
fn plugin_cleanup_only_failure_becomes_typed_plugin_cleanup() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let command = Command::new("root")
        .plugin(failing_cleanup_plugin("a", &calls))
        .handler(|_ctx| Ok(()));

    let error = command.run_from([] as [&str; 0]).unwrap_err();

    assert!(matches!(error, RunError::PluginCleanup { .. }));
}

#[test]
fn first_reverse_order_plugin_cleanup_failure_wins() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let command = Command::new("root")
        .plugin(failing_cleanup_plugin("a", &calls))
        .plugin(failing_cleanup_plugin("b", &calls))
        .handler(|_ctx| Ok(()));

    let error = command.run_from([] as [&str; 0]).unwrap_err();

    assert!(matches!(error, RunError::PluginCleanup { plugin, .. } if plugin == "b"));
}

#[test]
fn later_plugin_cleanup_hooks_still_run_after_a_plugin_cleanup_error() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let command = Command::new("root")
        .plugin(failing_cleanup_plugin("a", &calls))
        .plugin(failing_cleanup_plugin("b", &calls))
        .handler(|_ctx| Ok(()));

    command.run_from([] as [&str; 0]).unwrap_err();

    let recorded = calls.lock().unwrap();
    assert!(recorded.contains(&"a-cleanup".to_string()));
    assert!(recorded.contains(&"b-cleanup".to_string()));
}

#[test]
fn plugin_setup_exposes_source() {
    let command = Command::new("root")
        .plugin(Plugin::new("a").setup(|_ctx| Err(Box::new(Boom) as BoxError)))
        .handler(|_ctx| Ok(()));

    let error = command.run_from([] as [&str; 0]).unwrap_err();

    let source = std::error::Error::source(&error).expect("plugin setup error has a source");
    assert_eq!(source.to_string(), "boom");
}

#[test]
fn plugin_cleanup_exposes_source() {
    let command = Command::new("root")
        .plugin(Plugin::new("a").cleanup(|_ctx| Err(Box::new(Boom) as BoxError)))
        .handler(|_ctx| Ok(()));

    let error = command.run_from([] as [&str; 0]).unwrap_err();

    let source = std::error::Error::source(&error).expect("plugin cleanup error has a source");
    assert_eq!(source.to_string(), "boom");
}

#[test]
fn plugin_identity_retained_in_both_typed_errors() {
    let setup_error = Command::new("root")
        .plugin(Plugin::new("logger").setup(|_ctx| Err(Box::new(Boom) as BoxError)))
        .handler(|_ctx| Ok(()))
        .run_from([] as [&str; 0])
        .unwrap_err();
    assert!(matches!(setup_error, RunError::PluginSetup { plugin, .. } if plugin == "logger"));

    let cleanup_error = Command::new("root")
        .plugin(Plugin::new("logger").cleanup(|_ctx| Err(Box::new(Boom) as BoxError)))
        .handler(|_ctx| Ok(()))
        .run_from([] as [&str; 0])
        .unwrap_err();
    assert!(matches!(cleanup_error, RunError::PluginCleanup { plugin, .. } if plugin == "logger"));
}

// --- Built-ins ---

fn assert_help(action: CliAction, expected: &str) {
    match action {
        CliAction::Help(text) => assert_eq!(text, expected),
        _ => panic!("expected CliAction::Help"),
    }
}

fn assert_version(action: CliAction, expected: &str) {
    match action {
        CliAction::Version(text) => assert_eq!(text, expected),
        _ => panic!("expected CliAction::Version"),
    }
}

#[test]
fn builtin_long_help_renders_root_usage() {
    let command = Command::new("ritty").description("desc");

    let action = command.run_cli_from(["--help"]).unwrap();

    assert_help(action, &command.render_usage());
}

#[test]
fn builtin_short_help_renders_root_usage() {
    let command = Command::new("ritty").description("desc");

    let action = command.run_cli_from(["-h"]).unwrap();

    assert_help(action, &command.render_usage());
}

#[test]
fn builtin_help_runs_zero_lifecycle_callbacks() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let handler_calls = Arc::clone(&calls);
    let setup_calls = Arc::clone(&calls);
    let cleanup_calls = Arc::clone(&calls);
    let plugin_setup_calls = Arc::clone(&calls);
    let plugin_cleanup_calls = Arc::clone(&calls);

    let command = Command::new("ritty")
        .plugin(
            Plugin::new("p")
                .setup(move |_ctx| {
                    plugin_setup_calls.lock().unwrap().push("plugin_setup");
                    Ok(())
                })
                .cleanup(move |_ctx| {
                    plugin_cleanup_calls.lock().unwrap().push("plugin_cleanup");
                    Ok(())
                }),
        )
        .setup(move |_ctx| {
            setup_calls.lock().unwrap().push("setup");
            Ok(())
        })
        .cleanup(move |_ctx| {
            cleanup_calls.lock().unwrap().push("cleanup");
            Ok(())
        })
        .handler(move |_ctx| {
            handler_calls.lock().unwrap().push("handler");
            Ok(())
        });

    command.run_cli_from(["--help"]).unwrap();

    assert!(calls.lock().unwrap().is_empty());
}

#[test]
fn builtin_version_runs_zero_lifecycle_callbacks() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let handler_calls = Arc::clone(&calls);
    let setup_calls = Arc::clone(&calls);

    let command = Command::new("ritty")
        .version("1.0.0")
        .setup(move |_ctx| {
            setup_calls.lock().unwrap().push("setup");
            Ok(())
        })
        .handler(move |_ctx| {
            handler_calls.lock().unwrap().push("handler");
            Ok(())
        });

    command.run_cli_from(["--version"]).unwrap();

    assert!(calls.lock().unwrap().is_empty());
}

#[test]
fn builtin_nested_help_renders_qualified_child_usage() {
    let add = Command::new("add");
    let remote = Command::new("remote").command(add);
    let command = Command::new("ritty").command(remote);

    let action = command.run_cli_from(["remote", "add", "--help"]).unwrap();

    let expected = command
        .subcommands
        .iter()
        .find(|c| c.name() == "remote")
        .unwrap()
        .subcommands
        .iter()
        .find(|c| c.name() == "add")
        .unwrap()
        .render_usage_named("ritty remote add", None);
    assert_help(action, &expected);
}

#[test]
fn builtin_deeply_nested_help_renders_qualified_usage() {
    let leaf = Command::new("leaf");
    let mid = Command::new("mid").command(leaf);
    let top = Command::new("top").command(mid);
    let command = Command::new("ritty").command(top);

    let action = command
        .run_cli_from(["top", "mid", "leaf", "--help"])
        .unwrap();

    match action {
        CliAction::Help(text) => assert!(text.contains("ritty top mid leaf")),
        _ => panic!("expected CliAction::Help"),
    }
}

#[test]
fn builtin_subcommand_alias_help_renders_canonical_path() {
    let command = Command::new("ritty").command(Command::new("install").alias("i"));

    let action = command.run_cli_from(["i", "--help"]).unwrap();

    match action {
        CliAction::Help(text) => assert!(text.contains("ritty install")),
        _ => panic!("expected CliAction::Help"),
    }
}

#[test]
fn builtin_hidden_subcommand_help_still_resolves() {
    let command = Command::new("ritty").command(Command::new("secret").hidden());

    let action = command.run_cli_from(["secret", "--help"]).unwrap();

    match action {
        CliAction::Help(text) => assert!(text.contains("ritty secret")),
        _ => panic!("expected CliAction::Help"),
    }
}

#[test]
fn builtin_help_qualified_display_name_matches_render_usage_named() {
    let command = Command::new("ritty").command(Command::new("remote"));

    let action = command.run_cli_from(["remote", "--help"]).unwrap();

    let remote = command
        .subcommands
        .iter()
        .find(|c| c.name() == "remote")
        .unwrap();
    assert_help(action, &remote.render_usage_named("ritty remote", None));
}

#[test]
fn builtin_help_skips_parent_string_option_value_before_child() {
    let command = Command::new("ritty")
        .option(StringOption::new("config"))
        .command(Command::new("remote"));

    let action = command
        .run_cli_from(["--config", "production", "remote", "--help"])
        .unwrap();

    match action {
        CliAction::Help(text) => assert!(text.contains("ritty remote")),
        _ => panic!("expected CliAction::Help"),
    }
}

#[test]
fn builtin_help_skips_parent_enum_option_value_before_child() {
    let command = Command::new("ritty")
        .enum_option(EnumOption::new("mode", ["a", "b"]))
        .command(Command::new("remote"));

    let action = command
        .run_cli_from(["--mode", "remote", "remote", "--help"])
        .unwrap();

    match action {
        CliAction::Help(text) => assert!(text.contains("ritty remote")),
        _ => panic!("expected CliAction::Help"),
    }
}

#[test]
fn builtin_help_skips_short_value_bearing_option_before_child() {
    let command = Command::new("ritty")
        .option(StringOption::new("config").alias("c"))
        .command(Command::new("remote"));

    let action = command
        .run_cli_from(["-c", "remote", "remote", "--help"])
        .unwrap();

    match action {
        CliAction::Help(text) => assert!(text.contains("ritty remote")),
        _ => panic!("expected CliAction::Help"),
    }
}

#[test]
fn builtin_help_skips_long_equals_value_before_child() {
    let command = Command::new("ritty")
        .option(StringOption::new("config"))
        .command(Command::new("remote"));

    let action = command
        .run_cli_from(["--config=remote", "remote", "--help"])
        .unwrap();

    match action {
        CliAction::Help(text) => assert!(text.contains("ritty remote")),
        _ => panic!("expected CliAction::Help"),
    }
}

#[test]
fn builtin_help_skips_short_equals_value_before_child() {
    let command = Command::new("ritty")
        .option(StringOption::new("config").alias("c"))
        .command(Command::new("remote"));

    let action = command
        .run_cli_from(["-c=remote", "remote", "--help"])
        .unwrap();

    match action {
        CliAction::Help(text) => assert!(text.contains("ritty remote")),
        _ => panic!("expected CliAction::Help"),
    }
}

// --- Help conflicts ---

#[test]
fn user_defined_canonical_help_disables_builtin_entirely() {
    let command = Command::new("ritty").flag(Flag::new("help"));

    let action = command.run_cli_from(["--help"]).unwrap();
    match action {
        CliAction::Ran => {}
        _ => panic!("expected --help to parse as the user's own flag"),
    }

    let error = command.run_cli_from(["-h"]).unwrap_err();
    assert!(matches!(error, RunError::Parse(_)));
}

#[test]
fn user_defined_long_alias_help_disables_builtin_entirely() {
    let command = Command::new("ritty").flag(Flag::new("assist").alias("help"));

    let action = command.run_cli_from(["--help"]).unwrap();
    assert!(matches!(action, CliAction::Ran));

    let error = command.run_cli_from(["-h"]).unwrap_err();
    assert!(matches!(error, RunError::Parse(_)));
}

#[test]
fn user_owned_short_h_disables_only_short_builtin() {
    let command = Command::new("ritty").flag(Flag::new("host").short('h'));

    let action = command.run_cli_from(["-h"]).unwrap();
    assert!(matches!(action, CliAction::Ran));

    let action = command.run_cli_from(["--help"]).unwrap();
    assert_help(action, &command.render_usage());
}

#[test]
fn help_remains_when_only_short_conflicts() {
    let command = Command::new("ritty").flag(Flag::new("host").short('h'));

    let action = command.run_cli_from(["--help"]).unwrap();

    assert_help(action, &command.render_usage());
}

#[test]
fn normal_execution_occurs_for_user_owned_help_spelling() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let recorded = Arc::clone(&calls);
    let command = Command::new("ritty")
        .flag(Flag::new("host").short('h'))
        .handler(move |ctx| {
            recorded.lock().unwrap().push(ctx.matches().flag("host"));
            Ok(())
        });

    let action = command.run_cli_from(["-h"]).unwrap();

    assert!(matches!(action, CliAction::Ran));
    assert_eq!(*calls.lock().unwrap(), vec![true]);
}

// --- Version ---

#[test]
fn builtin_long_version_prints_exact_version() {
    let command = Command::new("ritty").version("1.2.3");

    let action = command.run_cli_from(["--version"]).unwrap();

    assert_version(action, "1.2.3");
}

#[test]
fn builtin_short_version_prints_exact_version() {
    let command = Command::new("ritty").version("1.2.3");

    let action = command.run_cli_from(["-v"]).unwrap();

    assert_version(action, "1.2.3");
}

#[test]
fn missing_version_is_no_version_error() {
    let command = Command::new("ritty");

    let error = command.run_cli_from(["--version"]).unwrap_err();

    assert!(matches!(error, RunError::NoVersion));
}

#[test]
fn no_version_display_and_source() {
    let error = RunError::NoVersion;

    assert_eq!(error.to_string(), "no version specified");
    assert!(std::error::Error::source(&error).is_none());
}

#[test]
fn user_defined_canonical_version_disables_builtin_entirely() {
    let command = Command::new("ritty")
        .version("1.2.3")
        .flag(Flag::new("version"));

    let action = command.run_cli_from(["--version"]).unwrap();
    assert!(matches!(action, CliAction::Ran));

    let error = command.run_cli_from(["-v"]).unwrap_err();
    assert!(matches!(error, RunError::Parse(_)));
}

#[test]
fn user_defined_long_alias_version_disables_builtin_entirely() {
    let command = Command::new("ritty")
        .version("1.2.3")
        .flag(Flag::new("verbose").alias("version"));

    let action = command.run_cli_from(["--version"]).unwrap();
    assert!(matches!(action, CliAction::Ran));

    let error = command.run_cli_from(["-v"]).unwrap_err();
    assert!(matches!(error, RunError::Parse(_)));
}

#[test]
fn user_owned_short_v_disables_only_short_builtin() {
    let command = Command::new("ritty")
        .version("1.2.3")
        .flag(Flag::new("verbose").short('v'));

    let action = command.run_cli_from(["-v"]).unwrap();
    assert!(matches!(action, CliAction::Ran));

    let action = command.run_cli_from(["--version"]).unwrap();
    assert_version(action, "1.2.3");
}

#[test]
fn version_remains_when_only_short_conflicts() {
    let command = Command::new("ritty")
        .version("1.2.3")
        .flag(Flag::new("verbose").short('v'));

    let action = command.run_cli_from(["--version"]).unwrap();

    assert_version(action, "1.2.3");
}

#[test]
fn version_requires_exactly_one_token_extra_trailing() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let recorded = Arc::clone(&calls);
    let command = Command::new("ritty")
        .version("1.2.3")
        .flag(Flag::new("version"))
        .arg(Arg::new("target"))
        .handler(move |ctx| {
            recorded.lock().unwrap().push((
                ctx.matches().flag("version"),
                ctx.matches().argument("target").map(str::to_owned),
            ));
            Ok(())
        });

    // With a user-declared "version" flag, ["--version", "extra"] must
    // parse and execute ordinarily rather than being intercepted as the
    // one-token builtin — proving the fallthrough is real execution, not
    // just a different CliAction.
    let action = command.run_cli_from(["--version", "extra"]).unwrap();

    assert!(matches!(action, CliAction::Ran));
    assert_eq!(
        *calls.lock().unwrap(),
        vec![(true, Some("extra".to_string()))]
    );
}

#[test]
fn version_requires_exactly_one_token_leading_extra() {
    let command = Command::new("ritty").version("1.2.3");

    // "extra" is not declared, so ordinary parsing must reject it as an
    // unexpected positional rather than the dispatcher silently treating
    // this as a builtin version request.
    let error = command.run_cli_from(["extra", "--version"]).unwrap_err();

    assert!(matches!(error, RunError::Parse(_)));
}

#[test]
fn version_requires_exactly_one_token_short_with_extra() {
    let command = Command::new("ritty").version("1.2.3");

    let error = command.run_cli_from(["-v", "extra"]).unwrap_err();

    assert!(matches!(error, RunError::Parse(_)));
}

#[test]
fn nested_long_version_is_not_automatic() {
    let command = Command::new("ritty")
        .version("1.0.0")
        .command(Command::new("remote"));

    let error = command.run_cli_from(["remote", "--version"]).unwrap_err();

    assert!(matches!(error, RunError::Parse(_)));
}

#[test]
fn nested_short_version_is_not_automatic() {
    let command = Command::new("ritty")
        .version("1.0.0")
        .command(Command::new("remote"));

    let error = command.run_cli_from(["remote", "-v"]).unwrap_err();

    assert!(matches!(error, RunError::Parse(_)));
}

#[test]
fn child_own_version_metadata_does_not_enable_nested_automatic_version() {
    let command = Command::new("ritty")
        .version("1.0.0")
        .command(Command::new("remote").version("2.0.0"));

    let error = command.run_cli_from(["remote", "--version"]).unwrap_err();

    assert!(matches!(error, RunError::Parse(_)));
}

#[test]
fn child_can_declare_its_own_version_option() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let recorded = Arc::clone(&calls);
    let command = Command::new("ritty").version("1.0.0").command(
        Command::new("remote")
            .flag(Flag::new("version"))
            .handler(move |ctx| {
                recorded.lock().unwrap().push(ctx.matches().flag("version"));
                Ok(())
            }),
    );

    let action = command.run_cli_from(["remote", "--version"]).unwrap();

    assert!(matches!(action, CliAction::Ran));
    assert_eq!(*calls.lock().unwrap(), vec![true]);
}

#[test]
fn child_can_declare_its_own_short_v_option() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let recorded = Arc::clone(&calls);
    let command = Command::new("ritty").version("1.0.0").command(
        Command::new("remote")
            .flag(Flag::new("verbose").short('v'))
            .handler(move |ctx| {
                recorded.lock().unwrap().push(ctx.matches().flag("verbose"));
                Ok(())
            }),
    );

    let action = command.run_cli_from(["remote", "-v"]).unwrap();

    assert!(matches!(action, CliAction::Ran));
    assert_eq!(*calls.lock().unwrap(), vec![true]);
}

#[test]
fn help_wins_over_version_when_version_first() {
    let command = Command::new("ritty").version("1.2.3");

    let action = command.run_cli_from(["--version", "--help"]).unwrap();

    assert_help(action, &command.render_usage());
}

#[test]
fn help_wins_over_version_when_help_first() {
    let command = Command::new("ritty").version("1.2.3");

    let action = command.run_cli_from(["--help", "--version"]).unwrap();

    assert_help(action, &command.render_usage());
}

#[test]
fn short_help_wins_over_short_version_when_version_first() {
    let command = Command::new("ritty").version("1.2.3");

    let action = command.run_cli_from(["-v", "-h"]).unwrap();

    assert_help(action, &command.render_usage());
}

#[test]
fn short_help_wins_over_short_version_when_help_first() {
    let command = Command::new("ritty").version("1.2.3");

    let action = command.run_cli_from(["-h", "-v"]).unwrap();

    assert_help(action, &command.render_usage());
}

#[test]
fn root_child_version_extra_token_falls_through_to_no_command() {
    let command = Command::new("ritty").version("1.0.0");

    let error = command.run_cli_from(["--version", "extra"]).unwrap_err();

    assert!(matches!(error, RunError::Parse(_)));
}

// --- API separation ---

#[test]
fn parse_from_help_remains_literal() {
    let command = Command::new("ritty");

    let error = command.parse_from(["--help"]).unwrap_err();

    assert_eq!(
        error.kind(),
        ParseErrorKind::Argument(ArgumentErrorKind::UnknownOption)
    );
}

#[test]
fn run_from_help_remains_literal() {
    let command = Command::new("ritty").handler(|_ctx| Ok(()));

    let error = command.run_from(["--help"]).unwrap_err();

    assert!(matches!(error, RunError::Parse(_)));
}

#[test]
fn parse_from_version_remains_literal() {
    let command = Command::new("ritty").version("1.0.0");

    let error = command.parse_from(["--version"]).unwrap_err();

    assert_eq!(
        error.kind(),
        ParseErrorKind::Argument(ArgumentErrorKind::UnknownOption)
    );
}

#[test]
fn run_from_version_remains_literal() {
    let command = Command::new("ritty")
        .version("1.0.0")
        .handler(|_ctx| Ok(()));

    let error = command.run_from(["--version"]).unwrap_err();

    assert!(matches!(error, RunError::Parse(_)));
}

#[test]
fn cli_dispatcher_recognizes_builtins() {
    let command = Command::new("ritty").version("1.0.0");

    assert!(matches!(
        command.run_cli_from(["--help"]).unwrap(),
        CliAction::Help(_)
    ));
    assert!(matches!(
        command.run_cli_from(["--version"]).unwrap(),
        CliAction::Version(_)
    ));
}

// --- Regression: normal CLI execution ---

#[test]
fn cli_dispatcher_runs_explicit_subcommand_handler() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let recorded = Arc::clone(&calls);
    let command = Command::new("ritty").command(Command::new("build").handler(move |_ctx| {
        recorded.lock().unwrap().push("build");
        Ok(())
    }));

    let action = command.run_cli_from(["build"]).unwrap();

    assert!(matches!(action, CliAction::Ran));
    assert_eq!(*calls.lock().unwrap(), vec!["build"]);
}

#[test]
fn cli_dispatcher_runs_alias_subcommand_handler() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let recorded = Arc::clone(&calls);
    let command =
        Command::new("ritty").command(Command::new("install").alias("i").handler(move |_ctx| {
            recorded.lock().unwrap().push("install");
            Ok(())
        }));

    let action = command.run_cli_from(["i"]).unwrap();

    assert!(matches!(action, CliAction::Ran));
    assert_eq!(*calls.lock().unwrap(), vec!["install"]);
}

#[test]
fn cli_dispatcher_runs_default_subcommand_handler() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let recorded = Arc::clone(&calls);
    let command =
        Command::new("ritty")
            .default_subcommand("build")
            .command(Command::new("build").handler(move |_ctx| {
                recorded.lock().unwrap().push("build");
                Ok(())
            }));

    let action = command.run_cli_from([] as [&str; 0]).unwrap();

    assert!(matches!(action, CliAction::Ran));
    assert_eq!(*calls.lock().unwrap(), vec!["build"]);
}

#[test]
fn cli_dispatcher_runs_hidden_subcommand_handler() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let recorded = Arc::clone(&calls);
    let command =
        Command::new("ritty").command(Command::new("secret").hidden().handler(move |_ctx| {
            recorded.lock().unwrap().push("secret");
            Ok(())
        }));

    let action = command.run_cli_from(["secret"]).unwrap();

    assert!(matches!(action, CliAction::Ran));
    assert_eq!(*calls.lock().unwrap(), vec!["secret"]);
}

#[test]
fn cli_dispatcher_runs_setup_and_cleanup() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let setup_calls = Arc::clone(&calls);
    let cleanup_calls = Arc::clone(&calls);
    let command = Command::new("ritty")
        .setup(move |_ctx| {
            setup_calls.lock().unwrap().push("setup");
            Ok(())
        })
        .cleanup(move |_ctx| {
            cleanup_calls.lock().unwrap().push("cleanup");
            Ok(())
        })
        .handler(|_ctx| Ok(()));

    command.run_cli_from([] as [&str; 0]).unwrap();

    assert_eq!(*calls.lock().unwrap(), vec!["setup", "cleanup"]);
}

#[test]
fn cli_dispatcher_runs_plugins() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let setup_calls = Arc::clone(&calls);
    let command = Command::new("ritty")
        .plugin(Plugin::new("p").setup(move |_ctx| {
            setup_calls.lock().unwrap().push("plugin_setup");
            Ok(())
        }))
        .handler(|_ctx| Ok(()));

    command.run_cli_from([] as [&str; 0]).unwrap();

    assert_eq!(*calls.lock().unwrap(), vec!["plugin_setup"]);
}

#[test]
fn cli_dispatcher_surfaces_parse_errors() {
    let command = Command::new("ritty");

    let error = command.run_cli_from(["--bogus"]).unwrap_err();

    assert!(matches!(error, RunError::Parse(_)));
}

#[derive(PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

// Deliberately not Debug/Clone/Send/Sync, to prove CommandOutput
// imposes none of those bounds on the contained value.
struct NotDebugCloneSendSync(std::rc::Rc<std::cell::Cell<i32>>);

#[test]
fn command_output_unit() {
    let output = CommandOutput::new(());

    assert_eq!(output.downcast::<()>().unwrap(), ());
}

#[test]
fn command_output_usize() {
    let output = CommandOutput::new(42usize);

    assert_eq!(output.downcast::<usize>().unwrap(), 42);
}

#[test]
fn command_output_string() {
    let output = CommandOutput::new(String::from("done"));

    assert_eq!(output.downcast::<String>().unwrap(), "done");
}

#[test]
fn command_output_custom_struct() {
    let output = CommandOutput::new(Point { x: 1, y: 2 });

    assert_eq!(output.downcast::<Point>().unwrap(), Point { x: 1, y: 2 });
}

#[test]
fn command_output_is_reports_contained_type() {
    let output = CommandOutput::new(42usize);

    assert!(output.is::<usize>());
    assert!(!output.is::<String>());
}

#[test]
fn command_output_downcast_ref_reads_without_consuming() {
    let output = CommandOutput::new(String::from("done"));

    assert_eq!(output.downcast_ref::<String>().unwrap(), "done");
    assert_eq!(output.downcast::<String>().unwrap(), "done");
}

#[test]
fn command_output_failed_downcast_preserves_output() {
    let output = CommandOutput::new(42usize);

    let output = output.downcast::<String>().unwrap_err();

    assert_eq!(output.downcast::<usize>().unwrap(), 42);
}

#[test]
fn command_output_type_name() {
    let output = CommandOutput::new(42usize);

    assert_eq!(output.type_name(), std::any::type_name::<usize>());
}

#[test]
fn command_output_debug_does_not_require_inner_debug() {
    let cell = std::rc::Rc::new(std::cell::Cell::new(7));
    let output = CommandOutput::new(NotDebugCloneSendSync(cell));

    let rendered = format!("{output:?}");

    assert!(rendered.contains("CommandOutput"));
    assert!(rendered.contains("type_name"));
    let value = output.downcast::<NotDebugCloneSendSync>().unwrap();
    assert_eq!(value.0.get(), 7);
}

#[test]
fn root_handler_result_is_returned() {
    let command = Command::new("root").handler(|_ctx| Ok(42usize));

    let output = command.run_from([] as [&str; 0]).unwrap();

    assert_eq!(output.downcast::<usize>().unwrap(), 42);
}

#[test]
fn captured_closure_handler_returns_value() {
    let prefix = String::from("hello, ");
    let command =
        Command::new("root").handler(move |_ctx| Ok::<_, BoxError>(format!("{prefix}world")));

    let output = command.run_from([] as [&str; 0]).unwrap();

    assert_eq!(output.downcast::<String>().unwrap(), "hello, world");
}

#[test]
fn cloned_command_retains_result_producing_handler() {
    let command = Command::new("root").handler(|_ctx| Ok(42usize));
    let cloned = command.clone();

    let output = cloned.run_from([] as [&str; 0]).unwrap();

    assert_eq!(output.downcast::<usize>().unwrap(), 42);
}

#[test]
fn explicit_child_result_propagates() {
    let command = Command::new("root").command(Command::new("child").handler(|_ctx| Ok(42usize)));

    let output = command.run_from(["child"]).unwrap();

    assert_eq!(output.downcast::<usize>().unwrap(), 42);
}

#[test]
fn aliased_child_result_propagates() {
    let command =
        Command::new("root").command(Command::new("child").alias("c").handler(|_ctx| Ok(42usize)));

    let output = command.run_from(["c"]).unwrap();

    assert_eq!(output.downcast::<usize>().unwrap(), 42);
}

#[test]
fn default_child_result_propagates() {
    let command = Command::new("root")
        .default_subcommand("child")
        .command(Command::new("child").handler(|_ctx| Ok(42usize)));

    let output = command.run_from([] as [&str; 0]).unwrap();

    assert_eq!(output.downcast::<usize>().unwrap(), 42);
}

#[test]
fn hidden_child_result_propagates() {
    let command =
        Command::new("root").command(Command::new("secret").hidden().handler(|_ctx| Ok(42usize)));

    let output = command.run_from(["secret"]).unwrap();

    assert_eq!(output.downcast::<usize>().unwrap(), 42);
}

#[test]
fn deep_nested_child_result_propagates() {
    let command = Command::new("root")
        .command(Command::new("mid").command(Command::new("leaf").handler(|_ctx| Ok(42usize))));

    let output = command.run_from(["mid", "leaf"]).unwrap();

    assert_eq!(output.downcast::<usize>().unwrap(), 42);
}

#[test]
fn empty_no_handler_command_returns_unit_output() {
    let command = Command::new("root");

    let output = command.run_from([] as [&str; 0]).unwrap();

    assert_eq!(output.downcast::<()>().unwrap(), ());
}

#[test]
fn handlerless_selected_leaf_returns_unit_output() {
    let command = Command::new("root").command(Command::new("child"));

    let output = command.run_from(["child"]).unwrap();

    assert_eq!(output.downcast::<()>().unwrap(), ());
}

#[test]
fn typed_handler_error_still_run_error_handler() {
    let command = Command::new("root").handler(|_ctx| Err::<usize, _>(Box::new(Boom) as BoxError));

    let error = command.run_from([] as [&str; 0]).unwrap_err();

    assert!(matches!(error, RunError::Handler(_)));
}

#[test]
fn setup_failure_produces_no_successful_output() {
    let command = Command::new("root")
        .setup(|_ctx| Err(Box::new(Boom) as BoxError))
        .handler(|_ctx| Ok(42usize));

    let error = command.run_from([] as [&str; 0]).unwrap_err();

    assert!(matches!(error, RunError::Setup(_)));
}

#[test]
fn plugin_setup_failure_produces_no_successful_output() {
    let command = Command::new("root")
        .plugin(Plugin::new("p").setup(|_ctx| Err(Box::new(Boom) as BoxError)))
        .handler(|_ctx| Ok(42usize));

    let error = command.run_from([] as [&str; 0]).unwrap_err();

    assert!(matches!(error, RunError::PluginSetup { .. }));
}

#[test]
fn command_cleanup_failure_overrides_successful_output() {
    let command = Command::new("root")
        .handler(|_ctx| Ok(42usize))
        .cleanup(|_ctx| Err(Box::new(Boom) as BoxError));

    let error = command.run_from([] as [&str; 0]).unwrap_err();

    assert!(matches!(error, RunError::Cleanup(_)));
}

#[test]
fn plugin_cleanup_failure_overrides_successful_output() {
    let command = Command::new("root")
        .plugin(Plugin::new("p").cleanup(|_ctx| Err(Box::new(Boom) as BoxError)))
        .handler(|_ctx| Ok(42usize));

    let error = command.run_from([] as [&str; 0]).unwrap_err();

    assert!(matches!(error, RunError::PluginCleanup { .. }));
}

#[test]
fn nested_parent_cleanup_failure_overrides_leaf_output() {
    let command = Command::new("root")
        .cleanup(|_ctx| Err(Box::new(Boom) as BoxError))
        .command(Command::new("child").handler(|_ctx| Ok(42usize)));

    let error = command.run_from(["child"]).unwrap_err();

    assert!(matches!(error, RunError::Cleanup(_)));
}

#[test]
fn cli_dispatcher_discards_handler_output() {
    let command = Command::new("ritty").handler(|_ctx| Ok(42usize));

    let action = command.run_cli_from([] as [&str; 0]).unwrap();

    assert!(matches!(action, CliAction::Ran));
}

// --- Lazy subcommands ---

use std::sync::atomic::{AtomicUsize, Ordering};

#[test]
fn lazy_command_construction_does_not_invoke_loader() {
    let loads = Arc::new(AtomicUsize::new(0));
    let counter = loads.clone();

    let _command = Command::new("root").lazy_command("build", move || {
        counter.fetch_add(1, Ordering::SeqCst);
        Command::new("build")
    });

    assert_eq!(loads.load(Ordering::SeqCst), 0);
}

#[test]
fn cloning_lazy_command_does_not_invoke_loader() {
    let loads = Arc::new(AtomicUsize::new(0));
    let counter = loads.clone();

    let command = Command::new("root").lazy_command("build", move || {
        counter.fetch_add(1, Ordering::SeqCst);
        Command::new("build")
    });
    let _clone = command.clone();

    assert_eq!(loads.load(Ordering::SeqCst), 0);
}

#[test]
fn selecting_eager_sibling_does_not_resolve_lazy_siblings() {
    let loads = Arc::new(AtomicUsize::new(0));
    let counter = loads.clone();

    let command = Command::new("root")
        .command(Command::new("build").handler(|_ctx| Ok(())))
        .lazy_command("test", move || {
            counter.fetch_add(1, Ordering::SeqCst);
            Command::new("test").handler(|_ctx| Ok(()))
        });

    command.run_from(["build"]).unwrap();

    assert_eq!(loads.load(Ordering::SeqCst), 0);
}

#[test]
fn canonical_lazy_selection_invokes_loader_exactly_once() {
    let loads = Arc::new(AtomicUsize::new(0));
    let counter = loads.clone();

    let command = Command::new("root").lazy_command("build", move || {
        counter.fetch_add(1, Ordering::SeqCst);
        Command::new("build").handler(|_ctx| Ok(()))
    });

    command.run_from(["build"]).unwrap();

    assert_eq!(loads.load(Ordering::SeqCst), 1);
}

#[test]
fn parse_and_execute_share_one_resolution() {
    let loads = Arc::new(AtomicUsize::new(0));
    let counter = loads.clone();

    let command = Command::new("root").lazy_command("build", move || {
        counter.fetch_add(1, Ordering::SeqCst);
        Command::new("build").handler(|_ctx| Ok(42usize))
    });

    let matches = command.parse_from(["build"]).unwrap();
    assert_eq!(loads.load(Ordering::SeqCst), 1);

    let output = command.execute(&matches, &matches).unwrap();
    assert_eq!(output.downcast::<usize>().unwrap(), 42);
    assert_eq!(loads.load(Ordering::SeqCst), 1);
}

#[test]
fn repeated_parsing_of_same_tree_does_not_reinvoke_loader() {
    let loads = Arc::new(AtomicUsize::new(0));
    let counter = loads.clone();

    let command = Command::new("root").lazy_command("build", move || {
        counter.fetch_add(1, Ordering::SeqCst);
        Command::new("build").handler(|_ctx| Ok(()))
    });

    command.parse_from(["build"]).unwrap();
    command.parse_from(["build"]).unwrap();
    command.run_from(["build"]).unwrap();

    assert_eq!(loads.load(Ordering::SeqCst), 1);
}

#[test]
fn cloned_command_shares_lazy_resolution_cache() {
    let loads = Arc::new(AtomicUsize::new(0));
    let counter = loads.clone();

    let command = Command::new("root").lazy_command("build", move || {
        counter.fetch_add(1, Ordering::SeqCst);
        Command::new("build").handler(|_ctx| Ok(()))
    });

    let clone = command.clone();
    clone.run_from(["build"]).unwrap();
    assert_eq!(loads.load(Ordering::SeqCst), 1);

    command.run_from(["build"]).unwrap();
    assert_eq!(loads.load(Ordering::SeqCst), 1);
}

#[test]
fn usage_resolution_of_lazy_child_is_cached_for_later_execution() {
    let loads = Arc::new(AtomicUsize::new(0));
    let counter = loads.clone();

    let command = Command::new("root").lazy_command("build", move || {
        counter.fetch_add(1, Ordering::SeqCst);
        Command::new("build").handler(|_ctx| Ok(()))
    });

    let _usage = command.render_usage();
    assert_eq!(loads.load(Ordering::SeqCst), 1);

    command.run_from(["build"]).unwrap();
    assert_eq!(loads.load(Ordering::SeqCst), 1);
}

#[test]
fn lazy_handler_output_propagates_through_command_output() {
    let command = Command::new("root").lazy_command("build", || {
        Command::new("build").handler(|_ctx| Ok(42usize))
    });

    let output = command.run_from(["build"]).unwrap();

    assert_eq!(output.downcast::<usize>().unwrap(), 42);
}

#[test]
fn lazy_handler_failure_becomes_run_error_handler() {
    let command = Command::new("root").lazy_command("build", || {
        Command::new("build").handler(|_ctx| Err::<(), _>(Box::new(Boom) as BoxError))
    });

    let error = command.run_from(["build"]).unwrap_err();

    assert!(matches!(error, RunError::Handler(_)));
}

#[test]
fn lazy_child_argument_parsing_matches_eager_behavior() {
    let command = Command::new("root").lazy_command("greet", || {
        Command::new("greet")
            .arg(Arg::new("name").required())
            .handler(|ctx| Ok(ctx.matches().argument("name").unwrap().to_owned()))
    });

    let output = command.run_from(["greet", "world"]).unwrap();

    assert_eq!(output.downcast::<String>().unwrap(), "world");

    let error = command.run_from(["greet"]).unwrap_err();
    assert!(matches!(error, RunError::Parse(_)));
}

#[test]
fn lazy_child_boolean_equals_value_works() {
    let command = Command::new("root").lazy_command("build", || {
        Command::new("build")
            .flag(Flag::new("release"))
            .handler(|ctx| Ok(ctx.matches().flag("release")))
    });

    let output = command.run_from(["build", "--release=true"]).unwrap();

    assert!(output.downcast::<bool>().unwrap());
}

#[test]
fn lazy_child_string_and_enum_options_work() {
    let command = Command::new("root").lazy_command("build", || {
        Command::new("build")
            .option(StringOption::new("target"))
            .enum_option(EnumOption::new("mode", ["debug", "release"]))
            .handler(|ctx| {
                Ok((
                    ctx.matches().option("target").unwrap().to_owned(),
                    ctx.matches().enum_option("mode").unwrap().to_owned(),
                ))
            })
    });

    let output = command
        .run_from(["build", "--target", "wasm", "--mode", "release"])
        .unwrap();

    assert_eq!(
        output.downcast::<(String, String)>().unwrap(),
        ("wasm".to_owned(), "release".to_owned())
    );
}

#[test]
fn lazy_declared_name_overrides_loaders_returned_name() {
    let command = Command::new("root")
        .lazy_command("build", || Command::new("compile").handler(|_ctx| Ok(())));

    let matches = command.parse_from(["build"]).unwrap();

    assert_eq!(matches.subcommand(), Some("build"));
}

#[test]
fn lazy_subcommand_selected_by_alias_canonicalizes() {
    let command = Command::new("root").lazy_command("install", || {
        Command::new("install").alias("i").handler(|_ctx| Ok(()))
    });

    let matches = command.parse_from(["i"]).unwrap();

    assert_eq!(matches.subcommand(), Some("install"));
}

#[test]
fn eager_alias_collides_with_lazy_canonical_name_is_ambiguous() {
    let command = Command::new("root")
        .command(Command::new("install").alias("build"))
        .lazy_command("build", || Command::new("build"));

    let error = command.parse_from(["build"]).unwrap_err();

    assert_eq!(error.kind(), ParseErrorKind::AmbiguousCommand);
}

#[test]
fn two_lazy_siblings_with_the_same_canonical_name_are_ambiguous() {
    let command = Command::new("root")
        .lazy_command("build", || Command::new("build"))
        .lazy_command("build", || Command::new("build"));

    let error = command.parse_from(["build"]).unwrap_err();

    assert_eq!(error.kind(), ParseErrorKind::AmbiguousCommand);
}

#[test]
fn lazy_canonical_name_colliding_with_eager_canonical_name_is_ambiguous() {
    let command = Command::new("root")
        .command(Command::new("build"))
        .lazy_command("build", || Command::new("build"));

    let error = command.parse_from(["build"]).unwrap_err();

    assert_eq!(error.kind(), ParseErrorKind::AmbiguousCommand);
}

#[test]
fn nested_lazy_subcommands_resolve_only_the_selected_path() {
    let remote_loads = Arc::new(AtomicUsize::new(0));
    let add_loads = Arc::new(AtomicUsize::new(0));
    let remote_counter = remote_loads.clone();
    let add_counter_outer = add_loads.clone();

    let command = Command::new("root").lazy_command("remote", move || {
        remote_counter.fetch_add(1, Ordering::SeqCst);
        let add_counter = add_counter_outer.clone();
        Command::new("remote").lazy_command("add", move || {
            add_counter.fetch_add(1, Ordering::SeqCst);
            Command::new("add").handler(|_ctx| Ok(()))
        })
    });

    command.run_from(["remote", "add"]).unwrap();

    assert_eq!(remote_loads.load(Ordering::SeqCst), 1);
    assert_eq!(add_loads.load(Ordering::SeqCst), 1);
}

#[test]
fn eager_parent_with_lazy_child_resolves_only_when_selected() {
    let loads = Arc::new(AtomicUsize::new(0));
    let counter = loads.clone();

    let command =
        Command::new("root").command(Command::new("remote").lazy_command("add", move || {
            counter.fetch_add(1, Ordering::SeqCst);
            Command::new("add").handler(|_ctx| Ok(()))
        }));

    command.run_from(["remote", "add"]).unwrap();

    assert_eq!(loads.load(Ordering::SeqCst), 1);
}

#[test]
fn lazy_parent_with_eager_child_resolves_parent_once() {
    let loads = Arc::new(AtomicUsize::new(0));
    let counter = loads.clone();

    let command = Command::new("root").lazy_command("remote", move || {
        counter.fetch_add(1, Ordering::SeqCst);
        Command::new("remote").command(Command::new("add").handler(|_ctx| Ok(())))
    });

    command.run_from(["remote", "add"]).unwrap();

    assert_eq!(loads.load(Ordering::SeqCst), 1);
}

#[test]
fn selecting_build_does_not_invoke_unrelated_test_or_remote_add_loaders() {
    let test_loads = Arc::new(AtomicUsize::new(0));
    let add_loads = Arc::new(AtomicUsize::new(0));
    let test_counter = test_loads.clone();
    let add_counter = add_loads.clone();

    let command = Command::new("root")
        .command(Command::new("build").handler(|_ctx| Ok(())))
        .lazy_command("test", move || {
            test_counter.fetch_add(1, Ordering::SeqCst);
            Command::new("test").handler(|_ctx| Ok(()))
        })
        .command(Command::new("remote").lazy_command("add", move || {
            add_counter.fetch_add(1, Ordering::SeqCst);
            Command::new("add").handler(|_ctx| Ok(()))
        }));

    command.run_from(["build"]).unwrap();

    assert_eq!(test_loads.load(Ordering::SeqCst), 0);
    assert_eq!(add_loads.load(Ordering::SeqCst), 0);
}

#[test]
fn selecting_remote_add_does_not_invoke_unrelated_test_loader() {
    let test_loads = Arc::new(AtomicUsize::new(0));
    let test_counter = test_loads.clone();

    let command = Command::new("root")
        .command(Command::new("build").handler(|_ctx| Ok(())))
        .lazy_command("test", move || {
            test_counter.fetch_add(1, Ordering::SeqCst);
            Command::new("test").handler(|_ctx| Ok(()))
        })
        .command(
            Command::new("remote")
                .lazy_command("add", || Command::new("add").handler(|_ctx| Ok(()))),
        );

    command.run_from(["remote", "add"]).unwrap();

    assert_eq!(test_loads.load(Ordering::SeqCst), 0);
}

#[test]
fn default_subcommand_resolves_lazy_child() {
    let command = Command::new("root")
        .default_subcommand("serve")
        .lazy_command("serve", || {
            Command::new("serve")
                .flag(Flag::new("watch"))
                .handler(|ctx| Ok(ctx.matches().flag("watch")))
        });

    let output = command.run_from(["--watch"]).unwrap();

    assert!(output.downcast::<bool>().unwrap());
}

#[test]
fn default_subcommand_resolves_through_lazy_alias() {
    let command = Command::new("root")
        .default_subcommand("s")
        .lazy_command("serve", || {
            Command::new("serve").alias("s").handler(|_ctx| Ok(()))
        });

    let matches = command.parse_from([] as [&str; 0]).unwrap();

    assert_eq!(matches.subcommand(), Some("serve"));
}

#[test]
fn nested_default_chain_through_lazy_and_eager_commands() {
    let command = Command::new("root")
        .default_subcommand("remote")
        .lazy_command("remote", || {
            Command::new("remote").default_subcommand("add").command(
                Command::new("add")
                    .flag(Flag::new("verbose"))
                    .handler(|ctx| Ok(ctx.matches().flag("verbose"))),
            )
        });

    let output = command.run_from(["--verbose"]).unwrap();

    assert!(output.downcast::<bool>().unwrap());
}

#[test]
fn builtin_help_targets_lazy_child_without_running_its_handler() {
    let ran = Arc::new(AtomicUsize::new(0));
    let counter = ran.clone();

    let command = Command::new("root").lazy_command("build", move || {
        let counter = counter.clone();
        Command::new("build")
            .description("Build the project")
            .handler(move |_ctx| {
                counter.fetch_add(1, Ordering::SeqCst);
                Ok(())
            })
    });

    let action = command.run_cli_from(["build", "--help"]).unwrap();

    match action {
        CliAction::Help(text) => assert!(text.contains("Build the project")),
        other => panic!("expected Help, got {other:?}"),
    }
    assert_eq!(ran.load(Ordering::SeqCst), 0);
}

#[test]
fn nested_lazy_help_renders_resolved_grandchild_usage() {
    let command = Command::new("root").lazy_command("remote", || {
        Command::new("remote").lazy_command("add", || {
            Command::new("add")
                .description("Add a remote")
                .handler(|_ctx| Ok(()))
        })
    });

    let action = command.run_cli_from(["remote", "add", "--help"]).unwrap();

    match action {
        CliAction::Help(text) => assert!(text.contains("Add a remote")),
        other => panic!("expected Help, got {other:?}"),
    }
}

#[test]
fn hidden_lazy_subcommand_omitted_from_usage() {
    let command = Command::new("root")
        .lazy_command("secret", || {
            Command::new("secret").hidden().handler(|_ctx| Ok(()))
        })
        .lazy_command("build", || Command::new("build").handler(|_ctx| Ok(())));

    let usage = command.render_usage();

    assert!(!usage.contains("secret"));
    assert!(usage.contains("build"));
}

#[test]
fn command_with_lazy_subcommand_is_clone() {
    fn assert_clone<T: Clone>() {}
    assert_clone::<Command>();

    let command = Command::new("root").lazy_command("build", || Command::new("build"));
    let _clone = command.clone();
}

#[test]
fn command_with_lazy_subcommand_has_useful_debug() {
    let command = Command::new("root").lazy_command("build", || Command::new("build"));

    let rendered = format!("{command:?}");

    assert!(rendered.contains("root"));
}

#[test]
fn lazy_handler_output_carries_no_extra_bounds() {
    let command = Command::new("root").lazy_command("build", || {
        Command::new("build").handler(|_ctx| {
            let cell = std::rc::Rc::new(std::cell::Cell::new(7));
            Ok(NotDebugCloneSendSync(cell))
        })
    });

    let output = command.run_from(["build"]).unwrap();

    let value = output.downcast::<NotDebugCloneSendSync>().unwrap();
    assert_eq!(value.0.get(), 7);
}

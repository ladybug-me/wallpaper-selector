#![cfg(test)]

use super::{Args, parse_args};

fn args(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}

#[test]
fn flags_from_argv() {
    let parsed = parse_args(args(&["--debug", "--version"]));
    assert!(parsed.debug);
    assert!(parsed.version);
    assert!(!parsed.help);
    assert_eq!(parsed.arguments, args(&["--debug", "--version"]));
    let short = parse_args(args(&["-V"]));
    assert!(short.version);
    assert!(!short.debug);
}

#[test]
fn help_leading_only() {
    assert!(parse_args(args(&["--help"])).help);
    assert!(parse_args(args(&["-h"])).help);
    assert!(!parse_args(args(&["--debug", "--help"])).help);
    let Args { arguments, debug, version, help } = parse_args(Vec::new());
    assert!(arguments.is_empty());
    assert!(!debug && !version && !help);
}

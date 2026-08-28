#![cfg(test)]

use super::*;

fn args(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}

#[test]
fn verbs_delegate() {
    for verb in VERBS {
        assert!(is_helm_command(&args(&[verb])));
    }
    assert!(is_helm_command(&args(&["--help"])));
    assert!(is_helm_command(&args(&["-h"])));
}

#[test]
fn gui_args_ignored() {
    assert!(!is_helm_command(&[]));
    assert!(!is_helm_command(&args(&["--version"])));
    assert!(!is_helm_command(&args(&["--debug"])));
    assert!(!is_helm_command(&args(&["unknown"])));
}

//! Exercise the shipped command surface rather than duplicating parser logic.

use std::process::{Command, Output};

fn apikman(arguments: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_apikman"))
        .args(arguments)
        .env_clear()
        .output()
        .expect("the CLI must start without a user environment")
}

#[test]
fn help_and_version_identify_the_distribution() {
    let help = apikman(&["--help"]);
    assert!(help.status.success());
    let text = String::from_utf8(help.stdout).unwrap();
    assert!(text.contains("development scaffold"));
    assert!(text.contains("status"));
    assert_eq!(help.stderr, [] as [u8; 0]);

    let version = apikman(&["--version"]);
    assert!(version.status.success());
    assert_eq!(
        String::from_utf8(version.stdout).unwrap().trim(),
        format!("apikman {}", env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn structured_status_does_not_claim_unimplemented_security() {
    let output = apikman(&["status", "--json"]);
    assert!(output.status.success());
    assert_eq!(output.stderr, [] as [u8; 0]);
    let status: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(status["schema_version"], 1);
    assert_eq!(status["stage"], "development-scaffold");
    assert_eq!(status["version"], env!("CARGO_PKG_VERSION"));
    for capability in [
        "vault_storage",
        "native_unlock",
        "icloud_unlock",
        "cloud_sync",
        "ai_broker",
    ] {
        assert_eq!(
            status[capability], false,
            "{capability} must be unavailable"
        );
    }
}

#[test]
fn human_status_explains_the_current_limitations() {
    let output = apikman(&["status"]);
    assert!(output.status.success());
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(text.contains("not implemented"));
    assert!(text.contains("native unlock"));
}

#[test]
fn absent_or_unsupported_commands_fail_without_success_output() {
    for arguments in [vec![], vec!["init"], vec!["status", "--unknown"]] {
        let output = apikman(&arguments);
        assert_eq!(output.status.code(), Some(2));
        assert_eq!(output.stdout, [] as [u8; 0]);
        assert_ne!(output.stderr, [] as [u8; 0]);
    }
}

use std::io::Write;
use std::process::{Command, Stdio};

fn cli() -> Command {
    Command::new(env!("CARGO_BIN_EXE_mongol-convert"))
}

#[test]
fn translates_positional_text_without_adding_a_newline() {
    let output = cli()
        .args([
            "translate",
            "--from",
            "z52",
            "--to",
            "menk_shape",
            "plain text",
        ])
        .output()
        .expect("mongol-convert command should run");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.stdout, b"plain text");
    assert!(output.stderr.is_empty());
}

#[test]
fn converts_to_utn57_without_any_external_backend() {
    let output = cli()
        .args([
            "translate",
            "--from",
            "zvvnmod",
            "--to",
            "utn57",
            "\u{E0E5}",
        ])
        .env_clear()
        .output()
        .expect("mongol-convert command should run");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.stdout, "\u{180A}".as_bytes());
    assert!(output.stderr.is_empty());
}

#[test]
fn accepts_utn57_as_a_source() {
    // ᠮᠣᠩᠭᠣᠯ spelled the UTN #57 way, which is not how delehi spells it.
    let utn57 = "\u{182E}\u{1833}\u{180C}\u{182D}\u{180C}\u{182C}\u{180B}\u{1823}\u{182F}";
    let output = cli()
        .args(["translate", "--from", "utn57", "--to", "delehi", utn57])
        .output()
        .expect("mongol-convert command should run");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        output.stdout,
        "\u{182E}\u{1823}\u{1829}\u{182D}\u{1823}\u{182F}".as_bytes()
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn still_rejects_oyun_as_a_source() {
    let output = cli()
        .args(["translate", "--from", "oyun", "--to", "z52", "\u{1820}"])
        .output()
        .expect("mongol-convert command should run");

    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("conversion not supported for Oyun"));
}

#[test]
fn reads_input_from_stdin_when_text_is_omitted() {
    let mut child = cli()
        .args(["translate", "--from", "z52", "--to", "menk_shape"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("mongol-convert command should start");

    child
        .stdin
        .take()
        .expect("stdin should be piped")
        .write_all(b"line one\nline two")
        .expect("stdin should accept input");

    let output = child
        .wait_with_output()
        .expect("mongol-convert command should finish");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.stdout, b"line one\nline two");
    assert!(output.stderr.is_empty());
}

#[test]
fn help_lists_the_command_and_supported_encoding_names() {
    let output = cli()
        .arg("--help")
        .output()
        .expect("mongol-convert command should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("help should be UTF-8");
    assert!(stdout.contains("mongol-convert translate --from <encoding> --to <encoding> [text]"));
    assert!(stdout.contains("reads UTF-8 text from stdin"));
    assert!(stdout.contains("--no-restore-menk-shape-emoji"));
    for encoding in [
        "zvvnmod",
        "delehi",
        "menk_shape",
        "menk_letter",
        "oyun",
        "utn57",
        "z52",
    ] {
        assert!(stdout.contains(encoding), "help omitted {encoding}");
    }
    assert!(output.stderr.is_empty());
}

#[test]
fn version_matches_the_mongol_convert_package() {
    let output = cli()
        .arg("--version")
        .output()
        .expect("mongol-convert command should run");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).expect("version should be UTF-8"),
        format!("mongol-convert {}\n", env!("CARGO_PKG_VERSION"))
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn reports_an_invented_zwj_on_stderr_and_still_succeeds() {
    // A lone medial glyph, U+E09C, can only be spelled with a joiner the hub did not carry.
    let output = cli()
        .args([
            "translate",
            "--from",
            "zvvnmod",
            "--to",
            "utn57",
            "\u{E09C}",
        ])
        .output()
        .expect("mongol-convert command should run");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        output.stdout,
        "\u{200D}\u{182D}\u{180C}\u{1825}\u{180C}".as_bytes()
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.starts_with("mongol-convert: warning: "), "{stderr}");
    assert!(stderr.contains("U+E09C"), "{stderr}");
    assert!(stderr.ends_with('\n'), "{stderr:?}");
}

#[test]
fn converts_the_written_unit_spelling_in_both_directions() {
    let out = cli()
        .args(["translate", "--from", "delehi", "--to", "utn57_shape", "\u{1830}\u{1820}\u{1822}\u{1828}"])
        .output()
        .expect("mongol-convert command should run");
    assert!(out.status.success(), "stderr: {}", String::from_utf8_lossy(&out.stderr));
    assert_eq!(out.stdout, b"SAIIA");
    assert!(out.stderr.is_empty());

    let back = cli()
        .args(["translate", "--from", "utn57_shape", "--to", "utn57", "SAIIA"])
        .output()
        .expect("mongol-convert command should run");
    assert!(back.status.success(), "stderr: {}", String::from_utf8_lossy(&back.stderr));
    assert_eq!(
        back.stdout,
        "\u{1830}\u{1820}\u{1822}\u{180D}\u{1822}\u{180D}\u{1820}\u{180C}".as_bytes()
    );

    let help = cli().arg("--help").output().expect("mongol-convert command should run");
    assert!(String::from_utf8_lossy(&help.stdout).contains("utn57_shape"));
}

#[test]
fn menk_shape_emoji_restore_can_be_disabled_from_cli() {
    let restored = cli()
        .args(["translate", "--from", "menk_shape", "--to", "zvvnmod", "➡️"])
        .output()
        .unwrap();
    assert!(restored.status.success());
    assert_eq!(restored.stdout, "\u{1800}".as_bytes());

    let raw = cli()
        .args([
            "translate",
            "--from",
            "menk_shape",
            "--to",
            "zvvnmod",
            "--no-restore-menk-shape-emoji",
            "➡️",
        ])
        .output()
        .unwrap();
    assert!(raw.status.success());
    assert_eq!(raw.stdout, "➡️".as_bytes());
}

#[test]
fn suffix_repair_is_explicit_and_reports_edits_on_stderr() {
    let out = cli()
        .args(["translate", "--from", "menk_letter", "--to", "menk_letter", "--repair-suffix-separators", "ᠤᠯᠤᠰ ᠤᠨ"])
        .output().unwrap();
    assert!(out.status.success());
    assert_eq!(out.stdout, "ᠤᠯᠤᠰ\u{202F}ᠤᠨ".as_bytes());
    assert!(String::from_utf8_lossy(&out.stderr).contains("input byte 12: U+0020 -> U+202F"));
}

#[test]
fn suffix_repair_reads_stdin_and_literal_option_can_be_escaped() {
    let mut child = cli()
        .args(["translate", "--from", "delehi", "--to", "delehi", "--repair-suffix-separators"])
        .stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all("ᠤᠯᠤᠰ ᠤᠨ".as_bytes()).unwrap();
    let out = child.wait_with_output().unwrap();
    assert!(out.status.success());
    assert_eq!(out.stdout, "ᠤᠯᠤᠰ\u{202F}ᠤᠨ".as_bytes());

    let out = cli().args(["translate", "--from", "delehi", "--to", "delehi", "--", "--repair-suffix-separators"]).output().unwrap();
    assert!(out.status.success());
    assert_eq!(out.stdout, b"--repair-suffix-separators");
    assert!(out.stderr.is_empty());
}

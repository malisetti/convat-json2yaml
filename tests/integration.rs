use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn converts_json_lines_to_yaml_one_doc_per_line() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_convat-json2yaml"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn convat-json2yaml");

    {
        let stdin = child.stdin.as_mut().expect("stdin");
        writeln!(stdin, r#"{{"name":"alice"}}"#).unwrap();
        writeln!(stdin, r#"{{"count":42}}"#).unwrap();
    }

    let output = child.wait_with_output().expect("wait");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout, "name: alice\ncount: 42\n", "unexpected stdout: {stdout:?}");
}

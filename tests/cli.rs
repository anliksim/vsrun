use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::process::Command;

// TODO add interface to mock actual vscode call
#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new(".vscode/extensions.json")?;
    file.write_str(
        r#"{
            "recommendations": [
                "vadimcn.vscode-lldb",
                "mutantdino.resourcemonitor",
                "rust-lang.rust-analyzer",
                "tamasfe.even-better-toml",
                "serayuzgur.crates"
            ],
            "unwantedRecommendations": [
                "vscode.typescript-language-features"
            ]
        }"#,
    )?;

    let mut cmd = Command::cargo_bin("vsrun")?;
    cmd.arg(file.parent().unwrap().parent().unwrap());
    cmd.assert().success();

    Ok(())
}

#[test]
fn extensions_file_wrong_format() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new(".vscode/extensions.json")?;
    file.write_str("this should be json")?;

    let mut cmd = Command::cargo_bin("vsrun")?;
    cmd.arg(file.parent().unwrap().parent().unwrap());
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not parse extensions.json"));

    Ok(())
}

#[test]
fn extensions_file_not_exists() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("vsrun")?;
    cmd.arg(file.parent().unwrap());
    cmd.assert().success();

    Ok(())
}

#[test]
fn dir_does_not_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("vsrun")?;

    cmd.arg("../dir/missing");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read dir"));

    Ok(())
}

use std::{fs, process::Command};

use anyhow::{Context, Result};
use clap::Parser;
use log::{debug, info};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[clap(version, author, about, long_about = None)]
struct Cli {
    /// The path to the file to read
    path: Option<std::path::PathBuf>,

    /// Extra agruments that are passed to vscode
    #[arg(value_parser)]
    args: Option<String>,
    // Do not install extensions in recommendations
    // #[clap(long, action)]
    // skip_install: bool,

    // Do not disable extensions in unwantedRecommendations
    // #[clap(long, action)]
    // skip_disable: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Extensions {
    recommendations: Option<Vec<String>>,
    unwanted_recommendations: Option<Vec<String>>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    debug!("starting up");

    let args = Cli::parse();
    debug!("{:?}", args);

    let path = args
        .path
        .with_context(|| format!("please specify a project path"))?;

    path.read_dir()
        .with_context(|| format!("could not read dir `{:?}`", path))?;

    let path = path.as_path().to_str().unwrap();
    let extensions_path = path.to_owned() + "/.vscode/extensions.json";

    info!("running project {}", path);

    let extensions = fs::read_to_string(extensions_path)
        .with_context(|| format!("could not find extensions.json"))?;
    let extensions: Extensions = serde_json::from_str(&extensions)
        .with_context(|| format!("could not parse extensions.json"))?;
    debug!("extensions.json: {:#?}", extensions);

    let all_extensions = Command::new("cmd")
        .args(["/C", "code", "--list-extensions"])
        .output()
        .expect("failed to execute process");
    let all_extensions = all_extensions.stdout;
    let all_extensions = match std::str::from_utf8(&all_extensions) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let mut installs = Vec::new();
    let mut disables = Vec::new();

    let recommnedations = extensions.recommendations.unwrap_or_default();
    for entry in &recommnedations {
        if !entry.trim().is_empty() {
            installs.push("--install-extension");
            installs.push(entry);
        }
    }

    let unwanted_recommendations = extensions.unwanted_recommendations.unwrap_or_default();
    for entry in &unwanted_recommendations {
        if !entry.trim().is_empty() {
            disables.push("--disable-extension");
            disables.push(entry);
        }
    }

    let not_used = not_in_installs(all_extensions.to_owned(), &installs);

    for entry in &not_used {
        if !entry.trim().is_empty() {
            disables.push("--disable-extension");
            disables.push(entry);
        }
    }

    let mut args = vec!["/C", "code"];
    args.extend(installs);
    args.extend(vec!["&&", "code"]);
    args.extend(disables);
    args.push(path);

    debug!("args: {:#?}", args);

    let output = Command::new("cmd")
        .args(args)
        .output()
        .expect("failed to execute process");

    let hello = output.stdout;

    let s = match std::str::from_utf8(&hello) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    debug!("out: {}", s);

    Ok(())
}

fn not_in_installs(all: String, installs: &Vec<&str>) -> Vec<String> {
    let mut output = Vec::new();
    for ext in all.lines() {
        let mut is_used = false;
        for inst in installs {
            if ext.eq_ignore_ascii_case(&inst) {
                is_used = true;
            }
        }
        if !is_used {
            output.push(ext.to_owned())
        }
    }
    output
}

#[test]
fn remove_installs_from_disable() {
    let all = "Abc\ndef";
    let installs = vec!["abc"];
    let result = not_in_installs(all.to_owned(), &installs);
    assert_eq!(result, vec!["def"]);
}

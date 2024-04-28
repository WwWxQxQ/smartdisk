use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command};
use clap::{Parser};
use anyhow::{Error, Result};
use chrono::Utc;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "DEVICE")]
    device: String,

    #[arg(short, long, value_name = "FILE", default_value = "./disk_record.md")]
    file: Option<PathBuf>,

}

fn main() {
    if let Err(e) = run() {
        eprintln!("An error occurred: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    let mut smart_ctl_command = Command::new("smartctl");
    let child = smart_ctl_command.arg("-a")
        .arg(&cli.device)
        .output()
        .expect("smartctl command failed to start");

    if !child.stderr.is_empty() {
        return Err(Error::msg(String::from_utf8(child.stderr)?));
    }

    let out = String::from_utf8(child.stdout)?;
    write_to_file(&out, cli.file.unwrap())?;

    let list: HashMap<&str, &str> = out
        .split('\n')
        .map(|s| parse_hash_map(s))
        .filter(|(k, v)| k.is_some() && v.is_some())
        .map(|(k, v)| (k.unwrap().trim(), v.unwrap().trim()))
        .collect();

    
    for (k, v) in list {
        println!("{}: {}", k, v);
    }

    // println!("{:?}", list);
    Ok(())
}

fn write_to_file(content: &String, path: PathBuf) -> Result<()> {
    let mut file = match path.exists() {
        true => {
            match File::options().append(true).open(path) {
                Ok(file) => { file }
                Err(e) => { return Err(Error::msg(e.to_string())); }
            }
        }
        false => {
            match File::create_new(path) {
                Ok(file) => { file }
                Err(e) => { return Err(Error::msg(e.to_string())); }
            }
        }
    };

    let markdown = wrap_markdown(content);
    match file.write_all(markdown.as_bytes()) {
        Ok(_) => {
            println!("file write success");
            Ok(())
        }
        Err(e) => { Err(Error::msg(e.to_string())) }
    }
}

fn wrap_markdown(content: &String) -> String {
    // 获取今天的日期 格式为 yyyy-MM-dd
    let now = Utc::now().format("%m%d").to_string();
    return format!("\n\n## {now}\n```shell\n{}\n```\n\n", content.trim());
}


fn parse_hash_map(s: &str) -> (Option<&str>, Option<&str>) {
    let kv: Vec<&str> = s.split(':').map(|s| s.trim()).collect();
    match kv.len() {
        0 => (None, None),
        1 => (Some(kv[0]), None),
        _ => (Some(kv[0]), Some(kv[1])),
    }
}
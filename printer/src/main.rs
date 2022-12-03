mod pastebin;
mod qr;

use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use std::{
    env,
    error::Error,
    fs::{read_to_string, write},
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::mpsc::channel,
};

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    dbg!(&args);

    dotenv().ok();

    match &args.cmd {
        SubCommand::Decode { path } => decode(Path::new(&path))?,
        SubCommand::Encode {
            input,
            output,
            year,
            day,
        } => encode(input, output, *year, *day)?,
        SubCommand::Watch { dir } => watch(dir)?,
    }

    Ok(())
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[clap(subcommand)]
    cmd: SubCommand,
}
#[derive(Debug, Subcommand)]
enum SubCommand {
    Decode {
        path: String,
    },
    Encode {
        input: String,
        output: String,
        #[clap(short)]
        year: u32,
        #[clap(short)]
        day: u8,
    },
    Watch {
        dir: PathBuf,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    year: u32,
    day: u8,
    input: String,
}

fn encode(input: &str, output: &str, year: u32, day: u8) -> Result<(), Box<dyn Error>> {
    let msg = read_to_string(input)?;
    let res = pastebin::upload(&msg)?;
    dbg!(&res);
    let data = Data {
        year,
        day,
        input: res,
    };
    qr::encode(serde_json::to_string(&data)?, output)
}

pub fn decode(path: &Path) -> Result<(), Box<dyn Error>> {
    let decoded = qr::decode(path)?;

    for s in decoded {
        if let Ok(data) = serde_json::from_str::<Data>(&s) {
            dbg!(&data);
            let input = pastebin::download(&data.input)?;
            let res = exec(data.year, data.day, input)?;
            eprintln!("result:\n{res}");
            write("output.txt", &res)?;
            print(&res)?;
        } else {
            eprintln!("failed to decode: {s}");
        }
    }

    Ok(())
}

fn exec(year: u32, day: u8, input: String) -> Result<String, Box<dyn Error>> {
    let root = find_root(year)?;
    eprintln!("executing in {}", root.display());
    let mut cmd = Command::new("cargo")
        .current_dir(root)
        .args([
            "run",
            "--release",
            "--package",
            &format!("day{day}"),
            "--",
            "-",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    cmd.stdin
        .take()
        .expect("failed to open stdin")
        .write_all(input.as_bytes())?;

    let output = cmd.wait_with_output()?;

    let mut res = format!("status: {}", output.status);
    if !output.stdout.is_empty() {
        res += "\n\nstdout:\n";
        res += String::from_utf8_lossy(&output.stdout).trim();
    }
    if !output.stderr.is_empty() {
        res += "\n\nstderr:\n";
        res += String::from_utf8_lossy(&output.stderr).trim();
    }
    Ok(res)
}

fn find_root(year: u32) -> Result<PathBuf, Box<dyn Error>> {
    let mut path = env::current_dir()?;
    loop {
        if path.join(".git").exists() {
            break;
        }
        if !path.pop() {
            return Err("failed to find root".into());
        }
    }
    path.push("rust");
    path.push(year.to_string());
    Ok(path)
}

fn print(s: &str) -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::new("pwsh.exe")
        .args([
            "-NoLogo",
            "-NoProfile",
            "-NonInteractive",
            "-Command",
            r#"$input | Out-Printer"#,
        ])
        .stdin(Stdio::piped())
        .spawn()?;

    cmd.stdin
        .take()
        .expect("failed to open stdin")
        .write_all(s.as_bytes())?;

    cmd.wait()?;
    Ok(())
}

fn watch(dir: &Path) -> Result<(), Box<dyn Error>> {
    let (tx, rx) = channel();
    let mut watcher = RecommendedWatcher::new(tx, notify::Config::default())?;
    watcher.watch(dir, RecursiveMode::NonRecursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                println!("changed: {:?}", &event);
                if event.kind == EventKind::Create(notify::event::CreateKind::Any)
                    && event.paths.len() == 1
                {
                    let path = &event.paths[0];
                    if let Some(ext) = path.extension() {
                        if ext == "jpg" {
                            println!("decoding: {:?}", path);
                            decode(path)?;
                        }
                    }
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

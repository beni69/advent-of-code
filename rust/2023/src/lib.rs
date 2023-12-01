use std::error::Error;

pub mod input {
    use super::Result;
    use std::{
        env::args,
        io::{stdin, Read},
        path::PathBuf,
        process::Command,
        str::FromStr,
    };

    type Date = (u32, u8);

    pub fn get_input((year, day): Date) -> Result<String> {
        // Read from stdin
        if args().len() > 1 && args().nth(1).unwrap() == "-" {
            let mut input = String::new();
            stdin().read_to_string(&mut input)?;
            return Ok(input.to_string());
        }

        let cwd = std::env::current_dir()?;
        let mut js: Option<PathBuf> = None;
        for p in cwd.ancestors() {
            for f in p.read_dir()? {
                let f = f?;
                if f.file_name() == "js-ts" {
                    js = Some(f.path());
                }
            }
        }
        let dir = match js {
            Some(p) => p,
            None => return Err("js-ts directory couldn't be found".into()),
        };

        let f = dir.join(format!(".cache/input-{year}-{day}.txt"));
        match f.try_exists() {
            Ok(true) => (),
            _ => get_input_exec(year, day, &dir)?,
        }

        Ok(std::fs::read_to_string(f)?)
    }

    fn get_input_exec(year: u32, day: u8, path: &PathBuf) -> Result<()> {
        let status = Command::new("deno")
            .args([
                "run",
                "--allow-read",
                "--allow-write",
                "--allow-net",
                "runner.ts",
                "--dl",
                "-y",
                &year.to_string(),
                "-d",
                &day.to_string(),
            ])
            .current_dir(path)
            .status()?;
        eprintln!("input download finished with {status}");
        Ok(())
    }

    pub fn get_input_lines(date: Date) -> Result<Vec<String>> {
        Ok(get_input(date)?.lines().map(String::from).collect())
    }
    pub fn get_input_parse<T: FromStr>(date: Date) -> Result<Vec<T>> {
        get_input(date)?
            .lines()
            .map(|x| match x.parse() {
                Ok(x) => Ok(x),
                Err(_) => Err(format!("{x:?}: parse failed").into()),
            })
            .collect()
    }
}

#[macro_export]
macro_rules! year_day {
    () => {
        (
            2023,
            (env!("CARGO_PKG_NAME").replace("day", ""))
                .parse()
                .expect("day number parse failed"),
        )
    };
}

#[macro_export]
macro_rules! main {
    {$($tt:tt)*} => {
        fn main() -> Result<()> {
            $($tt)*;
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! dprintln {
    ($($tt:tt)*) => {{
        #[cfg(debug_assertions)]
        eprintln!($($tt)*);
    }};
}
#[macro_export]
macro_rules! dprint {
    ($($tt:tt)*) => {{
        #[cfg(debug_assertions)]
        eprint!($($tt)*);
    }};
}
#[macro_export]
macro_rules! ddbg {
    ($($tt:tt)*) => {{
        #[cfg(debug_assertions)]
        dbg!($($tt)*);
    }};
}

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

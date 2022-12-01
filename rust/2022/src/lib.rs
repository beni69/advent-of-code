pub mod input {
    use std::{error::Error, path::PathBuf, process::Command, str::FromStr};

    type Date = (u32, u8);
    type Result<T> = std::result::Result<T, Box<dyn Error>>;

    pub fn get_input((year, day): Date) -> Result<String> {
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
        println!("import download finished with {status}");
        Ok(())
    }

    pub fn get_input_lines(date: Date) -> Result<Vec<String>> {
        Ok(get_input(date)?
            .trim()
            .split("\n")
            .map(String::from)
            .collect())
    }
    pub fn get_input_parse<T: FromStr>(date: Date) -> Result<Vec<T>> {
        get_input(date)?
            .trim()
            .split("\n")
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
            2022,
            (env!("CARGO_PKG_NAME").replace("day", ""))
                .parse()
                .expect("day number parse failed"),
        )
    };
}

use serde::Serialize;
use serde_json::Value;
use std::error::Error;
use std::fmt::Write as FmtWrite;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::{fs, io};
use tinytemplate::TinyTemplate;

fn days(input_dir: &str) -> io::Result<Vec<u32>> {
    let mut days = read_dir(input_dir)?
        .flatten()
        .filter(|e| e.path().is_file())
        .flat_map(|e| e.file_name().into_string())
        .flat_map(|s| s[0..2].parse::<u32>())
        .collect::<Vec<_>>();
    days.sort_unstable();
    Ok(days)
}

#[derive(Serialize)]
struct Context<'a> {
    days: &'a [u32],
}

fn leading_zero(v: &Value, output: &mut String) -> tinytemplate::error::Result<()> {
    if let Value::Number(n) = v {
        if let Some(n) = n.as_u64() {
            write!(output, "{0:02}", n)?;
        }
    }

    Ok(())
}

fn gen_solutions_mod<P: AsRef<Path>>(p: P, days: &[u32]) -> io::Result<()> {
    let mut tpl = TinyTemplate::new();
    tpl.add_template("mod", include_str!("mod.rs.template"))
        .unwrap();
    tpl.add_formatter("leading_zero", leading_zero);
    let s = tpl.render("mod", &Context { days }).unwrap();

    fs::write(p, s)
}

fn gen_solutions(dir: &str, days: &[u32]) -> io::Result<()> {
    for day in days {
        let file = PathBuf::from(format!("{}/day{:02}.rs", dir, day));
        if file.exists() {
            continue;
        }

        fs::copy("solution.rs.template", file)?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_dir = "./input";
    let output_dir = "./src/solutions";
    let solutions_mod_output_path = Path::new(&output_dir).join("mod.rs");

    let days = days(input_dir)?;

    // write solutions mod file
    gen_solutions_mod(solutions_mod_output_path, &days)?;

    // write solutions
    gen_solutions(output_dir, &days)?;

    Ok(())
}

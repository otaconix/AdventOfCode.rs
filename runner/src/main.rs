use clap::Parser;
use directories::ProjectDirs;
use dotenvy::dotenv;
use reqwest::blocking::ClientBuilder;
use reqwest::header::COOKIE;
use std::fs::{create_dir_all, write, File};
use std::os::unix::process::CommandExt;
use std::process::{exit, Command, Stdio};

#[derive(Parser, Debug)]
struct Opt {
    year: u16,
    day: u8,
    command: String,
    args: Vec<String>,
    #[arg(long = "contact-info", env = "AOC_CONTACT_INFO")]
    contact_info: String,
    #[arg(long = "session", env = "AOC_SESSION")]
    session: String,
}

fn main() {
    dotenv().ok();
    let opt = Opt::parse();
    let input_file = get_input_file(&opt).unwrap();

    let exec_error = Command::new(opt.command)
        .args(opt.args)
        .stdin(Stdio::from(input_file))
        .exec();

    println!("Error executing solution: {exec_error}");
    exit(1);
}

fn get_input_file(opt: &Opt) -> Result<File, String> {
    let cache_dir = &ProjectDirs::from(
        "nl",
        "svanur",
        option_env!("CARGO_PACKAGE_NAME").unwrap_or("aoc-runner"),
    )
    .ok_or("Couldn't determine cache dir")?
    .cache_dir()
    .join(opt.year.to_string());
    let cache_file = cache_dir.join(format!("{:02}", opt.day));

    create_dir_all(cache_dir).map_err(|e| {
        format!(
            "Couldn't create cache directory {}: {e}",
            cache_dir.display()
        )
    })?;

    if !cache_file.exists() {
        eprintln!("Puzzle input not cached, fetching from adventofcode.com");
        let client = ClientBuilder::new()
            .user_agent(&opt.contact_info)
            .build()
            .map_err(|e| format!("Couldn't create HTTP client: {e}"))?;
        let response = client
            .get(format!(
                "https://adventofcode.com/{}/day/{}/input",
                opt.year, opt.day
            ))
            .header(COOKIE, format!("session={}", opt.session))
            .send()
            .map_err(|e| format!("{e}"))?
            .error_for_status()
            .map_err(|e| format!("Server responded with error: {e}"))?
            .text()
            .map_err(|e| format!("Error retrieving response body: {e}"))?;

        write(&cache_file, response).map_err(|e| format!("Couldn't write into cache: {e}"))?
    } else {
        eprintln!("Puzzle input will be read from cache.");
    }

    File::open(cache_file).map_err(|e| format!("Couldn't open input file: {e}"))
}

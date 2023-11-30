use clap::Parser;
use directories::ProjectDirs;
use dotenvy::dotenv;
use reqwest::blocking::ClientBuilder;
use reqwest::header::COOKIE;
use std::env;
use std::fs::{create_dir_all, write, File};
use std::os::unix::process::CommandExt;
use std::process::{exit, Command, Stdio};

#[derive(Parser, Debug)]
struct Opt {
    year: u16,
    day: u8,
    command: String,
    args: Vec<String>,
}

const CONTACT_INFO: &str = "AOC_CONTACT_INFO";
const SESSION: &str = "AOC_SESSION";

fn main() {
    dotenv().ok();
    let opt = Opt::parse();
    let input_file = get_input_file(opt.year, opt.day).unwrap();

    let exec_error = Command::new(opt.command)
        .args(opt.args)
        .stdin(Stdio::from(input_file))
        .exec();

    println!("Error executing solution: {exec_error}");
    exit(1);
}

fn get_input_file(year: u16, day: u8) -> Result<File, String> {
    let contact_info = env::var(CONTACT_INFO).unwrap();
    let session = env::var(SESSION).unwrap();
    let cache_dir = &ProjectDirs::from(
        "nl",
        "svanur",
        option_env!("CARGO_PACKAGE_NAME").unwrap_or("aoc-runner"),
    )
    .ok_or("Couldn't determine cache dir")?
    .cache_dir()
    .join(year.to_string());
    let cache_file = cache_dir.join(format!("{day:02}"));

    create_dir_all(cache_dir).map_err(|e| {
        format!(
            "Couldn't create cache directory {}: {e}",
            cache_dir.display()
        )
    })?;

    if !cache_file.exists() {
        eprintln!("Puzzle input not cached, fetching from adventofcode.com");
        let client = ClientBuilder::new()
            .user_agent(contact_info)
            .build()
            .map_err(|e| format!("Couldn't create HTTP client: {e}"))?;
        let response = client
            .get(format!("https://adventofcode.com/{year}/day/{day}/input"))
            .header(COOKIE, format!("session={session}"))
            .send()
            .map_err(|e| format!("{}", e))?
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

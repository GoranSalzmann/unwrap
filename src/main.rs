mod counter;
use std::{fs, io::stdin, time::Instant};

use counter::Counter;
use reqwest::Error;

const EXPR: &'static str = "if err != nil";
const SRC_DIR: &'static str = "./data";
const EXTS: &[&'static str] = &["go"];

#[tokio::main]
async fn main() {
    let mut counter = Counter::new();

    load_repo()
        .await
        .unwrap_or_else(|_| fs::remove_dir(SRC_DIR).unwrap());

    counter.run();

    counter.print_results();
}

async fn load_repo() -> Result<(), Error> {
    let now = Instant::now();

    fs::create_dir_all(SRC_DIR).unwrap();

    let mut repo = String::new();

    println!("Repo Format: <owner>/<repo>");
    stdin()
        .read_line(&mut repo)
        .expect("No repository specified.");

    repo = repo.trim().to_owned();

    let link = format!("https://github.com/{}/archive/refs/heads/master.zip", repo);
    println!("Searching through {}.", link);
    let req = reqwest::get(link).await?;
    let bytes = req.bytes().await?;

    fs::write(format!("{}/file.zip", SRC_DIR), bytes).unwrap();

    println!("Downloaded in {} ms.", now.elapsed().as_millis());

    Ok(())
}

use std::{
    fs::{self},
    time::Instant,
};

use colored::*;

const EXPR: &'static str = "if err != nil";
const _REPO: &'static str = "https://github.com/serenity-rs/serenity";
const SRC_DIR: &'static str = "./source";

#[tokio::main]
async fn main() {
    let mut counter = Counter::new();

    counter.run();
    Counter::cleanup();
    counter.print_results();
}

struct Counter {
    loc: u64,
    err: u64,
    files: u64,
    time: u128,
}

impl Counter {
    pub fn new() -> Self {
        Self {
            loc: 0,
            err: 0,
            files: 0,
            time: 0,
        }
    }

    pub fn line(&mut self) {
        self.loc += 1;
    }

    pub fn err(&mut self) {
        self.err += 1;
    }

    pub fn file(&mut self) {
        self.files += 1;
    }

    pub fn run(&mut self) {
        let now = Instant::now();
        self.search_dir(SRC_DIR);
        self.time = now.elapsed().as_millis();
    }

    pub fn cleanup() {
        fs::remove_dir_all(SRC_DIR).unwrap();
    }

    fn search_dir(&mut self, dir: &str) {
        let root = fs::read_dir(dir).unwrap();
        for dir_entry in root {
            let entry = dir_entry.unwrap();
            let file_type = entry.file_type().unwrap();
            let path = entry.path();
            if file_type.is_dir() {
                self.search_dir(path.to_str().unwrap())
            }
            if file_type.is_file()
                && path.extension().is_some()
                && path.extension().unwrap() == "go"
            {
                let src = fs::read_to_string(path);
                match src {
                    Ok(source) => {
                        for line in source.lines() {
                            if line.contains(EXPR) {
                                self.err();
                            }
                            self.line();
                        }
                        self.file();
                    }
                    Err(_) => (),
                }
            }
        }
    }

    pub fn print_results(self) {
        if self.loc == 0 {
            println!("No lines searched.");
            return;
        }
        let loc = format!("{}", self.loc).blue();
        let err = format!("{}", self.err).blue();
        let files = format!("{}", self.files).blue();
        let percentage = format!("{:.2}", self.err as f64 / self.loc as f64).green();
        let expr = EXPR.red();

        println!(
            "Searched through {} lines of code in {} files. Found '{}' {} times. Total percentage: {}%",
            loc,
            files,
            expr,
            err,
            percentage
        );

        println!("Ran in {} ms.", self.time);
    }
}

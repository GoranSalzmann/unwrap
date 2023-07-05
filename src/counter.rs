use std::{
    fs,
    io::{BufReader, Read},
    time::Instant,
};

use colored::Colorize;
use zip::read::ZipFile;

use crate::{EXPR, EXTS, SRC_DIR};

pub struct Counter {
    loc: u64,
    expr: u64,
    files: u64,
    time: u128,
}

impl Counter {
    pub fn new() -> Self {
        Self {
            loc: 0,
            expr: 0,
            files: 0,
            time: 0,
        }
    }

    fn line(&mut self) {
        self.loc += 1;
    }

    fn expr(&mut self) {
        self.expr += 1;
    }

    fn file(&mut self) {
        self.files += 1;
    }

    pub fn run(&mut self) {
        fs::read_dir(SRC_DIR).expect("Data Directory not present.");
        let now = Instant::now();
        self.search_zip(format!("{}/file.zip", SRC_DIR));
        self.time = now.elapsed().as_millis();
        fs::remove_dir_all(SRC_DIR).unwrap();
    }

    fn search_zip(&mut self, path: String) {
        let file = fs::File::open(path).unwrap();
        let reader = BufReader::new(file);
        let archive_res = zip::ZipArchive::new(reader);
        match archive_res {
            Ok(mut archive) => {
                for i in 0..archive.len() {
                    let mut file = archive.by_index(i).unwrap();
                    for ext in EXTS {
                        if file.name().ends_with(ext) {
                            self.search_file(&mut file);
                        }
                    }
                }
            }
            Err(_) => println!("Invalid Archive."),
        }
    }

    fn search_file(&mut self, file: &mut ZipFile) {
        self.file();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        for line in content.lines() {
            self.line();
            if line.contains(EXPR) {
                self.expr();
            }
        }
    }

    pub fn print_results(self) {
        if self.loc == 0 {
            println!("No lines searched.");
            return;
        }
        let loc = format!("{}", self.loc).blue();
        let err = format!("{}", self.expr).blue();
        let files = format!("{}", self.files).blue();
        let percentage = format!("{:.2}", self.expr as f64 / self.loc as f64).green();
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

extern crate clap;

mod expr;
mod scanner;
mod token;
mod token_type;

use crate::scanner::Scanner;
use clap::{App, Arg};
use std::fs;
use std::io::{self, Write};

static mut HAD_ERROR: bool = false;

fn main() {
    let matches = App::new("CNF SAT Solver")
        .version("0.1.0")
        .author("Bobbie Soedirgo <bobbie@soedirgo.dev>")
        .about("Lox interpreter")
        .arg(
            Arg::with_name("script")
                .value_name("FILE")
                .help("Path of lox script")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    if matches.is_present("script") {
        let file_path = matches.value_of("script").unwrap();
        run_file(file_path);
    } else {
        run_prompt();
    }
}

fn run_file(path: &str) {
    let script = fs::read_to_string(path).expect("Failed reading from file");
    run(&script);

    unsafe {
        if HAD_ERROR {
            panic!(65);
        }
    }
}

fn run_prompt() {
    let mut line = String::new();
    loop {
        print!("❯ ");
        // Need to flush the print above because it's buffered
        io::stdout().flush().unwrap();
        line.clear();
        io::stdin().read_line(&mut line).unwrap();
        if line.is_empty() {
            break;
        }
        run(&line);
        unsafe {
            HAD_ERROR = false;
        }
    }
}

fn run(source: &str) {
    let scanner = Scanner::new(source.to_string());
    let tokens = scanner.scan_tokens();

    tokens.into_iter().for_each(|x| println!("tok: {}", x))
}

pub fn error(line: usize, message: &str) {
    report(line, "", message);
}

pub fn report(line: usize, location: &str, message: &str) {
    eprintln!("[line {}] Error{}: {}", line, location, message);

    unsafe {
        HAD_ERROR = true;
    }
}

use std::fs;
use std::fs::*;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Option {
    #[structopt(short = "q", long = "quiet")]
    quiet: bool,
    #[structopt(name = "directory")]
    directory: String,
}

fn full_path(from: String) -> String {
    fs::canonicalize(&(PathBuf::from(from)))
        .expect("can not parse directory path")
        .to_str()
        .expect("can not parse directory path")
        .to_string()
}

fn mounted(lines: Vec<&str>, path: String) -> bool {
    let pattern = format!("on {} ", path);
    for line in lines {
        match line.find(pattern.as_str()) {
            Some(_) => {
                return true;
            }
            None => {}
        }
    }
    return false;
}

fn main() {
    let opt = Option::from_args();
    match Command::new("mount").output() {
        Ok(output) => {
            if mounted(
                String::from_utf8(output.stdout)
                    .expect("can not convert u8 to string")
                    .split("\n")
                    .collect(),
                full_path(format!("{}", &(opt).directory)),
            ) {
                if !&(opt).quiet {
                    println!("{} is a mountpoint", &(opt).directory)
                }
                exit(0);
            } else {
                if !&(opt).quiet {
                    println!("{} is not a mountpoint", &(opt).directory)
                }
                exit(1);
            }
        }
        Err(_) => {
            panic!("command `mount` is not found!");
        }
    }
}

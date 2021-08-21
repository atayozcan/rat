use std::{fs::read_to_string, path::PathBuf};
use structopt::{clap::AppSettings, StructOpt};

#[derive(StructOpt)]
#[structopt(setting = AppSettings::InferSubcommands)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: PathBuf,

    #[structopt(short = "n", long)]
    number: bool,

    #[structopt(short = "E", long)]
    show_ends: bool,

    #[structopt(short = "T", long)]
    show_tabs: bool,

    #[structopt(short = "s", long)]
    squeeze_blank: bool,

    #[structopt(short = "b", long)]
    number_nonblank: bool,
}

fn main() {
    std::env::args().nth(2);

    let mut read = read_to_string(&Cli::from_args().path).unwrap();

    match Cli::from_args() {
        Cli { number: true, .. } => {
            println!("{}", number(&mut read))
        }
        Cli {
            show_ends: true, ..
        } => {
            println!("{}", show_ends(&mut read));
        }
        Cli {
            show_tabs: true, ..
        } => {
            println!("{}", show_tabs(&mut read))
        }
        Cli {
            squeeze_blank: true,
            ..
        } => {
            println!("{}", squeeze_blank(&mut read))
        }
        Cli {
            number_nonblank: true,
            ..
        } => {
            println!("{}", number_nonblank(&mut read))
        }
        _ => {
            println!("{}", read)
        }
    }
}

fn number(read: &mut String) -> String {
    let mut i = 1;
    let mut new_read = String::new();
    let digits = &read.lines().count().to_string().len();
    for line in read.lines() {
        new_read.push_str(spaces(*digits).as_str());
        let s = format!("{:>d$}", &*i.to_string(), d = digits);
        new_read.push_str(&*s);
        new_read.push('\t');
        new_read.push_str(line);
        new_read.push('\n');
        i += 1;
    }
    new_read.trim_end().to_string()
}

fn show_ends(read: &mut String) -> String {
    let mut new_read = String::new();
    for line in read.lines() {
        new_read.push_str(line);
        new_read.push('$');
        new_read.push('\n');
    }
    new_read
}

fn show_tabs(read: &mut String) -> String {
    read.replace("\t", "^I")
}

fn squeeze_blank(read: &mut String) -> String {
    read.trim_matches('\n').to_string()
}

fn number_nonblank(read: &mut String) -> String {
    let mut i = 1;
    let mut new_read = String::new();
    let digits = &read.lines().count().to_string().len();
    for line in read.lines() {
        if !line.is_empty() {
            new_read.push_str(spaces(*digits).as_str());
            let s = format!("{:>d$}", &*i.to_string(), d = digits);
            new_read.push_str(&*s);
            new_read.push('\t');
            new_read.push_str(line);
            new_read.push('\n');
            i += 1;
        } else {
            new_read.push('\n');
        }
    }
    new_read
}

fn spaces(digits: usize) -> String {
    let times = 6 - digits;
    let mut space = "".to_string();
    for _ in 0..times {
        space.push(' ');
    }
    space
}

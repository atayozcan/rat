use std::fs::read_to_string;
use std::io::{stdin, stdout, BufRead, Write};
use std::path::{Display, PathBuf};
use std::str::from_utf8;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

use anyhow::{Context, Result};
use clap::Parser;

const ABOUT: &str = "Concatenate FILE(s) to standard output.\n\nWith no FILE, or when FILE is -, read standard input.";
static LINE_NUMBER: AtomicUsize = AtomicUsize::new(1);

#[derive(Parser, Clone)]
#[command(author, version, about = ABOUT)]
struct Cli {
    #[arg(name = "FILE")]
    file: Vec<PathBuf>,
    ///equivalent to -vET
    #[arg(short = 'A', long)]
    show_all: bool,
    ///number nonempty output lines, overrides -n
    #[arg(short = 'b', long)]
    number_nonblank: bool,
    ///equivalent to -vE
    #[arg(short = 'e')]
    e: bool,
    ///display $ at end of each line
    #[arg(short = 'E', long)]
    show_ends: bool,
    ///number all output lines
    #[arg(short = 'n', long)]
    number: bool,
    ///suppress repeated empty output lines
    #[arg(short = 's', long)]
    squeeze_blank: bool,
    ///equivalent to -vT
    #[arg(short = 't')]
    t: bool,
    ///display TAB characters as ^I
    #[arg(short = 'T', long)]
    show_tabs: bool,
    ///(ignored)
    #[arg(short = 'u')]
    u: bool,
    ///use ^ and M- notation, except for LFD and TAB
    #[arg(short = 'v', long)]
    show_nonprinting: bool,
}

fn main() -> Result<()> {
    let mut args = Cli::parse();
    let mut buf;
    if args.file.is_empty() {
        read_from_stdin(&mut args);
    }
    for file in args.clone().file.iter() {
        if *file == PathBuf::from("-") {
            read_from_stdin(&mut args);
            continue;
        }
        buf = read_to_string(file).context(err_in_path(file.display()))?;
        buf = init(&args, buf);
        print(buf);
    }
    Ok(())
}

fn err_in_path(file_name: Display) -> String {
    format!("rat: {}: No such file or directory", file_name)
}

fn read_from_stdin(args: &mut Cli) {
    let stdin = stdin();
    let mut stdin = stdin.lock();
    loop {
        let buffer = from_utf8(stdin.fill_buf().unwrap()).unwrap().to_string();
        print(init(&args, buffer.clone()));
        stdin.consume(buffer.len());
    }
}

fn print(buf: String) {
    stdout().lock().write(buf.as_bytes()).unwrap();
}

fn init(args: &Cli, buf: String) -> String {
    let mut args = args.clone();
    match args {
        Cli {
            number_nonblank: true,
            number: true,
            ..
        } => {
            args.number = false;
            init(&args, buf)
        }
        Cli { show_all: true, .. } => {
            args.show_all = false;
            args.show_nonprinting = true;
            args.show_ends = true;
            args.show_tabs = true;
            init(&args, buf)
        }
        Cli { e: true, .. } => {
            args.e = false;
            args.show_ends = true;
            args.show_nonprinting = true;
            init(&args, buf)
        }
        Cli { t: true, .. } => {
            args.t = false;
            args.show_tabs = true;
            args.show_nonprinting = true;
            init(&args, buf)
        }
        Cli {
            squeeze_blank: true,
            ..
        } => {
            args.squeeze_blank = false;
            init(&args, squeeze_blank(&buf))
        }
        Cli {
            show_tabs: true, ..
        } => {
            args.show_tabs = false;
            init(&args, show_tabs(&buf))
        }
        Cli {
            show_ends: true, ..
        } => {
            args.show_ends = false;
            init(&args, show_ends(&buf))
        }
        Cli {
            show_nonprinting: true,
            ..
        } => {
            args.show_nonprinting = false;
            init(&args, show_nonprinting(&buf))
        }
        Cli {
            number_nonblank: true,
            ..
        } => {
            args.number_nonblank = false;
            init(&args, number_nonblank(&buf))
        }
        Cli { number: true, .. } => {
            args.number = false;
            init(&args, number(&buf))
        }
        _ => buf,
    }
}

fn show_nonprinting(read: &String) -> String {
    read.replace('\n', "$\n")
        .replace('\t', "^I")
        .replace('\0', "^@")
}

fn number(read: &String) -> String {
    let mut output = String::new();
    for line in read.lines() {
        output.push_str(&format!("{:6}\t{}", LINE_NUMBER.load(Relaxed), line));
        output.push('\n');
        LINE_NUMBER.fetch_add(1, Relaxed);
    }
    output
}

fn show_ends(read: &String) -> String {
    read.replace('\n', "$\n")
}

fn show_tabs(read: &String) -> String {
    read.replace('\t', "^I")
}

fn squeeze_blank(read: &String) -> String {
    let mut output = String::new();
    let mut last_line = String::new();
    for line in read.lines() {
        if !line.is_empty() {
            output.push_str(line);
            output.push('\n');
            last_line = line.to_string();
        } else if !last_line.is_empty() {
            output.push('\n');
            last_line = line.to_string();
        }
    }
    output
}

fn number_nonblank(read: &String) -> String {
    let mut output = String::new();
    for line in read.lines() {
        if line.is_empty() {
            output.push('\n');
            continue;
        }
        output.push_str(&format!("{:6}\t{}", LINE_NUMBER.load(Relaxed), line));
        LINE_NUMBER.fetch_add(1, Relaxed);
        output.push('\n');
    }
    output
}

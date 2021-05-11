use std::fs;
use std::path::PathBuf;
use structopt::clap::AppSettings;
use structopt::clap::Shell;
use structopt::StructOpt;

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
    generate_completions();

    match std::env::args().nth(1) {
        Some(contents) => contents,
        None => "No path given".to_string(),
    };

    std::env::args().nth(2);

    let args = Cli::from_args();

    let mut read = match std::fs::read_to_string(&args.path) {
        Ok(contents) => contents,
        Err(err) => err.to_string(),
    };

    if args.number && args.show_ends {
        println!("{}", show_ends(&mut number(&mut read)));
    } else if args.number {
        println!("{}", number(&mut read))
    } else if args.show_ends {
        println!("{}", show_ends(&mut read));
    } else if args.show_tabs {
        println!("{}", show_tabs(&mut read))
    } else if args.squeeze_blank {
        println!("{}", squeeze_blank(&mut read))
    } else if args.number_nonblank {
        println!("{}", number_nonblank(&mut read))
    } else {
        println!("{}", read)
    }
}

fn generate_completions() {
    match fs::read(std::env::current_exe().unwrap()) {
        Ok(..) => {}
        Err(..) => {
            Cli::clap().gen_completions(
                env!("CARGO_PKG_NAME"),
                Shell::Bash,
                std::env::current_exe().unwrap(),
            );
            Cli::clap().gen_completions(
                env!("CARGO_PKG_NAME"),
                Shell::Zsh,
                std::env::current_exe().unwrap(),
            );
            Cli::clap().gen_completions(
                env!("CARGO_PKG_NAME"),
                Shell::Fish,
                std::env::current_exe().unwrap(),
            );
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

fn spaces(digits: usize) -> String {
    let times = 6 - digits;
    let mut space = "".to_string();
    for _ in 0..times {
        space.push(' ');
    }
    space
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

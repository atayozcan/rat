use std::borrow::{Borrow, BorrowMut};
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
        for line in number(&mut read).lines() {
            println!("{}$", line)
        }
    } else if args.number {
        /*let mut i = 1;
        let digits = read.lines().count().to_string().len();
        let spaces = spaces(digits);
        for line in read.lines() {
            println!("{}{:d$}  {}", spaces, i, line, d = digits);
            i += 1;
        }*/
        println!("{}", number(&mut read))
    } else if args.show_ends {
        for line in read.lines() {
            println!("{}$", line)
        }
    } else {
        println!("{}", read)
    }
}

fn generate_completions() {
    match fs::read(std::env::current_exe().unwrap()) {
        Ok(..) => return,
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

fn spaces(digits: usize) -> String {
    match digits {
        1 => "     ".to_string(),
        2 => "    ".to_string(),
        3 => "   ".to_string(),
        4 => "  ".to_string(),
        5 => " ".to_string(),
        _ => "".to_string(),
    }
}

fn number(read: &mut String) -> String {
    let readd = read.clone();
    let mut i = 1;
    let digits = &read.lines().count().to_string().len();
    let spaces = spaces(*digits);
    let mut redd = String::new();
    for line in readd.lines() {
        redd.push_str(spaces.as_str());
        let s = format!("{:>d$}", &*i.to_string(), d = digits);
        redd.push_str(&*s);
        redd.push_str(" ");
        redd.push_str(line);
        redd.push_str("\n");

        i += 1;
    }

    redd
}

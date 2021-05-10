use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt)]
struct Cli {
	#[structopt(parse(from_os_str))]
	path: PathBuf,

	#[structopt(short = "c", long)]
	length: bool,
}

fn main() {
	let path = match std::env::args().nth(1){
		Some(contents)=> contents,
		None=>"No path given".to_string()
	};

	let flags = std::env::args().nth(2);

	let args = Cli::from_args();

	let read= match std::fs::read_to_string(&args.path){
		Ok(contents)=> contents,
		Err(err)=>{err.to_string()},
	};

	if args.length{
		println!("{}",read.len())
	} else { println!("{}",read) }
}
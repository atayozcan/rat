use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt)]
struct Cli {
	#[structopt(parse(from_os_str))]
	path: PathBuf,
}

fn main() {
	let args = Cli::from_args();

	let read= match std::fs::read_to_string(&args.path){
		Ok(contents)=> contents,
		Err(err)=>{err.to_string()},
	};

	println!("{}",read);
}
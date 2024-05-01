use oxiflex::{parse, run, Opt};
use std::process;
use structopt::StructOpt;

fn main() {
    let opt = Opt::from_args();

    if opt.parse {
        if let Err(e) = parse(opt) {
            eprintln!("Application error: {}", e);
            process::exit(1);
        }
        return;
    }

    if let Err(e) = run(opt) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

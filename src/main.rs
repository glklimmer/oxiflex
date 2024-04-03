use std::process;

use structopt::StructOpt;

fn main() {
    let opt = oxiflex::Opt::from_args();

    if let Err(e) = oxiflex::run(opt) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

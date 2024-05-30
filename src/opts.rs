use std::path::PathBuf;
use structopt::StructOpt;

/// FlatZinc solver
#[derive(StructOpt, Debug)]
#[structopt(name = "oxiflex")]
pub struct Opt {
    /// File to solve
    #[structopt(parse(from_os_str), default_value = "problems/simple/simple.fzn")]
    pub filename: PathBuf,

    /// Only parse using flatzinc
    #[structopt(short, long)]
    pub parse: bool,

    /// Use naive backtracking, e.g. no forward_checking
    #[structopt(short, long)]
    pub naive_backtracking: bool,
}

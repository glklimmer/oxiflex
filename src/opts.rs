use std::{path::PathBuf, u8};
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

    /// Use random order for variable assignments. Usually the goal is to fail early and assign
    /// variables that have the most constraints.
    #[structopt(short, long)]
    pub random_variable_order: bool,

    /// Use naive backtracking, e.g. no forward_checking
    #[structopt(short, long)]
    pub naive_backtracking: bool,

    /// Use forward checking as inference
    #[structopt(short, long)]
    pub forward_checking: bool,

    /// Specify arc consistency version
    #[structopt(short, long, default_value = "3")]
    pub arc_consistency: u8,
}

use std::error::Error;
use std::fs;

use std::path::PathBuf;

use structopt::StructOpt;

use flatzinc::{statement, *};

/// FlatZinc solver
#[derive(StructOpt, Debug)]
#[structopt(name = "oxiflex")]
pub struct Opt {
    /// File to solve
    #[structopt(parse(from_os_str))]
    filename: PathBuf,
}

pub fn run(opt: Opt) -> Result<(), Box<dyn Error>> {
    let buf = fs::read_to_string(opt.filename)?;

    for line in buf.lines() {
        match statement::<VerboseError<&str>>(line) {
            Ok((_, result)) => match result {
                Stmt::Comment(item) => println!("{:#?}", item),
                Stmt::Predicate(item) => println!("{:#?}", item),
                Stmt::Parameter(item) => println!("{:#?}", item),
                Stmt::Variable(item) => println!("variable: {:#?}", item),
                Stmt::Constraint(item) => println!("constraint: {:#?}", item),
                Stmt::SolveItem(item) => println!("{:#?}", item),
            },
            Err(Err::Error(e)) => {
                let error = convert_error(buf.as_str(), e);
                eprintln!("Failed to parse flatzinc!\n{}", error)
            }
            Err(e) => eprintln!("Failed to parse flatzinc: {:?}", e),
        }
    }

    Ok(())
}

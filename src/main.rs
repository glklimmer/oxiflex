use flatzinc::{statement, *};
use std::{io::Error, process};

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {err}");
        process::exit(1);
    }
}
fn run() -> Result<(), Error> {
    let opt = "problems/queens/queens.fzn";
    let buf = std::fs::read_to_string(opt)?;
    for line in buf.lines() {
        match statement::<VerboseError<&str>>(line) {
            Ok((_, result)) => println!("{:#?}", result),
            Err(Err::Error(e)) => {
                let bla = convert_error(buf.as_str(), e);
                eprintln!("Failed to parse flatzinc!\n{}", bla)
            }
            Err(e) => eprintln!("Failed to parse flatzinc: {:?}", e),
        }
    }
    Ok(())
}

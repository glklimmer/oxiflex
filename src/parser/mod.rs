pub mod utils;

use self::utils::{extract_par_id, extract_var_id, is_output};
use crate::{
    model::{var_id::VarId, Model},
    Opt,
};
use flatzinc::{Stmt, VarDeclItem};

use std::{collections::HashMap, error::Error, fs};

pub fn parse_and_print(opt: Opt) -> Result<(), Box<dyn Error>> {
    let buf = fs::read_to_string(opt.filename)?;

    for line in buf.lines() {
        match <Stmt as std::str::FromStr>::from_str(line) {
            Ok(result) => println!("{:#?}", result),
            Err(e) => {
                eprintln!("Failed to parse flatzinc statement:\n{}", e);
            }
        }
        println!()
    }

    Ok(())
}

type OutputAnnotations = (Vec<VarId>, HashMap<VarId, VarDeclItem>);

pub fn parse_to_model(opt: &Opt) -> Result<(Model, OutputAnnotations), Box<dyn Error>> {
    let buf = fs::read_to_string(&opt.filename)?;

    let mut parameters = HashMap::new();
    let mut variables = HashMap::new();
    let mut constraints = Vec::new();
    let mut output: Vec<VarId> = Vec::new();
    let mut output_arrays = HashMap::new();

    for line in buf.lines() {
        match <Stmt as std::str::FromStr>::from_str(line) {
            Ok(result) => match result {
                Stmt::Comment(_) => (),
                Stmt::Predicate(_) => (),
                Stmt::Parameter(item) => {
                    let (k, v) = extract_par_id(item);
                    parameters.insert(k, v);
                }
                Stmt::Variable(item) => {
                    let id = extract_var_id(&item);
                    if is_output(&item) {
                        output.push(id.clone());
                        if let VarDeclItem::ArrayOfInt { .. } = item {
                            output_arrays.insert(id, item);
                            continue;
                        }
                    }
                    variables.insert(id, item);
                }
                Stmt::Constraint(item) => constraints.push(item),
                Stmt::SolveItem(_) => (),
            },
            Err(e) => eprintln!("Failed to parse flatzinc: {:?}", e),
        }
    }

    Ok((
        Model::new(variables, &constraints, &parameters),
        (output, output_arrays),
    ))
}

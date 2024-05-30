use super::SearchResult;
use crate::model::{var_id::VarId, Model};
use flatzinc::*;
use std::collections::HashMap;

pub fn output_as_minizinc(
    result: SearchResult,
    model: &Model,
    output: Vec<VarId>,
    output_arrays: &HashMap<VarId, VarDeclItem>,
) {
    match result {
        SearchResult::Unknown => println!("=====UNKNOWN====="),
        SearchResult::Unbounded => println!("=====UNBOUNDED====="),
        SearchResult::Unsatisfiable => println!("=====UNSATISFIABLE====="),
        SearchResult::Assignment(assignments) => {
            for id in output {
                let variable = model.variables.get(&id);
                let variable = match variable {
                    Some(variable) => variable,
                    None => output_arrays.get(&id).expect("test"),
                };
                match variable {
                    VarDeclItem::ArrayOfInt { ix, array_expr, .. } => {
                        if let Option::Some(array_of_int_expr) = array_expr {
                            match array_of_int_expr {
                                ArrayOfIntExpr::Array(exprs) => {
                                    let values: Vec<String> = exprs
                                        .iter()
                                        .map(|expr| match expr {
                                            IntExpr::Int(_) => todo!(),
                                            IntExpr::VarParIdentifier(var_id) => assignments
                                                .get(&var_id.into())
                                                .expect("No value set.")
                                                .to_string(),
                                        })
                                        .collect();
                                    println!(
                                        "{} = array1d(1..{}, [{}]);",
                                        id,
                                        ix.0,
                                        values.join(", ")
                                    );
                                }
                                ArrayOfIntExpr::VarParIdentifier(_) => todo!(),
                            }
                        }
                    }
                    _ => {
                        println!("{} = {};", id, assignments.get(&id).expect("No value set."))
                    }
                }
            }
            println!("----------");
            println!("==========");
        }
    }
}

pub mod forward_checking;
pub mod naive_backtracking;
pub mod output;

use crate::{
    model::partial_assignment::PartialAssignment,
    parser::{parse_to_model, utils::extract_var_id},
    Opt,
};
use std::error::Error;

use {
    forward_checking::backtracking_with_forward_checking, naive_backtracking::naive_backtracking,
    output::output_as_minizinc,
};

#[derive(PartialEq)]
pub enum SearchResult {
    Unsatisfiable,
    Unbounded,
    Unknown,
    Assignment(PartialAssignment),
}

pub fn run(opt: Opt) -> Result<(), Box<dyn Error>> {
    // parse
    let (model, (output, output_arrays)) = parse_to_model(&opt)?;

    // solve
    let empty_assignment = PartialAssignment::new(
        model
            .variables
            .values()
            .map(|variable| (extract_var_id(variable), None))
            .collect(),
    );
    let result = if opt.naive_backtracking {
        naive_backtracking(&model, empty_assignment)
    } else {
        backtracking_with_forward_checking(&model, empty_assignment)
    };

    // output
    output_as_minizinc(result, &model, output, &output_arrays);

    Ok(())
}

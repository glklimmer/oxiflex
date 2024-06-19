use super::{partial_assignment::PartialAssignment, var_id::VarId};
use flatzinc::*;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct ConstraintConstruction;

impl fmt::Display for ConstraintConstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error while construction Constraint")
    }
}

impl std::error::Error for ConstraintConstruction {}

#[derive(Clone, Debug)]
pub enum Builtin {
    IntLinEq(Vec<i128>, Vec<VarId>, i128),
    IntLinLe(Vec<i128>, Vec<VarId>, i128),
    IntLinNe(Vec<i128>, Vec<VarId>, i128),
}

impl Builtin {
    pub fn involved_var_ids(&self) -> Vec<VarId> {
        match self {
            Builtin::IntLinEq(_, ids, _) => ids.clone(),
            Builtin::IntLinLe(_, ids, _) => ids.clone(),
            Builtin::IntLinNe(_, ids, _) => ids.clone(),
        }
    }

    pub fn from(
        constraint: &ConstraintItem,
        parameters: &HashMap<String, ParDeclItem>,
    ) -> Result<Self, ConstraintConstruction> {
        match constraint.id.as_str() {
            "int_lin_eq" => process_linear_constraint(constraint, parameters, Builtin::IntLinEq),
            "int_lin_le" => process_linear_constraint(constraint, parameters, Builtin::IntLinLe),
            "int_lin_ne" => process_linear_constraint(constraint, parameters, Builtin::IntLinNe),
            _ => Err(ConstraintConstruction),
        }
    }

    pub fn check(&self, alpha: &PartialAssignment) -> bool {
        match self {
            Builtin::IntLinEq(a_vec, b_vec, c) => {
                check_linear_constraint(a_vec, b_vec, c, alpha, |x, y| x == y)
            }
            Builtin::IntLinLe(a_vec, b_vec, c) => {
                check_linear_constraint(a_vec, b_vec, c, alpha, |x, y| x <= y)
            }
            Builtin::IntLinNe(a_vec, b_vec, c) => {
                check_linear_constraint(a_vec, b_vec, c, alpha, |x, y| x != y)
            }
        }
    }
}

fn process_linear_constraint(
    constraint: &ConstraintItem,
    parameters: &HashMap<String, ParDeclItem>,
    builtin_constructor: fn(Vec<i128>, Vec<VarId>, i128) -> Builtin,
) -> Result<Builtin, ConstraintConstruction> {
    let par_identifier = constraint.exprs[0].to_owned();
    let par_id = match par_identifier {
        Expr::VarParIdentifier(id) => id,
        _ => panic!("Only VarParIdentifier supported for linear constraints"),
    };

    let parameter = parameters
        .get(&par_id)
        .unwrap_or_else(|| panic!("Parameter {} not found", par_id));
    let par_data = match parameter {
        ParDeclItem::ArrayOfInt { v, .. } => v.to_owned(),
        _ => Vec::new(),
    };

    let var_expr = constraint.exprs[1].to_owned();
    let vars = match var_expr {
        Expr::ArrayOfBool(exprs) => exprs,
        _ => Vec::new(),
    };
    let var_ids = vars
        .iter()
        .map(|expr| match expr {
            BoolExpr::VarParIdentifier(id) => id.into(),
            _ => todo!("Only Bool Expr supported"),
        })
        .collect();

    let expr = constraint.exprs[2].to_owned();
    let c = match expr {
        Expr::Int(i) => i,
        _ => 0,
    };

    Ok(builtin_constructor(par_data, var_ids, c))
}

fn check_linear_constraint<F>(
    a_vec: &[i128],
    b_vec: &[VarId],
    c: &i128,
    alpha: &PartialAssignment,
    comparison: F,
) -> bool
where
    F: Fn(i128, i128) -> bool,
{
    assert!(a_vec.len() == 2, "Only binary constraints supported");
    assert!(b_vec.len() == 2, "Only binary constraints supported");

    let mut b_vec_iter = b_vec.iter();

    let u_key = b_vec_iter.next().unwrap();
    let u_assignment = alpha.get(u_key);
    let u = if let Some(value) = u_assignment {
        value
    } else {
        return true;
    };

    let v_key = b_vec_iter.next().unwrap();
    let v_assignment = alpha.get(v_key);
    let v = if let Some(value) = v_assignment {
        value
    } else {
        return true;
    };

    let a_1 = a_vec[0];
    let a_2 = a_vec[1];
    comparison(a_1 * u + a_2 * v, *c)
}

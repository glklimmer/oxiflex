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

#[derive(Clone)]
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
            "int_lin_eq" => {
                let par_identifier = constraint.exprs[0].to_owned();
                let par_id = if let Expr::VarParIdentifier(id) = par_identifier {
                    id
                } else {
                    panic!("Only VarParIdentifier supported for int_lin_eq")
                };

                let parameter = parameters
                    .get(&par_id)
                    .unwrap_or_else(|| panic!("Parameter {} not found", par_id));
                let par_data = if let ParDeclItem::ArrayOfInt { v, .. } = parameter {
                    v.to_owned()
                } else {
                    Vec::new()
                };

                let var_expr = constraint.exprs[1].to_owned();
                let vars = if let Expr::ArrayOfBool(exprs) = var_expr {
                    exprs
                } else {
                    Vec::new()
                };
                let var_ids = vars
                    .iter()
                    .map(|expr| {
                        if let BoolExpr::VarParIdentifier(id) = expr {
                            id.into()
                        } else {
                            todo!("Only Bool Expr supported");
                        }
                    })
                    .collect();

                let expr = constraint.exprs[2].to_owned();
                let c = if let Expr::Int(i) = expr { i } else { 0 };

                Ok(Builtin::IntLinEq(par_data, var_ids, c))
            }
            "int_lin_le" => {
                let par_identifier = constraint.exprs[0].to_owned();
                let par_id = if let Expr::VarParIdentifier(id) = par_identifier {
                    id
                } else {
                    panic!("Only VarParIdentifier supported for int_lin_le")
                };

                let parameter = parameters
                    .get(&par_id)
                    .unwrap_or_else(|| panic!("Parameter {} not found", par_id));
                let par_data = if let ParDeclItem::ArrayOfInt { v, .. } = parameter {
                    v.to_owned()
                } else {
                    Vec::new()
                };

                let var_expr = constraint.exprs[1].to_owned();
                let vars = if let Expr::ArrayOfBool(exprs) = var_expr {
                    exprs
                } else {
                    Vec::new()
                };
                let var_ids = vars
                    .iter()
                    .map(|expr| {
                        if let BoolExpr::VarParIdentifier(id) = expr {
                            id.into()
                        } else {
                            todo!("Only Bool Expr supported");
                        }
                    })
                    .collect();

                let expr = constraint.exprs[2].to_owned();
                let c = if let Expr::Int(i) = expr { i } else { 0 };

                Ok(Builtin::IntLinLe(par_data, var_ids, c))
            }
            "int_lin_ne" => {
                let par_identifier = constraint.exprs[0].to_owned();
                let par_id = if let Expr::VarParIdentifier(id) = par_identifier {
                    id
                } else {
                    panic!("Only VarParIdentifier supported for int_lin_ne")
                };

                let parameter = parameters
                    .get(&par_id)
                    .unwrap_or_else(|| panic!("Parameter {} not found", par_id));
                let par_data = if let ParDeclItem::ArrayOfInt { v, .. } = parameter {
                    v.to_owned()
                } else {
                    Vec::new()
                };

                let var_expr = constraint.exprs[1].to_owned();
                let vars = if let Expr::ArrayOfBool(exprs) = var_expr {
                    exprs
                } else {
                    Vec::new()
                };
                let var_ids = vars
                    .iter()
                    .map(|expr| {
                        if let BoolExpr::VarParIdentifier(id) = expr {
                            id.into()
                        } else {
                            todo!("Only Bool Expr supported");
                        }
                    })
                    .collect();

                let expr = constraint.exprs[2].to_owned();
                let c = if let Expr::Int(i) = expr { i } else { 0 };

                Ok(Builtin::IntLinNe(par_data, var_ids, c))
            }
            _ => Err(ConstraintConstruction),
        }
    }

    // check (v, u, c, alpha)
    pub fn check(&self, alpha: &PartialAssignment) -> bool {
        match self {
            Builtin::IntLinEq(a_vec, b_vec, c) => {
                assert!(a_vec.len() == 2, "Only binary constraints supported");
                assert!(b_vec.len() == 2, "Only binary constraints supported");

                let mut b_vec_iter = b_vec.iter();

                let u_key = b_vec_iter.next().unwrap();
                let u_assignment = alpha.get(u_key);
                let u = if let Some(value) = u_assignment {
                    value
                } else {
                    // println!("{u_key} not set, check: true");
                    return true;
                };

                let v_key = b_vec_iter.next().unwrap();
                let v_assignment = alpha.get(v_key);
                let v = if let Some(value) = v_assignment {
                    value
                } else {
                    // println!("{v_key} not set, check: true");
                    return true;
                };

                // println!("----------");
                // println!("checking: {u_key}, {v_key}, int_lin_eq");
                // println!("{alpha}");
                // println!("c == a_1 * u + a_2 * v");
                let a_1 = a_vec[0];
                let a_2 = a_vec[1];
                let r = *c == a_1 * u + a_2 * v;
                // println!("{c} == {a_1} * {u} + {a_2} * {v}, check: {r}");
                if r {
                    return true;
                }
                false
            }
            Builtin::IntLinLe(a_vec, b_vec, c) => {
                assert!(a_vec.len() == 2, "Only binary constraints supported");
                assert!(b_vec.len() == 2, "Only binary constraints supported");

                let mut b_vec_iter = b_vec.iter();

                let u_key = b_vec_iter.next().unwrap();
                let u_assignment = alpha.get(u_key);
                let u = if let Some(value) = u_assignment {
                    value
                } else {
                    // println!("{u_key} not set, check: true");
                    return true;
                };

                let v_key = b_vec_iter.next().unwrap();
                let v_assignment = alpha.get(v_key);
                let v = if let Some(value) = v_assignment {
                    value
                } else {
                    // println!("{v_key} not set, check: true");
                    return true;
                };

                // println!("----------");
                // println!("checking: {u_key}, {v_key}, int_lin_le");
                // println!("{alpha}");
                // println!("c >= a_1 * u + a_2 * v");
                let a_1 = a_vec[0];
                let a_2 = a_vec[1];
                let r = *c >= a_1 * u + a_2 * v;
                // println!("{c} >= {a_1} * {u} + {a_2} * {v}, check: {r}");
                if r {
                    return true;
                }
                false
            }
            Builtin::IntLinNe(a_vec, b_vec, c) => {
                assert!(a_vec.len() == 2, "Only binary constraints supported");
                assert!(b_vec.len() == 2, "Only binary constraints supported");

                let mut b_vec_iter = b_vec.iter();

                let u_key = b_vec_iter.next().unwrap();
                let u_assignment = alpha.get(u_key);
                let u = if let Some(value) = u_assignment {
                    value
                } else {
                    // println!("{u_key} not set, check: true");
                    return true;
                };

                let v_key = b_vec_iter.next().unwrap();
                let v_assignment = alpha.get(v_key);
                let v = if let Some(value) = v_assignment {
                    value
                } else {
                    // println!("{v_key} not set, check: true");
                    return true;
                };

                // println!("----------");
                // println!("checking: {u_key}, {v_key}, int_lin_ne");
                // println!("{alpha}");
                // println!("c >= a_1 * u + a_2 * v");
                let a_1 = a_vec[0];
                let a_2 = a_vec[1];
                let r = *c != a_1 * u + a_2 * v;
                // println!("{c} != {a_1} * {u} + {a_2} * {v}, check: {r}");
                if r {
                    return true;
                }
                false
            }
        }
    }
}

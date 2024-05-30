pub mod constraint;
pub mod domain;
pub mod partial_assignment;
pub mod var_id;

use crate::parser::utils::extract_var_id;
use flatzinc::*;
use std::{collections::HashMap, rc::Rc};
use {constraint::Builtin, domain::Domain, partial_assignment::PartialAssignment, var_id::VarId};

#[derive(Clone)]
pub struct Model {
    pub variables: HashMap<VarId, VarDeclItem>,
    domains: HashMap<VarId, Domain>,
    constraints: Vec<Rc<Builtin>>,
    index: HashMap<VarId, Vec<Rc<Builtin>>>,
}

impl Model {
    pub fn new(
        variables: HashMap<VarId, VarDeclItem>,
        constraints: &[ConstraintItem],
        parameters: &HashMap<String, ParDeclItem>,
    ) -> Self {
        let mut constraint_vec = Vec::new();
        let mut index: HashMap<VarId, Vec<Rc<Builtin>>> = HashMap::new();

        for constraint in constraints.iter() {
            if let Ok(builtin) = Builtin::from(constraint, parameters) {
                let rc_builtin = Rc::new(builtin);
                for var_id in rc_builtin.involved_var_ids() {
                    index.entry(var_id).or_default().push(rc_builtin.clone());
                }
                constraint_vec.push(rc_builtin);
            }
        }

        let domains: HashMap<VarId, Domain> = variables
            .iter()
            .map(|(id, variable)| {
                let range = match variable {
                    VarDeclItem::Bool { .. } => 0..=1,
                    VarDeclItem::Int { .. } => i128::MIN..=i128::MAX,
                    VarDeclItem::IntInRange { lb, ub, .. } => *lb..=*ub,
                    _ => todo!(),
                };
                (id.clone(), range.collect())
            })
            .collect();

        Self {
            variables,
            domains,
            constraints: constraint_vec,
            index,
        }
    }

    pub fn is_inconsistent(&self, alpha: &PartialAssignment) -> bool {
        self.constraints
            .iter()
            .any(|constraint| !constraint.check(alpha))
    }

    pub fn domains_available(&self) -> bool {
        self.variables.iter().all(|v| {
            let id = extract_var_id(v.1);
            if let Some(domain) = self.domains.get(&id) {
                if !domain.is_empty() {
                    return true;
                }
            }
            false
        })
    }

    pub fn dom(&self, var_id: &VarId) -> &Domain {
        self.domains.get(var_id).unwrap()
    }

    pub fn forward_checking(&mut self, alpha: &PartialAssignment) {
        for unassigned_id in alpha.unassigned_variables() {
            let domain = self.domains.get_mut(unassigned_id).unwrap();

            if let Some(constraints) = self.index.get(unassigned_id) {
                for constraint in constraints.iter() {
                    domain.retain(|&possible_value| {
                        constraint.check(&alpha.union(unassigned_id, possible_value))
                    });
                }
            }
        }
    }
}

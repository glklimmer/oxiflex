pub mod constraint;
pub mod domain;
pub mod partial_assignment;
pub mod var_id;

use crate::parser::utils::extract_var_id;
use flatzinc::*;
use std::{
    collections::{HashMap, VecDeque},
    rc::Rc,
};
use {constraint::Builtin, domain::Domain, partial_assignment::PartialAssignment, var_id::VarId};

#[derive(Clone)]
pub struct Model {
    pub variables: HashMap<VarId, VarDeclItem>,
    domains: HashMap<VarId, Domain>,
    constraints: Vec<Rc<Builtin>>,
    constraint_index: HashMap<VarId, Vec<Rc<Builtin>>>,
}

impl Model {
    pub fn new(
        variables: HashMap<VarId, VarDeclItem>,
        constraints: &[ConstraintItem],
        parameters: &HashMap<String, ParDeclItem>,
    ) -> Self {
        let mut constraint_vec = Vec::new();
        let mut constraint_index: HashMap<VarId, Vec<Rc<Builtin>>> = HashMap::new();

        for constraint in constraints.iter() {
            if let Ok(builtin) = Builtin::from(constraint, parameters) {
                let rc_builtin = Rc::new(builtin);
                for var_id in rc_builtin.involved_var_ids() {
                    constraint_index
                        .entry(var_id)
                        .or_default()
                        .push(rc_builtin.clone());
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
            constraint_index,
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

            if let Some(constraints) = self.constraint_index.get(unassigned_id) {
                for constraint in constraints {
                    domain.retain(|&possible_value| {
                        constraint.check(&alpha.union(unassigned_id, possible_value))
                    });
                }
            }
        }
    }

    pub fn arc_consistency_1(&mut self) {
        let mut domain_changed = false;
        loop {
            for constraint in self.constraints.clone() {
                let mut var_ids = constraint.involved_var_ids();
                let v = var_ids.pop().unwrap();
                let u = var_ids.pop().unwrap();
                // println!("revise: {u}, {v}");
                let first_changed = self.revise(&u, &v);
                // println!("revise: {v}, {u}");
                let second_changed = self.revise(&v, &u);
                domain_changed = first_changed || second_changed;
                // println!("any changed: {domain_changed}");
                // println!("==========");
            }
            // println!("constraints checked, change: {domain_changed}");

            if !domain_changed {
                // println!("stopping arc consistency");
                return;
            }
        }
    }

    pub fn arc_consistency_3(&mut self) {
        let mut queue = VecDeque::new();
        for constraint in self.constraints.clone() {
            let mut var_ids = constraint.involved_var_ids();
            let v = var_ids.pop().unwrap();
            let u = var_ids.pop().unwrap();

            queue.push_back((u.clone(), v.clone()));
            queue.push_back((v, u));
        }
        while !queue.is_empty() {
            let (u, v) = queue.pop_back().unwrap();
            let changed = self.revise(&u, &v);
            if changed {
                for constraint in self.constraint_index.get(&u).unwrap() {
                    let mut var_ids = constraint.involved_var_ids();
                    let first = var_ids.pop().unwrap();
                    let other = var_ids.pop().unwrap();
                    if u == first {
                        queue.push_back((other, first));
                    } else {
                        queue.push_back((first, other));
                    }
                }
            }
        }
    }

    fn revise(&mut self, v: &VarId, v_prime: &VarId) -> bool {
        let mut domain = self.domains.remove(v).unwrap();
        let domain_len = domain.len();

        let changed = match self.constraint_index.get(v) {
            Some(constraints) => {
                for constraint in constraints {
                    // println!("current {:?}", domain);
                    domain.retain(|&d| {
                        self.domains
                            .get(v_prime)
                            .unwrap()
                            .into_iter()
                            .any(|&d_prime| {
                                let mut variables = HashMap::new();
                                variables.insert(v.clone(), Some(d));
                                variables.insert(v_prime.clone(), Some(d_prime));
                                constraint.check(&PartialAssignment::new(variables))
                                // println!(
                                //     "checking: {} = {}, {} = {}, {:?} = {}",
                                //     v, d, v_prime, d_prime, constraint, check_result
                                // );
                            })
                    });
                    // println!("new {:?}", domain);
                }
                domain.len() != domain_len
            }
            None => false,
        };

        // println!("changed: {changed}");
        // println!("----------");
        self.domains.insert(v.clone(), domain);
        changed
    }
}

use std::collections::{HashMap, VecDeque};

use crate::model::Model;

use super::{constraint::Pair, partial_assignment::PartialAssignment, var_id::VarId};

impl Model {
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
                                // let mut variables = HashMap::new();
                                // variables.insert(v.clone(), Some(d));
                                // variables.insert(v_prime.clone(), Some(d_prime));

                                let pair = Pair {
                                    u: v.clone(),
                                    u_value: d,
                                    v: v_prime.clone(),
                                    v_value: d_prime,
                                };

                                constraint.check_pair(pair)
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

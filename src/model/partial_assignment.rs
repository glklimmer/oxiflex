use super::{var_id::VarId, Model};
use std::{
    cmp::Ordering::{Equal, Greater, Less},
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

#[derive(Clone, PartialEq)]
pub struct PartialAssignment(HashMap<VarId, Option<i128>>);

impl Display for PartialAssignment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (id, assignment) in &self.0 {
            match assignment {
                Some(v) => writeln!(f, "{} = {};", id, v)?,
                None => writeln!(f, "{}: unset", id)?,
            }
        }
        Ok(())
    }
}

impl PartialAssignment {
    pub fn new(variables: HashMap<VarId, Option<i128>>) -> Self {
        PartialAssignment(variables)
    }

    pub fn get(&self, var_id: &VarId) -> Option<i128> {
        match self.0.get(var_id) {
            Some(value) => *value,
            None => Option::None,
        }
    }

    pub fn union(&self, id: &VarId, value: i128) -> PartialAssignment {
        let mut alpha_prime = self.clone();
        alpha_prime.0.insert(id.clone(), Some(value));
        alpha_prime
    }

    pub fn is_total_assignment(&self) -> bool {
        self.0
            .iter()
            .all(|assignment_entry| assignment_entry.1.is_some())
    }

    pub fn find_any_unassigned(&self) -> &VarId {
        self.0
            .iter()
            .find(|assignment_entry| assignment_entry.1.is_none())
            .unwrap()
            .0
    }

    pub fn find_resticting_unassigned(&self, model: &Model) -> &VarId {
        self.0
            .iter()
            .filter(|assignment_entry| assignment_entry.1.is_none())
            .map(|assignment_entry| {
                (
                    assignment_entry.0,
                    model.constraint_amount(assignment_entry.0),
                )
            })
            .max_by(|x, y| match x.1.cmp(&y.1) {
                Less => Less,
                Greater => Greater,
                Equal => x.0.cmp(y.0),
            })
            .unwrap()
            .0
    }

    pub fn unassigned_variables(&self) -> impl Iterator<Item = &VarId> {
        self.0
            .iter()
            .filter(|(_, opt)| opt.is_none())
            .map(|(var_id, _)| var_id)
    }
}

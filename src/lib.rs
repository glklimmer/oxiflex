use core::panic;
use flatzinc::{statement, *};
use std::ops::RangeInclusive;
use std::path::PathBuf;
use std::{collections::HashMap, error::Error};
use std::{fmt, fs, i128};
use structopt::StructOpt;

/// FlatZinc solver
#[derive(StructOpt, Debug)]
#[structopt(name = "oxiflex")]
pub struct Opt {
    /// File to solve
    #[structopt(parse(from_os_str), default_value = "problems/simple/simple.fzn")]
    filename: PathBuf,
}

#[derive(Clone, PartialEq)]
struct Assignment<'a> {
    variable: &'a VarDeclItem,
    value: Option<i128>,
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
struct VarId(String);

impl From<&String> for VarId {
    fn from(item: &String) -> Self {
        VarId(item.clone())
    }
}

#[derive(Clone, PartialEq)]
struct PartialAssignment<'a>(HashMap<VarId, Assignment<'a>>);

impl<'a> fmt::Display for PartialAssignment<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (id, assignment) in &self.0 {
            match assignment.value {
                Some(v) => writeln!(f, "{} = {};", id.0, v)?,
                None => writeln!(f, "{}: unset", id.0)?,
            }
        }
        Ok(())
    }
}

struct Model {
    variables: HashMap<VarId, VarDeclItem>,
    constraints: Vec<Builtin>,
}

impl Model {
    fn new(
        variables: HashMap<VarId, VarDeclItem>,
        constraints: &[ConstraintItem],
        parameters: &HashMap<String, ParDeclItem>,
    ) -> Self {
        let constraints = constraints
            .iter()
            .filter_map(|constraint| Builtin::from(constraint, &variables, parameters).ok())
            .collect();

        Self {
            variables,
            constraints,
        }
    }

    fn dom(variable: &VarDeclItem) -> RangeInclusive<i128> {
        match variable {
            VarDeclItem::Bool { .. } => 0..=1,
            VarDeclItem::Int { .. } => i128::MIN..=i128::MAX,
            VarDeclItem::IntInRange { lb, ub, .. } => *lb..=*ub,
            _ => todo!(),
        }
    }
}

enum Builtin {
    IntLinEq(Vec<i128>, Vec<String>, i128), // TODO: Replace String key with VarId
    IntLinLe(Vec<i128>, Vec<String>, i128), // TODO: Replace String key with VarId
}

impl Builtin {
    fn from(
        constraint: &ConstraintItem,
        variables: &HashMap<VarId, VarDeclItem>,
        parameters: &HashMap<String, ParDeclItem>,
    ) -> Result<Self, ()> {
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
                            if variables.get(&id.into()).is_none() {
                                panic!("Variable {} not found", id);
                            }
                            id.to_owned()
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
                            if variables.get(&id.into()).is_none() {
                                panic!("Variable {} not found", id);
                            }
                            id.to_owned()
                        } else {
                            todo!("Only Bool Expr supported");
                        }
                    })
                    .collect();

                let expr = constraint.exprs[2].to_owned();
                let c = if let Expr::Int(i) = expr { i } else { 0 };

                Ok(Builtin::IntLinLe(par_data, var_ids, c))
            }
            _ => Err(()),
        }
    }

    // check (v, u, c, alpha)
    fn check(&self, model: &Model, alpha: &PartialAssignment) -> bool {
        match self {
            Builtin::IntLinEq(a_vec, b_vec, c) => {
                assert!(a_vec.len() == 2, "Only binary constraints supported");
                assert!(b_vec.len() == 2, "Only binary constraints supported");

                let mut b_vec_iter = b_vec.iter();

                let u_key = b_vec_iter.next().unwrap();
                let u_assignment = alpha.0.get(&u_key.into()).expect("Variable not found");
                let u_range = if let Some(value) = u_assignment.value {
                    value..=value
                } else {
                    // println!("{u_key} not set, check: true");
                    return true;
                };

                let v_key = b_vec_iter.next().unwrap();
                let v_assignment = alpha.0.get(&v_key.into()).expect("Variable not found");
                let v_range = if let Some(value) = v_assignment.value {
                    value..=value
                } else {
                    // println!("{v_key} not set, check: true");
                    return true;
                };

                // println!("----------");
                // println!("checking: {u_key}, {v_key}, int_lin_eq");
                // println!("{alpha}");
                // println!("c == a_1 * u + a_2 * v");
                for (u, v) in u_range.zip(v_range) {
                    let a_1 = a_vec[0];
                    let a_2 = a_vec[1];
                    let r = *c == a_1 * u + a_2 * v;
                    // println!("{c} == {a_1} * {u} + {a_2} * {v}, check: {r}");
                    if r {
                        return true;
                    }
                }
                false
            }
            Builtin::IntLinLe(a_vec, b_vec, c) => {
                assert!(a_vec.len() == 2, "Only binary constraints supported");
                assert!(b_vec.len() == 2, "Only binary constraints supported");

                let mut b_vec_iter = b_vec.iter();

                let u_key = b_vec_iter.next().unwrap();
                let u_assignment = alpha.0.get(&u_key.into()).expect("Variable not found");
                let u_range = if let Some(value) = u_assignment.value {
                    value..=value
                } else {
                    // println!("{u_key} not set, check: true");
                    return true;
                };

                let v_key = b_vec_iter.next().unwrap();
                let v_assignment = alpha.0.get(&v_key.into()).expect("Variable not found");
                let v_range = if let Some(value) = v_assignment.value {
                    value..=value
                } else {
                    // println!("{v_key} not set, check: true");
                    return true;
                };

                // println!("----------");
                // println!("checking: {u_key}, {v_key}, int_lin_le");
                // println!("{alpha}");
                // println!("c >= a_1 * u + a_2 * v");
                for (u, v) in u_range.zip(v_range) {
                    let a_1 = a_vec[0];
                    let a_2 = a_vec[1];
                    let r = *c >= a_1 * u + a_2 * v;
                    // println!("{c} >= {a_1} * {u} + {a_2} * {v}, check: {r}");
                    if r {
                        return true;
                    }
                }
                false
            }
        }
    }
}

#[derive(PartialEq)]
enum SearchResult<'a> {
    Unsatisfiable,
    Unbounded,
    Unknown,
    Assignment(PartialAssignment<'a>),
}

pub fn run(opt: Opt) -> Result<(), Box<dyn Error>> {
    let buf = fs::read_to_string(opt.filename)?;

    let mut parameters = HashMap::new();
    let mut variables = HashMap::new();
    let mut constraints = Vec::new();

    for line in buf.lines() {
        match statement::<VerboseError<&str>>(line) {
            Ok((_, result)) => match result {
                Stmt::Comment(_) => (),
                Stmt::Predicate(_) => (),
                Stmt::Parameter(item) => {
                    let (k, v) = extract_par_id(item);
                    parameters.insert(k, v);
                }
                Stmt::Variable(item) => {
                    let id = extract_var_id(&item);
                    variables.insert(id, item);
                }
                Stmt::Constraint(item) => constraints.push(item),
                Stmt::SolveItem(_) => (),
            },
            Err(Err::Error(e)) => {
                let error = convert_error(buf.as_str(), e);
                eprintln!("Failed to parse flatzinc!\n{}", error)
            }
            Err(e) => eprintln!("Failed to parse flatzinc: {:?}", e),
        }
    }

    let model = Model::new(variables, &constraints, &parameters);

    // solve
    let empty_assignment = PartialAssignment(
        model
            .variables
            .values()
            .map(|v| Assignment {
                variable: v,
                value: None,
            })
            .map(|assignment| (extract_var_id(assignment.variable), assignment))
            .collect(),
    );
    let result = naive_backtracking(&model, empty_assignment);

    // output
    match result {
        SearchResult::Unknown => println!("=====UNKNOWN====="),
        SearchResult::Unbounded => println!("=====UNBOUNDED====="),
        SearchResult::Unsatisfiable => println!("=====UNSATISFIABLE====="),
        SearchResult::Assignment(assignments) => {
            println!("{assignments}");
            println!("----------");
            println!("==========");
        }
    }

    Ok(())
}

fn extract_par_id(item: ParDeclItem) -> (String, ParDeclItem) {
    let id = match &item {
        ParDeclItem::Bool { id, .. } => id.clone(),
        ParDeclItem::Int { id, .. } => id.clone(),
        ParDeclItem::Float { id, .. } => id.clone(),
        ParDeclItem::SetOfInt { id, .. } => id.clone(),
        ParDeclItem::ArrayOfBool { id, .. } => id.clone(),
        ParDeclItem::ArrayOfInt { id, .. } => id.clone(),
        ParDeclItem::ArrayOfFloat { id, .. } => id.clone(),
        ParDeclItem::ArrayOfSet { id, .. } => id.clone(),
    };
    (id, item)
}

fn extract_var_id(item: &VarDeclItem) -> VarId {
    match &item {
        VarDeclItem::Bool { id, .. } => id.into(),
        VarDeclItem::Int { id, .. } => id.into(),
        VarDeclItem::IntInRange { id, .. } => id.into(),
        VarDeclItem::IntInSet { id, .. } => id.into(),
        VarDeclItem::Float { id, .. } => id.into(),
        VarDeclItem::BoundedFloat { id, .. } => id.into(),
        VarDeclItem::SetOfInt { id, .. } => id.into(),
        VarDeclItem::SubSetOfIntSet { id, .. } => id.into(),
        VarDeclItem::SubSetOfIntRange { id, .. } => id.into(),
        VarDeclItem::ArrayOfBool { id, .. } => id.into(),
        VarDeclItem::ArrayOfInt { id, .. } => id.into(),
        VarDeclItem::ArrayOfIntInRange { id, .. } => id.into(),
        VarDeclItem::ArrayOfIntInSet { id, .. } => id.into(),
        VarDeclItem::ArrayOfFloat { id, .. } => id.into(),
        VarDeclItem::ArrayOfBoundedFloat { id, .. } => id.into(),
        VarDeclItem::ArrayOfSet { id, .. } => id.into(),
        VarDeclItem::ArrayOfSubSetOfIntRange { id, .. } => id.into(),
        VarDeclItem::ArrayOfSubSetOfIntSet { id, .. } => id.into(),
    }
}

fn naive_backtracking<'a>(model: &'a Model, alpha: PartialAssignment<'a>) -> SearchResult<'a> {
    // if α is inconsistent with C:
    // // return inconsistent
    if model
        .constraints
        .iter()
        .any(|constraint| !constraint.check(model, &alpha))
    {
        return SearchResult::Unsatisfiable;
    }

    // if α is a total assignment:
    // // return α
    if alpha
        .0
        .iter()
        .all(|assignment_entry| assignment_entry.1.value.is_some())
    {
        return SearchResult::Assignment(alpha);
    }

    // select some variable v for which α is not defined
    let v = alpha
        .0
        .iter()
        .find(|assignment_entry| assignment_entry.1.value.is_none());
    let v = if let Some(assignment) = v {
        assignment.1.variable
    } else {
        return SearchResult::Unsatisfiable;
    };

    // for each d ∈ dom(v ) in some order:
    for d in Model::dom(v) {
        // // α′ := α ∪ {v 7→ d}
        let mut alpha_prime = alpha.clone();
        alpha_prime.0.insert(
            extract_var_id(v),
            Assignment {
                variable: v,
                value: Some(d),
            },
        );
        // // α′′ := NaiveBacktracking(C, α′ )
        let alpha_prime_prime = naive_backtracking(model, alpha_prime);

        // // if α′′ ̸= inconsistent:
        if alpha_prime_prime != SearchResult::Unsatisfiable {
            // // // return α′′
            return alpha_prime_prime;
        }
    }

    // return inconsistent
    SearchResult::Unsatisfiable
}

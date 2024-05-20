use core::panic;
use flatzinc::{statement, *};
use std::fmt::{Display, Formatter};
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

    /// Only parse using flatzinc
    #[structopt(short, long)]
    pub parse: bool,
}

#[derive(Clone, PartialEq)]
struct Assignment {
    var_id: VarId,
    value: Option<i128>,
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
struct VarId(String);

impl From<&String> for VarId {
    fn from(item: &String) -> Self {
        VarId(item.clone())
    }
}

impl From<String> for VarId {
    fn from(item: String) -> Self {
        VarId(item.clone())
    }
}

impl Display for VarId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}

#[derive(Clone, PartialEq)]
struct PartialAssignment(HashMap<VarId, Option<i128>>);

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

#[derive(Clone)]
struct Model {
    variables: HashMap<VarId, VarDeclItem>,
    domains: HashMap<VarId, Vec<i128>>,
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
            domains: HashMap::new(),
            constraints,
        }
    }

    fn domains_available(&self) -> bool {
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

    fn dom(&self, var_id: &VarId) -> Box<dyn Iterator<Item = i128> + '_> {
        if let Some(domain) = self.domains.get(&var_id) {
            if !domain.is_empty() {
                return Box::new(domain.iter().cloned());
            }
        }

        let variable = self.variables.get(&var_id).expect("Variable not found.");

        let range = match variable {
            VarDeclItem::Bool { .. } => 0..=1,
            VarDeclItem::Int { .. } => i128::MIN..=i128::MAX,
            VarDeclItem::IntInRange { lb, ub, .. } => *lb..=*ub,
            _ => todo!(),
        };

        Box::new(range)
    }

    fn forward_checking(&mut self) {
        self.variables.remove(&VarId("test".to_string()));
    }
}

#[derive(Clone)]
enum Builtin {
    IntLinEq(Vec<i128>, Vec<String>, i128), // TODO: Replace String key with VarId
    IntLinLe(Vec<i128>, Vec<String>, i128), // TODO: Replace String key with VarId
    IntLinNe(Vec<i128>, Vec<String>, i128), // TODO: Replace String key with VarId
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

                Ok(Builtin::IntLinNe(par_data, var_ids, c))
            }
            _ => Err(()),
        }
    }

    // check (v, u, c, alpha)
    fn check(&self, alpha: &PartialAssignment) -> bool {
        match self {
            Builtin::IntLinEq(a_vec, b_vec, c) => {
                assert!(a_vec.len() == 2, "Only binary constraints supported");
                assert!(b_vec.len() == 2, "Only binary constraints supported");

                let mut b_vec_iter = b_vec.iter();

                let u_key = b_vec_iter.next().unwrap();
                let u_assignment = alpha.0.get(&u_key.into()).expect("Variable not found");
                let u = if let Some(value) = u_assignment {
                    value
                } else {
                    // println!("{u_key} not set, check: true");
                    return true;
                };

                let v_key = b_vec_iter.next().unwrap();
                let v_assignment = alpha.0.get(&v_key.into()).expect("Variable not found");
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
                let u_assignment = alpha.0.get(&u_key.into()).expect("Variable not found");
                let u = if let Some(value) = u_assignment {
                    value
                } else {
                    // println!("{u_key} not set, check: true");
                    return true;
                };

                let v_key = b_vec_iter.next().unwrap();
                let v_assignment = alpha.0.get(&v_key.into()).expect("Variable not found");
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
                let u_assignment = alpha.0.get(&u_key.into()).expect("Variable not found");
                let u = if let Some(value) = u_assignment {
                    value
                } else {
                    // println!("{u_key} not set, check: true");
                    return true;
                };

                let v_key = b_vec_iter.next().unwrap();
                let v_assignment = alpha.0.get(&v_key.into()).expect("Variable not found");
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

#[derive(PartialEq)]
enum SearchResult {
    Unsatisfiable,
    Unbounded,
    Unknown,
    Assignment(PartialAssignment),
}

pub fn parse(opt: Opt) -> Result<(), Box<dyn Error>> {
    let buf = fs::read_to_string(opt.filename)?;

    for line in buf.lines() {
        match statement::<VerboseError<&str>>(line) {
            Ok((_, result)) => println!("{:#?}", result),
            Err(Err::Error(e)) => {
                let error = convert_error(buf.as_str(), e);
                eprintln!("Failed to parse flatzinc!\n{}", error)
            }
            Err(e) => eprintln!("Failed to parse flatzinc: {:?}", e),
        }
        println!()
    }

    Ok(())
}

pub fn run(opt: Opt) -> Result<(), Box<dyn Error>> {
    let buf = fs::read_to_string(opt.filename)?;

    let mut parameters = HashMap::new();
    let mut variables = HashMap::new();
    let mut constraints = Vec::new();
    let mut output: Vec<VarId> = Vec::new();
    let mut output_arrays = HashMap::new();

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
            .map(|variable| (extract_var_id(variable), None))
            .collect(),
    );
    // let result = naive_backtracking(&model, empty_assignment);
    let result = backtracking_with_forward_checking(&model, empty_assignment);

    // output
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
                                                .0
                                                .get(&var_id.into())
                                                .expect("Variable not found in assignments.")
                                                .expect("No value for variable found.")
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
                        println!(
                            "{} = {};",
                            id,
                            assignments
                                .0
                                .get(&id)
                                .expect("No value for variable found")
                                .expect("No value for variable found.")
                        )
                    }
                }
            }
            println!("----------");
            println!("==========");
        }
    }

    Ok(())
}

fn is_output(item: &VarDeclItem) -> bool {
    let annos = match item {
        VarDeclItem::Bool { annos, .. } => annos,
        VarDeclItem::Int { annos, .. } => annos,
        VarDeclItem::IntInRange { annos, .. } => annos,
        VarDeclItem::IntInSet { annos, .. } => annos,
        VarDeclItem::Float { annos, .. } => annos,
        VarDeclItem::BoundedFloat { annos, .. } => annos,
        VarDeclItem::SetOfInt { annos, .. } => annos,
        VarDeclItem::SubSetOfIntSet { annos, .. } => annos,
        VarDeclItem::SubSetOfIntRange { annos, .. } => annos,
        VarDeclItem::ArrayOfBool { annos, .. } => annos,
        VarDeclItem::ArrayOfInt { annos, .. } => annos,
        VarDeclItem::ArrayOfIntInRange { annos, .. } => annos,
        VarDeclItem::ArrayOfIntInSet { annos, .. } => annos,
        VarDeclItem::ArrayOfFloat { annos, .. } => annos,
        VarDeclItem::ArrayOfBoundedFloat { annos, .. } => annos,
        VarDeclItem::ArrayOfSet { annos, .. } => annos,
        VarDeclItem::ArrayOfSubSetOfIntRange { annos, .. } => annos,
        VarDeclItem::ArrayOfSubSetOfIntSet { annos, .. } => annos,
    };
    annos
        .iter()
        .any(|anno| anno.id == "output_array" || anno.id == "output_var")
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

fn naive_backtracking(model: &Model, alpha: PartialAssignment) -> SearchResult {
    // if α is inconsistent with C:
    // // return inconsistent
    if model
        .constraints
        .iter()
        .any(|constraint| !constraint.check(&alpha))
    {
        return SearchResult::Unsatisfiable;
    }

    // if α is a total assignment:
    // // return α
    if alpha
        .0
        .iter()
        .all(|assignment_entry| assignment_entry.1.is_some())
    {
        return SearchResult::Assignment(alpha);
    }

    // select some variable v for which α is not defined
    let v = alpha
        .0
        .iter()
        .find(|assignment_entry| assignment_entry.1.is_none());
    let v = if let Some(assignment) = v {
        assignment.0
    } else {
        return SearchResult::Unsatisfiable;
    };

    // for each d ∈ dom(v ) in some order:
    for d in model.dom(v) {
        // // α′ := α ∪ {v 7→ d}
        let mut alpha_prime = alpha.clone();
        alpha_prime.0.insert(v.clone(), Some(d));
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

fn backtracking_with_forward_checking(model: &Model, alpha: PartialAssignment) -> SearchResult {
    // if α is inconsistent with C:
    // // return inconsistent
    if model
        .constraints
        .iter()
        .any(|constraint| !constraint.check(&alpha))
    {
        return SearchResult::Unsatisfiable;
    }

    // if α is a total assignment:
    // // return α
    if alpha
        .0
        .iter()
        .all(|assignment_entry| assignment_entry.1.is_some())
    {
        return SearchResult::Assignment(alpha);
    }

    // C′ := ⟨V, dom′, (R_uv)⟩ := copy of C
    let mut model_prime = model.clone();

    // apply inference to C′
    model_prime.forward_checking();

    // if dom′(v) ̸= ∅ for all variables v:
    if model_prime.domains_available() {
        // // select some variable v for which α is not defined
        let v = alpha
            .0
            .iter()
            .find(|assignment_entry| assignment_entry.1.is_none());
        let v = if let Some(assignment) = v {
            assignment.0
        } else {
            return SearchResult::Unsatisfiable;
        };

        // // for each d ∈ dom(v ) in some order:
        for d in model_prime.dom(v) {
            // // // α′ := α ∪ {v 7→ d}
            let mut alpha_prime = alpha.clone();
            alpha_prime.0.insert(v.clone(), Some(d));

            // // // α′′ := BacktrackingWithForwardChecking(C, α′ )
            let alpha_prime_prime = backtracking_with_forward_checking(&model_prime, alpha_prime);

            // // // if α′′ ̸= inconsistent:
            if alpha_prime_prime != SearchResult::Unsatisfiable {
                // // // // return α′′
                return alpha_prime_prime;
            }
        }
    }

    // return inconsistent
    SearchResult::Unsatisfiable
}

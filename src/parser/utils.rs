use crate::model::var_id::VarId;
use flatzinc::*;

pub fn is_output(item: &VarDeclItem) -> bool {
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

pub fn extract_par_id(item: ParDeclItem) -> (String, ParDeclItem) {
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

pub fn extract_var_id(item: &VarDeclItem) -> VarId {
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

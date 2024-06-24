use super::SearchResult;
use crate::model::{partial_assignment::PartialAssignment, Model};

pub fn naive_backtracking(
    model: &Model,
    alpha: PartialAssignment,
    random_variable_order: bool,
) -> SearchResult {
    // if α is inconsistent with C:
    // // return inconsistent
    if model.is_inconsistent(&alpha) {
        return SearchResult::Unsatisfiable;
    }

    // if α is a total assignment:
    // // return α
    if alpha.is_total_assignment() {
        return SearchResult::Assignment(alpha);
    }

    // select some variable v for which α is not defined
    // let v = alpha.find_any_unassigned();infere
    let v = if random_variable_order {
        alpha.find_any_unassigned()
    } else {
        alpha.find_resticting_unassigned(model)
    };

    // for each d ∈ dom(v ) in some order:
    for d in model.dom(v) {
        // // α′ := α ∪ {v 7→ d}
        let alpha_prime = alpha.union(v, *d);
        // // α′′ := NaiveBacktracking(C, α′ )
        let alpha_prime_prime = naive_backtracking(model, alpha_prime, random_variable_order);

        // // if α′′ ̸= inconsistent:
        if alpha_prime_prime != SearchResult::Unsatisfiable {
            // // // return α′′
            return alpha_prime_prime;
        }
    }

    // return inconsistent
    SearchResult::Unsatisfiable
}

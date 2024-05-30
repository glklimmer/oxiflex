use super::SearchResult;
use crate::model::{partial_assignment::PartialAssignment, Model};

pub fn backtracking_with_forward_checking(model: &Model, alpha: PartialAssignment) -> SearchResult {
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

    // C′ := ⟨V, dom′, (R_uv)⟩ := copy of C
    let mut model_prime = model.clone();

    // apply inference to C′
    model_prime.forward_checking(&alpha);

    // if dom′(v) ̸= ∅ for all variables v:
    if model_prime.domains_available() {
        // // select some variable v for which α is not defined
        let v = alpha.find_any_unassigned();

        // // for each d ∈ dom(v ) in some order:
        for d in model_prime.dom(v) {
            // // // α′ := α ∪ {v 7→ d}
            let alpha_prime = alpha.union(v, *d);

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

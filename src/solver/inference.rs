use core::panic;

use super::SearchResult;
use crate::model::{partial_assignment::PartialAssignment, Model};

pub fn backtracking_with_inference(
    model: &Model,
    alpha: PartialAssignment,
    random_variable_order: bool,
    use_forward_checking: bool,
    arc_consistency: u8,
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

    // C′ := ⟨V, dom′, (R_uv)⟩ := copy of C
    let mut model_prime = model.clone();

    // apply inference to C′
    if use_forward_checking {
        model_prime.forward_checking(&alpha);
    } else {
        match arc_consistency {
            1 => model_prime.arc_consistency_1(),
            3 => model_prime.arc_consistency_3(),
            _ => panic!("No such arc consistency implemented."),
        };
    };

    // if dom′(v) ̸= ∅ for all variables v:
    if model_prime.domains_available() {
        // // select some variable v for which α is not defined
        let v = if random_variable_order {
            alpha.find_any_unassigned()
        } else {
            alpha.find_resticting_unassigned(model)
        };

        // // for each d ∈ dom(v ) in some order:
        for d in model_prime.dom(v) {
            // // // α′ := α ∪ {v 7→ d}
            let alpha_prime = alpha.union(v, *d);

            // // // α′′ := BacktrackingWithForwardChecking(C, α′ )
            let alpha_prime_prime = backtracking_with_inference(
                &model_prime,
                alpha_prime,
                random_variable_order,
                use_forward_checking,
                arc_consistency,
            );

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

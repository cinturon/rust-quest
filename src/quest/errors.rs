const ERROR_HINTS: &[(&str, &str)] = &[
    ("E0425", "Typo in variable name; not found in scope"),
    ("E0382", "Use after move"),
    ("E0599", "Wrong method on type"),
    ("E0308", "Type mismatch"),
    ("E0412", "Undefined type/name"),
    ("E0502", "Cannot borrow as mutable"),
];

fn explain_error_message(stderr: &str) -> Vec<String> {
    let mut explanations = Vec::new();

    for (code, hint) in ERROR_HINTS {
        let marker = format!("error[{code}]");
        if stderr.contains(&marker) {
            explanations.push(format!("Hint ({code}): {hint}"));
        }
    }

    explanations
}

pub fn print_error_hints(stderr: &str)  {
    let explanations = explain_error_message(stderr);

    if explanations.is_empty() {
        eprintln!("---------------No error hints found---------------");
        eprintln!("---------------Compiler Output--------------------");
        eprintln!("{}", stderr);
    } else {
        eprintln!("---------------Error hints---------------------");
        for explanation in explanations {
            eprintln!("{}", explanation);
        }
        eprintln!("---------------Compiler Output-----------------");
        eprintln!("{}", stderr);
    }
}

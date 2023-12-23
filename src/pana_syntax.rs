use egui_code_editor::Syntax;
use std::collections::HashSet;

pub fn pana_syntax() -> Syntax {
    Syntax {
        language: "Pana",
        case_sensitive: true,
        comment: "#",
        comment_multiline: ["###", "###"],
        keywords: HashSet::from([
            "mientras",
            "para",
            "romper",
            "retornar",
            "continuar",
            "si",
            "sino",
            "nulo",
            "fn",
            "var",
        ]),
        types: HashSet::from([]),
        special: HashSet::from(["Bucle"]),
    }
}

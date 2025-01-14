use super::*;
use eterned::memo;

#[derive(Debug, PartialEq, Eq)]
pub struct LxCommandPathMenu {
    // - general
    pub begin: LxCommandPath,
    pub end: LxCommandPath,
    // - root
    pub usepackage: LxCommandPath,
    pub documentclass: LxCommandPath,
    pub newtheorem: LxCommandPath,
    // - divisions
    pub part: LxCommandPath,
    pub chapter: LxCommandPath,
    pub section: LxCommandPath,
    pub subsection: LxCommandPath,
    pub subsubsection: LxCommandPath,
    // - maths
    // -- letter style
    pub mathbb: LxCommandPath,
    pub mathbf: LxCommandPath,
    pub mathcal: LxCommandPath,
    pub mathit: LxCommandPath,
    pub mathrm: LxCommandPath,
    pub mathsf: LxCommandPath,
    pub mathscr: LxCommandPath,
    // -- operators
    // --- relations
    pub eq: LxCommandPath,
    pub ne: LxCommandPath,
    pub neq: LxCommandPath,
    pub le: LxCommandPath,
    pub leq: LxCommandPath,
    pub ge: LxCommandPath,
    pub geq: LxCommandPath,
    pub r#in: LxCommandPath,
    pub subset: LxCommandPath,
    pub supset: LxCommandPath,
    pub subseteq: LxCommandPath,
    pub supseteq: LxCommandPath,
    pub subseteqq: LxCommandPath,
    pub supseteqq: LxCommandPath,
    pub subsetneq: LxCommandPath,
    pub supsetneq: LxCommandPath,
    // -- arithmetics
    pub cdot: LxCommandPath,
    pub int: LxCommandPath,
    pub sum: LxCommandPath,
    pub times: LxCommandPath,
    pub otimes: LxCommandPath,
    pub prod: LxCommandPath,
    // -- extended letters
    pub alpha: LxCommandPath,
    pub beta: LxCommandPath,
    pub gamma: LxCommandPath,
    pub pi: LxCommandPath,
    // --- functions
    pub sin: LxCommandPath,
    pub cos: LxCommandPath,
    // -- layouts
    pub sqrt: LxCommandPath,
    pub frac: LxCommandPath,
    // -- environments
    pub text: LxCommandPath,
    // - others
    pub left: LxCommandPath,
    pub right: LxCommandPath,
}

impl LxCommandPathMenu {
    fn new(db: &EternerDb) -> Self {
        let p = |data: &str| LxCommandPath::new_prelude(BaseCoword::from_ref(data, db), db);
        Self {
            // - general
            begin: p("begin"),
            end: p("end"),
            // - root
            usepackage: p("usepackage"),
            documentclass: p("documentclass"),
            newtheorem: p("newtheorem"),
            // - divisions
            part: p("part"),
            chapter: p("chapter"),
            section: p("section"),
            subsection: p("subsection"),
            subsubsection: p("subsubsection"),
            // - maths
            // ## letter style
            mathbb: p("mathbb"),
            mathbf: p("mathbf"),
            mathcal: p("mathcal"),
            mathit: p("mathit"),
            mathrm: p("mathrm"),
            mathsf: p("mathsf"),
            mathscr: p("mathscr"),
            // - operators
            // -- relations
            eq: p("eq"),
            ne: p("ne"),
            neq: p("neq"),
            le: p("le"),
            leq: p("leq"),
            ge: p("ge"),
            geq: p("geq"),
            r#in: p("in"),
            subset: p("subset"),
            supset: p("supset"),
            subseteq: p("subseteq"),
            supseteq: p("supseteq"),
            subseteqq: p("subseteqq"),
            supseteqq: p("supseteqq"),
            subsetneq: p("subsetneq"),
            supsetneq: p("supsetneq"),
            // -- arithmetic
            cdot: p("cdot"),
            int: p("int"),
            sum: p("sum"),
            times: p("times"),
            otimes: p("otimes"),
            prod: p("prod"),
            // -- extended letters
            alpha: p("alpha"),
            beta: p("beta"),
            gamma: p("gamma"),
            pi: p("pi"),
            // -- functions
            sin: p("sin"),
            cos: p("cos"),
            // -- layouts
            sqrt: p("sqrt"),
            frac: p("frac"),
            text: p("text"),
            left: p("left"),
            right: p("right"),
        }
    }
}

#[memo(return_ref)]
pub fn lx_command_path_menu(db: &EternerDb) -> LxCommandPathMenu {
    LxCommandPathMenu::new(db)
}

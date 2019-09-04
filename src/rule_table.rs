use ll_lexer::rule::{Rule, RuleTable};

use crate::symbol::GramSym;

pub fn get_rt() -> RuleTable<GramSym> {
    return RuleTable::new(
        vec![(GramSym::NtsExpr, false)],
        GramSym::TsEos,
        vec![
            Rule::new(
                GramSym::NtsExpr,
                GramSym::TsNbr,
                vec![(GramSym::NtsSign, true)],
            ),
            Rule::new(
                GramSym::NtsExpr,
                GramSym::TsLBracket1,
                vec![
                    (GramSym::NtsExpr, false),
                    (GramSym::TsRBracket1, false),
                    (GramSym::NtsSign, true),
                ],
            ),
            Rule::new(
                GramSym::NtsExpr,
                GramSym::TsLBracket2,
                vec![
                    (GramSym::NtsExpr, false),
                    (GramSym::TsRBracket2, false),
                    (GramSym::NtsSign, true),
                ],
            ),
            Rule::new(
                GramSym::NtsExpr,
                GramSym::TsLBracket3,
                vec![
                    (GramSym::NtsExpr, false),
                    (GramSym::TsRBracket3, false),
                    (GramSym::NtsSign, true),
                ],
            ),
            Rule::new(
                GramSym::NtsSign,
                GramSym::TsPlus,
                vec![(GramSym::NtsExpr, false)],
            ),
            Rule::new(
                GramSym::NtsSign,
                GramSym::TsLess,
                vec![(GramSym::NtsExpr, false)],
            ),
            Rule::new(
                GramSym::NtsSign,
                GramSym::TsTimes,
                vec![(GramSym::NtsExpr, false)],
            ),
            Rule::new(
                GramSym::NtsSign,
                GramSym::TsDivide,
                vec![(GramSym::NtsExpr, false)],
            ),
            Rule::new(
                GramSym::NtsSign,
                GramSym::TsModulo,
                vec![(GramSym::NtsExpr, false)],
            ),
            Rule::new(
                GramSym::NtsSign,
                GramSym::TsPower,
                vec![(GramSym::NtsExpr, false)],
            ),
        ],
    );
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LexSym {
    TsLBracket1,
    TsRBracket1,
    TsLBracket2,
    TsRBracket2,
    TsLBracket3,
    TsRBracket3,
    TsPlus,
    TsLess,
    TsTimes,
    TsDivide,
    TsModulo,
    TsPower,
    TsNbr(u32),
    TsEos,
    TsInvalid,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum GramSym {
    TsLBracket1,
    TsRBracket1,
    TsLBracket2,
    TsRBracket2,
    TsLBracket3,
    TsRBracket3,
    TsPlus,
    TsLess,
    TsTimes,
    TsDivide,
    TsModulo,
    TsPower,
    TsNbr,
    TsEos,
    TsInvalid,
    NtsExpr,
    NtsSign,
}

pub fn get_opposite_bracket(br: LexSym) -> Result<LexSym, String> {
    return match br {
        LexSym::TsLBracket1 => Ok(LexSym::TsRBracket1),
        LexSym::TsRBracket1 => Ok(LexSym::TsLBracket1),
        LexSym::TsLBracket2 => Ok(LexSym::TsRBracket2),
        LexSym::TsRBracket2 => Ok(LexSym::TsLBracket2),
        LexSym::TsLBracket3 => Ok(LexSym::TsRBracket3),
        LexSym::TsRBracket3 => Ok(LexSym::TsLBracket3),
        _ => Err(String::from("Symbol is not a bracket")),
    };
}

fn get_nb_spaces(s: &str) -> usize {
    let mut nb_spaces = 0;

    for c in s.chars() {
        if !c.is_whitespace() {
            break;
        }

        nb_spaces += 1;
    }

    return nb_spaces;
}

fn get_sub_or_add_symbol(s: &str) -> Result<(LexSym, GramSym, usize), String> {
    let mut addition = true;
    let mut size = 0;

    for c in s.chars() {
        match c {
            '+' => {}
            '-' => addition = !addition,
            _ => break,
        };

        size += 1;
    }

    if addition {
        return Ok((LexSym::TsPlus, GramSym::TsPlus, size));
    } else {
        return Ok((LexSym::TsLess, GramSym::TsLess, size));
    }
}

fn get_symbol_nbr(s: &str) -> Result<(LexSym, GramSym, usize), String> {
    let mut nb: u32 = 0;
    let mut size = 0;

    for c in s.chars() {
        if !c.is_digit(10) {
            break;
        }

        if let (nb2, false) = nb.overflowing_mul(10) {
            nb = nb2;
        } else {
            return Err("Error: Too large number".to_string());
        }

        if let (nb2, false) = nb.overflowing_add(c.to_digit(10).unwrap()) {
            nb = nb2;
        } else {
            return Err("Error: Too large number".to_string());
        }

        size += 1;
    }

    return Ok((LexSym::TsNbr(nb), GramSym::TsNbr, size));
}

pub fn get_symbol(s: &str) -> Result<(LexSym, GramSym, usize), String> {
    if s.len() == 0 {
        return Ok((LexSym::TsEos, GramSym::TsEos, 0));
    }

    let nb_spaces = get_nb_spaces(s);
    let c = s.chars().nth(nb_spaces).unwrap();

    let (lex, gram, size) = match c {
        '(' => (LexSym::TsLBracket1, GramSym::TsLBracket1, 1),
        ')' => (LexSym::TsRBracket1, GramSym::TsRBracket1, 1),
        '[' => (LexSym::TsLBracket2, GramSym::TsLBracket2, 1),
        ']' => (LexSym::TsRBracket2, GramSym::TsRBracket2, 1),
        '{' => (LexSym::TsLBracket3, GramSym::TsLBracket3, 1),
        '}' => (LexSym::TsRBracket3, GramSym::TsRBracket3, 1),
        '+' | '-' => get_sub_or_add_symbol(&s[nb_spaces..])?,
        '*' | '.' => (LexSym::TsTimes, GramSym::TsTimes, 1),
        '/' => (LexSym::TsDivide, GramSym::TsDivide, 1),
        '%' => (LexSym::TsModulo, GramSym::TsModulo, 1),
        '^' => (LexSym::TsPower, GramSym::TsPower, 1),
        '0'...'9' => get_symbol_nbr(&s[nb_spaces..])?,
        _ => (LexSym::TsInvalid, GramSym::TsInvalid, 1),
    };

    return Ok((lex, gram, size + nb_spaces));
}

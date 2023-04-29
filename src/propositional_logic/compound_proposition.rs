use super::proposition::Proposition;
use std::fmt::Display;

pub enum Operation {
    AND,
    OR,
    XOR,
    IMPLY,
    IFF,
}

pub enum Operands {
    Simple(Proposition, Proposition),
    Complex(Box<CompoundProposition>, Box<CompoundProposition>),
}

fn get_operator_symbol(operation: &Operation) -> char {
    match operation {
        Operation::AND => '^',
        Operation::OR => '\u{2228}',
        Operation::XOR => '\u{22bb}',
        Operation::IMPLY => '\u{2192}',
        Operation::IFF => '\u{27f7}',
    }
}

fn parse_operands(operands: &Operands) -> (String, String) {
    match operands {
        Operands::Simple(a, b) => (a.to_string(), b.to_string()),
        Operands::Complex(a, b) => (a.to_string(), b.to_string()),
    }
}

pub struct CompoundProposition {
    operands: Operands,
    operation: Operation,
}

impl CompoundProposition {
    pub fn new(operands: Operands, operation: Operation) -> CompoundProposition {
        CompoundProposition {
            operands,
            operation,
        }
    }
}

impl Display for CompoundProposition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let operands = parse_operands(&self.operands);
        write!(
            f,
            "({} {} {})",
            operands.0,
            get_operator_symbol(&self.operation),
            operands.1
        )
    }
}

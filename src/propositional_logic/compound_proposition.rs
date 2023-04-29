use super::proposition::Proposition;
use std::fmt::Display;
use regex::Regex;

#[derive(Clone, PartialEq)]
pub enum Operation {
    AND,
    OR,
    XOR,
    IMPLY,
    IFF,
}

#[derive(Clone, PartialEq)]
pub enum Operands {
    Simple(Proposition, Proposition),
    Complex(Box<CompoundProposition>, Box<CompoundProposition>),
}

fn get_operator_symbol(operation: &Operation) -> char {
    match operation {
        Operation::AND => '^',
        Operation::OR => '\u{2228}',
        Operation::XOR => '\u{2295}',
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

#[derive(Clone, PartialEq)]
pub struct CompoundProposition {
    operands: Operands,
    operation: Operation,
}

impl CompoundProposition {
    pub fn new(operands: &Operands, operation: Operation) -> CompoundProposition {
        CompoundProposition {
            operands: operands.clone(),
            operation,
        }
    }

    pub fn parse_sentence(sentence: &str) -> CompoundProposition {
        let sentence = sentence.to_lowercase();
        let sentence = sentence.as_str();
        let and_regex = Regex::new(r"(?P<op1>.+) and (?P<op2>.+)").unwrap();

        let and_captures = and_regex.captures(sentence);

        if and_captures.is_some() {
            let captures = and_captures.unwrap();
            let op1 = Proposition::new(&String::from(&captures["op1"]));
            let op2 = Proposition::new(&String::from(&captures["op2"]));

            let operands = Operands::Simple(op1, op2);
            return CompoundProposition::new(&operands, Operation::AND);

        } else {
            let op1 = Proposition::new(&String::from("lol"));
            let op2 = Proposition::new(&String::from("non"));

            let operands = Operands::Simple(op1, op2);
            return CompoundProposition::new(&operands, Operation::AND);
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

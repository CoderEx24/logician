use super::proposition::Proposition;
use std::fmt::Display;
use regex::Regex;

#[derive(Clone, PartialEq, Debug)]
pub enum Operation {
    AND,
    OR,
    XOR,
    IMPLY,
    IFF,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Operands {
    Simple(Proposition, Proposition),
    Complex(Box<CompoundProposition>, Box<CompoundProposition>),
    Mixed(Box<CompoundProposition>, Proposition, bool),
}

#[derive(Clone, PartialEq, Debug)]
pub struct CompoundProposition {
    operands: Operands,
    operation: Operation,
    negated: bool,
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
        Operands::Mixed(a, b, flip) => if *flip { (b.to_string(), a.to_string()) } else { (a.to_string(), b.to_string()) },
    }
}

pub fn pack_operands(prop1: &CompoundProposition, prop2: &CompoundProposition) -> Operands {
    if prop1.is_redundant() && prop2.is_redundant() {
        Operands::Simple(prop1.degrade().unwrap(), prop2.degrade().unwrap())
    } else if prop1.is_redundant() {
        Operands::Mixed(Box::new(prop2.clone()), prop1.degrade().unwrap(), true)
    } else if prop2.is_redundant() {
        Operands::Mixed(Box::new(prop1.clone()), prop2.degrade().unwrap(), false)
    } else {
        Operands::Complex(Box::new(prop1.clone()), Box::new(prop2.clone()))
    }
}

pub fn unpack_operands(ops: &Operands) -> (CompoundProposition, CompoundProposition) {
    match ops {
        Operands::Simple(a, b) => (CompoundProposition::new_redendant(&a), CompoundProposition::new_redendant(&b)),
        Operands::Mixed(a, b, flip) =>
            if *flip {
                (CompoundProposition::new_redendant(&b), *a.clone())
            } else {
                (*a.clone(), CompoundProposition::new_redendant(&b))
            }

        Operands::Complex(a, b) => (*a.clone(), *b.clone())
    }
}

impl CompoundProposition {
    pub fn new(operands: &Operands, operation: Operation) -> CompoundProposition {
        CompoundProposition {
            operands: operands.clone(),
            operation,
            negated: false,
        }
    }

    pub fn new_redendant(p: &Proposition) -> CompoundProposition {
        CompoundProposition {
            operands: Operands::Simple(p.clone(), p.clone()), /*if p.negated() {
                Operands::Simple(p.clone().negate(), p.clone().negate())
            } else {
                Operands::Simple(p.clone(), p.clone())
            },*/
            operation: Operation::OR,
            negated: false,
        }
    }

    pub fn parse_sentence(sentence: &str) -> CompoundProposition {
        let process_captures = move |captures: regex::Captures, operation: Operation| {
            let op1 = CompoundProposition::parse_sentence(&captures["op1"]);
            let op2 = CompoundProposition::parse_sentence(&captures["op2"]);

            let (degrade1, degrade2) = (op1.degrade(), op2.degrade());

            let operands = if degrade1.is_some() && degrade2.is_some() {
                Operands::Simple(degrade1.unwrap(), degrade2.unwrap())
            } else if degrade1.is_some() {
                Operands::Mixed(Box::new(op2), degrade1.unwrap(), true)
            } else if degrade2.is_some() {
                Operands::Mixed(Box::new(op1), degrade2.unwrap(), false)
            } else {
                Operands::Complex(Box::new(op1), Box::new(op2))
            };

            CompoundProposition::new(&operands, operation)
        };


        let sentence = sentence.to_lowercase();
        let sentence = sentence.as_str();
        let and_regex = Regex::new(r"(?P<op1>.+) and (?P<op2>.+)").unwrap();
        let or_regex  = Regex::new(r"(?P<op1>.+) or (?P<op2>.+)").unwrap();
        let imply_regex = Regex::new(r"if (?P<op1>.+), then (?P<op2>.+)").unwrap();
        let iff_regex = Regex::new(r"(?P<op1>.+) iff (?P<op2>.+)").unwrap();

        let and_captures = and_regex.captures(sentence);
        let or_captures = or_regex.captures(sentence);
        let imply_captures = imply_regex.captures(sentence);
        let iff_captures = iff_regex.captures(sentence);

        if iff_captures.is_some() {
            process_captures(iff_captures.unwrap(), Operation::IFF)
        } else if imply_captures.is_some() {
            process_captures(imply_captures.unwrap(), Operation::IMPLY)
        } else if or_captures.is_some() {
            process_captures(or_captures.unwrap(), Operation::OR)
        } else if and_captures.is_some() {
            process_captures(and_captures.unwrap(), Operation::AND)
        } else {
            let p = Proposition::new(&String::from(sentence));
            let operands = Operands::Simple(p.clone(), p.clone());

            CompoundProposition::new(&operands, Operation::OR)
        }
    }

    pub fn is_redundant(&self) -> bool {
        (self.operation == Operation::OR || self.operation == Operation::AND) 
            && match &self.operands {
            Operands::Simple(a, b) => a == b,
            _ => false
        }
    }

    pub fn degrade(&self) -> Option<Proposition> {
        if self.is_redundant() {
            if let Operands::Simple(o, _) = &self.operands {
                let o = if self.negated { o.clone().negate() } else { o.clone() };
                Some(o)
            } else {
                None
            }

        } else {
            None
        }
    }

    pub fn negate(&mut self) -> CompoundProposition {
//        self.negated = ! self.negated;

        self.operands = match &self.operands {
            Operands::Simple(a, b) => Operands::Simple(a.clone().negate(), b.clone().negate()),
            Operands::Complex(a, b) => Operands::Complex(Box::new(a.clone().negate()), Box::new(b.clone().negate())),
            Operands::Mixed(a, b, c) => Operands::Mixed(Box::new(a.clone().negate()), b.clone().negate(), *c),
        };

        self.clone()
    }

    pub fn operands(&self) -> Operands {
        self.operands.clone()
    }

    pub fn operation(&self) -> Operation {
        self.operation.clone()
    }
}

impl Display for CompoundProposition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let operands = parse_operands(&self.operands);
        write!(
            f,
            "{}({} {} {})",
            if self.negated { '\u{00ac}' } else { '\0' },
            operands.0,
            get_operator_symbol(&self.operation),
            operands.1
        )
    }
}


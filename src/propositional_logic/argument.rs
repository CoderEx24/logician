use std::collections::HashMap;
use std::fmt::Display;
use super::compound_proposition::{CompoundProposition, Operands};
use super::proposition::Proposition;

pub struct Argument {
    premises: Vec<CompoundProposition>,
    conclusion: CompoundProposition,
    propositional_variables: HashMap<String, String>,
}

pub fn get_propositional_variables(proposition: &CompoundProposition) -> HashMap<String, String> {
    let mut map = HashMap::new();
    
    match proposition.operands() {
        Operands::Atomic(p) => { map.insert(p.letter().to_string(), p.text().to_owned()); },
        Operands::Simple(a, b) => {
            map.insert(a.letter().to_string(), a.text().to_owned());
            map.insert(b.letter().to_string(), b.text().to_owned());
        },
        Operands::Mixed(a, b, _) => {
            map.insert(b.letter().to_string(), b.text().to_owned());
            map.extend(get_propositional_variables(&a));
        },
        Operands::Complex(a, b) => {
            map.extend(get_propositional_variables(&a));
            map.extend(get_propositional_variables(&b));
        }

    }

    map
}

impl Argument {
    pub fn new(premises: Vec<CompoundProposition>, conclusion: CompoundProposition) -> Argument {
        Argument {
            premises: premises.clone(),
            conclusion,
            propositional_variables: premises.iter()
                .map(|p| get_propositional_variables(&p))
                .reduce(|a, v| a.into_iter().chain(v).collect()).unwrap()
        }
    }

    pub fn parse_argument(argument: &str) -> Argument {
        let mut premises: Vec<CompoundProposition> = argument.lines().filter(|l| !l.trim().is_empty())
            .map(|l| CompoundProposition::parse_sentence(l)).collect();
        let conclusion = premises.pop().unwrap();

        Argument::new(premises, conclusion)
        
    }

    pub fn propositional_variables(&self) -> &HashMap<String, String> {
        &self.propositional_variables
    }
}


impl Display for Argument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();

        for (k, v) in &self.propositional_variables {
            string += format!("{}: {}\n", k, v).as_str();
        }

        for premise in &self.premises {
            string += format!("{}\n", premise).as_str()
        }

        string += format!("Conclusion: {}\n", self.conclusion).as_str();

        write!(f, "{}", string)
    }
}


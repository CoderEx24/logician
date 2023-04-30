use std::collections::HashMap;
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

    pub fn propositional_variables(&self) -> &HashMap<String, String> {
        &self.propositional_variables
    }
}


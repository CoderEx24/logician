use std::collections::HashMap;
use std::fmt::Display;
use super::compound_proposition::{CompoundProposition, Operands, Operation};
use super::proposition::Proposition;

pub struct Argument {
    premises: Vec<CompoundProposition>,
    conclusion: CompoundProposition,
    propositional_variables: HashMap<String, String>,
}

pub fn get_propositional_variables(proposition: &CompoundProposition) -> HashMap<String, String> {
    let mut map = HashMap::new();
    
    match proposition.operands() {
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
                .reduce(|a, v| a.into_iter().chain(v).collect()).unwrap(),
        }
    }

    pub fn check_argument(&self) -> bool {
        let rules_of_inference = [
            // modes ponens
            |proposition1: &CompoundProposition, proposition2: &CompoundProposition| -> Option<CompoundProposition> {
                let (op1, op2) = match proposition1.operands() {
                    Operands::Simple(a, b) => ((CompoundProposition::new_redendant(&a), CompoundProposition::new_redendant(&b))),
                    Operands::Complex(a, b) => ((*a.clone(), *b.clone())),
                    Operands::Mixed(a, b, flip) => if flip {
                        ((CompoundProposition::new_redendant(&b), *a.clone())) 
                    } else {
                        ((*a.clone(), CompoundProposition::new_redendant(&b))) 
                    }
                };
                
                if proposition1.operation() == Operation::IMPLY && op1 == *proposition2 { 
                    Some(op2)
                } else {
                    None
                }
            },

            // modes tollens
            |proposition1: &CompoundProposition, proposition2: &CompoundProposition| -> Option<CompoundProposition> {
                let (op1, op2) = match proposition1.operands() {
                    Operands::Simple(a, b) => ((CompoundProposition::new_redendant(&a), CompoundProposition::new_redendant(&b))),
                    Operands::Complex(a, b) => ((*a.clone(), *b.clone())),
                    Operands::Mixed(a, b, flip) => if flip {
                        ((CompoundProposition::new_redendant(&b), *a.clone())) 
                    } else {
                        ((*a.clone(), CompoundProposition::new_redendant(&b))) 
                    }
                };
                
                if proposition1.operation() == Operation::IMPLY && op2.clone().negate() == *proposition2 { 
                    Some(op1.clone().negate())
                } else {
                    None
                }
            },
        ];

        let mut premises = self.premises.clone();
        let mut premises_len = premises.len();

        let mut i = 0;

        while i < premises_len {
            let op1 = &premises[i].clone();

            let mut j = 0;
            while j < premises_len {
                if i == j {
                    j += 1;
                    continue;
                }


                let op2 = &premises[j].clone();

                let mut results: Vec<CompoundProposition> = rules_of_inference.iter()
                    .map(|rule| rule(&op1, &op2).or(rule(&op2, &op1)))
                    .filter(|value| value.is_some())
                    .map(|value| value.unwrap())
                    .filter(|value| !premises.contains(value))
                    .collect();

                if results.contains(&self.conclusion) {
                    println!("CONCLUSION REACHED!!!!!!");
                }
               
                premises_len += results.len();
                premises.append(&mut results);
                
                j += 1;
            }
            i += 1;
        }


        false

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


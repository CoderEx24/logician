use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct Proposition {
    text: String,
    letter: char,
    negated: bool,
}

impl Proposition {
    pub fn new(s: &String) -> Proposition {
        Proposition {
            text: s.clone().replace("not ", ""),
            // TODO: better way to assign values to letter and negated
            letter: s.as_bytes()[0].into(),
            negated: s.contains("not"),
        }
    }

    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn letter(&self) -> char {
        self.letter
    }

    pub fn negated(&self) -> bool {
        self.negated
    }

    pub fn negate(&mut self) -> Proposition {
        self.negated = !self.negated;
        self.clone()
    }
}

impl Display for Proposition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", if self.negated { '\u{00ac}' } else { '\0' } , self.letter)
    }
}

impl PartialEq for Proposition {
    fn eq(&self, other: &Self) -> bool {
        self.letter == other.letter && self.negated == other.negated
    }
    
}

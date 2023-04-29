use std::fmt::Display;

pub struct Proposition {
    text: String,
    letter: char,
    negated: bool,
}

impl Proposition {
    pub fn new(s: &String) -> Proposition {
        Proposition {
            text: s.clone(),
            // TODO: better way to assign values to letter and negated
            letter: s.as_bytes()[0].into(),
            negated: s.contains("not"),
        }
    }

    pub fn text(&self) -> String {
        self.text
    }

    pub fn letter(&self) -> char {
        self.letter
    }

    pub fn negated(&self) -> bool {
        self.negated
    }

    pub fn negate(&mut self) {
        self.negated = !self.negated;
    }
}

impl Display for Proposition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.letter, self.text)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum PatternPosition {
    Begin,
    End,
    Anywhere,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Pattern<'a> {
    pattern: &'a str,
    position: PatternPosition,
}

impl<'a> From<&'a str> for Pattern<'a> {
    fn from(p: &'a str) -> Self {
        let mut position = PatternPosition::Anywhere;
        let mut pattern = p;

        if p.ends_with('$') {
            pattern = &pattern[..pattern.len() - 1];
            position = PatternPosition::End;
        }
        if p.starts_with('^') {
            pattern = &pattern[1..];
            position = PatternPosition::Begin;
        }

        Self { pattern, position }
    }
}

impl<'a> Pattern<'a> {
    pub fn is_match(&'a self, string: &'_ str) -> bool {
        match self.position {
            PatternPosition::End => string.ends_with(self.pattern),
            PatternPosition::Begin => string.starts_with(self.pattern),
            PatternPosition::Anywhere => string.contains(self.pattern),
        }
    }
}

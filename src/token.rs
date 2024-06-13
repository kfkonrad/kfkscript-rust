#[derive(PartialEq)]
pub enum TokenType {
    Keyword,
    KfkApostropheString,
    KfkDollarString,
    Number,
    None,
}

#[derive(Debug, Clone)]
pub struct Keyword {
    pub lexem: String,
    pub line_number: u32,
}

// for KfkString and Number equivalence and hashing only pertain to their values, that is 'foo" and $foo are equivalent
// and will hash identically, as will 4 and 4.0

#[derive(Debug, Clone)]
pub struct KfkString {
    pub lexem: String,
    pub line_number: u32,
}

impl PartialEq for KfkString {
    fn eq(&self, other: &Self) -> bool {
        self.lexem == other.lexem
    }
}

impl std::hash::Hash for KfkString {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.lexem.hash(state);
    }
}

impl Eq for KfkString {}

#[derive(Debug, Clone)]
pub struct Number {
    pub lexem: String,
    pub number: f64,
    pub line_number: u32,
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.number.to_string() == other.number.to_string()
    }
}

impl Eq for Number {}

impl std::hash::Hash for Number {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.number.to_string().hash(state);
    }
}

#[derive(Debug, Clone)]
pub enum Token {
    Keyword(Keyword),
    KfkString(KfkString),
    Number(Number),
}

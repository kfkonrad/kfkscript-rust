#[allow(clippy::module_name_repetitions)]
#[derive(PartialEq, Eq)]
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

#[derive(Debug, Clone)]
pub struct KfkString {
    pub lexem: String,
    pub line_number: u32,
}

#[allow(clippy::struct_field_names)]
#[derive(Debug, Clone)]
pub struct Number {
    pub lexem: String,
    pub number: f64,
    pub line_number: u32,
}

#[derive(Debug, Clone)]
pub enum Token {
    Keyword(Keyword),
    KfkString(KfkString),
    Number(Number),
}

#[derive(PartialEq)]
pub enum TokenType {
    Keyword,
    KfkApostropheString,
    KfkDollarString,
    Number,
    None
}

#[derive(Debug)]
pub struct Keyword{
  pub lexem: String,
  pub line_number: u32
}

#[derive(Debug)]
pub struct KfkString{
  pub lexem: String,
  pub line_number: u32
}

#[derive(Debug)]
pub struct Number{
  pub lexem: String,
  pub number: f64,
  pub line_number: u32
}

#[derive(Debug)]
pub enum Token {
    Keyword(Keyword),
    KfkString(KfkString),
    Number(Number),
}

use crate::token;
use crate::token::{Token, TokenType};

use color_eyre::eyre::{eyre, Result};

fn determine_next_token_type(current_char: char, next_char: Option<&char>) -> (TokenType, String) {
    match current_char {
        '-' => next_char.map_or_else(
            || (TokenType::Keyword, '-'.into()),
            |next_char| {
                if next_char.is_ascii_digit() {
                    (TokenType::Number, current_char.into())
                } else {
                    (TokenType::Keyword, current_char.into())
                }
            },
        ),
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
            (TokenType::Number, current_char.into())
        }
        '$' => (TokenType::KfkDollarString, String::new()),
        '\'' => (TokenType::KfkApostropheString, String::new()),
        x if x.is_whitespace() => (TokenType::None, current_char.into()),
        _ => (TokenType::Keyword, current_char.into()),
    }
}

pub fn parse(source_code: &str) -> Result<Vec<Token>> {
    let mut tokens: Vec<Token> = vec![];
    let mut line_number = 1;
    let mut prelim = String::new();
    let mut token_type = TokenType::Keyword;
    let mut source_code_iter = source_code.chars().peekable();
    let mut is_comment = false;

    while let Some(current_char) = source_code_iter.next() {
        let next_char = source_code_iter.peek();
        let newline = current_char == '\n';

        if token_type == TokenType::None {
            if !is_comment {
                is_comment = current_char == '#';
            }
            if is_comment {
                if newline {
                    is_comment = false;
                    line_number += 1;
                    continue;
                }
                continue;
            }

            (token_type, prelim) = determine_next_token_type(current_char, next_char);

            if newline {
                line_number += 1;
            }
            continue;
        }

        match token_type {
            TokenType::KfkApostropheString => {
                if current_char == '"' {
                    tokens.push(Token::KfkString(token::KfkString {
                        lexem: prelim,
                        line_number,
                    }));
                    prelim = String::new();
                    token_type = TokenType::None;
                } else {
                    prelim = format!("{prelim}{current_char}");
                }
            }
            TokenType::KfkDollarString => {
                if current_char == ' ' || current_char == '\n' {
                    tokens.push(Token::KfkString(token::KfkString {
                        lexem: prelim,
                        line_number,
                    }));
                    prelim = String::new();
                    token_type = TokenType::None;
                } else {
                    prelim = format!("{prelim}{current_char}");
                }
            }
            TokenType::Keyword => {
                if current_char.is_whitespace() || current_char == '#' {
                    is_comment = current_char == '#';
                    tokens.push(Token::Keyword(token::Keyword {
                        lexem: prelim,
                        line_number,
                    }));
                    prelim = String::new();
                    token_type = TokenType::None;
                } else {
                    prelim = format!("{prelim}{current_char}");
                }
            }
            TokenType::Number => {
                if current_char.is_whitespace() || current_char == '#' {
                    is_comment = current_char == '#';
                    let number = prelim.parse().map_err(|_| {
                        eyre!(format!("invalid number in line {line_number}: {prelim}"))
                    })?;
                    tokens.push(Token::Number(token::Number {
                        lexem: prelim,
                        number,
                        line_number,
                    }));
                    prelim = String::new();
                    token_type = TokenType::None;
                } else {
                    prelim = format!("{prelim}{current_char}");
                }
            }
            TokenType::None => {}
        }
        if newline {
            line_number += 1;
        }
    }
    Ok(tokens)
}

pub fn print_tokens(tokens: Vec<Token>) {
    let mut current_line: u32 = 1;
    for token in tokens {
        match token {
            Token::Keyword(token) => {
                if token.line_number > current_line {
                    for _ in current_line..token.line_number {
                        println!();
                    }
                    current_line = token.line_number;
                }
                print!("{} ", token.lexem);
            }
            Token::KfkString(token) => {
                if token.line_number > current_line {
                    for _ in current_line..token.line_number {
                        println!();
                    }
                    current_line = token.line_number;
                }
                print!("'{}\" ", token.lexem);
            }
            Token::Number(token) => {
                if token.line_number > current_line {
                    for _ in current_line..token.line_number {
                        println!();
                    }
                    current_line = token.line_number;
                }
                print!("{} ", token.lexem);
            }
        }
    }
    println!();
}

use crate::utils::exit_with_error;

#[derive(Debug, Clone)]
pub enum Token {
    Number(f64),
    Operator(char),
    Parenthesis(char),
    Function(String),
    Separator,
}

pub const OPERATORS: [char; 4] = ['+', '-', '*', '/'];
pub const FUNCTIONS: [&str; 8] = ["sin", "cos", "tan", "asin", "acos", "atan", "sqrt", "pow"];

pub fn tokenize_expression(exp: &str) -> Vec<Token> {
    let exp_len = exp.len();
    let mut i = 0;
    let mut tokens: Vec<Token> = Vec::new();

    while i < exp_len {
        let current = exp.chars().nth(i).unwrap();

        if current.is_numeric() {
            let (number, new_i) = tokenize_number(exp, i);
            tokens.push(Token::Number(number.parse().unwrap()));
            i = new_i;
        } else if OPERATORS.contains(&current) {
            tokens.push(Token::Operator(current));
            i += 1;
        } else if current == '(' || current == ')' {
            tokens.push(Token::Parenthesis(current));
            i += 1;
        } else if current == ',' {
            tokens.push(Token::Separator);
            i += 1;
        } else if current.is_alphabetic() {
            let (function, new_i) = tokenize_function(exp, i);
            tokens.push(Token::Function(function));
            i = new_i;
        } else {
            exit_with_error(format!("Invalid character: {} at position {}", current, i+1));
        }
    }

    return tokens;
}

fn tokenize_number(exp: &str, i: usize) -> (String, usize) {
    let mut number = String::new();
    let mut i = i;

    while i < exp.len() {
        let c = exp.chars().nth(i).unwrap();

        if c.is_numeric() || c == '.' {
            number.push(c);
            i += 1;
        } else {
            break;
        }
    }

    (number, i)
}

fn tokenize_function(exp: &str, i: usize) -> (String, usize) {
    let mut function = String::new();
    let mut i = i;

    while i < exp.len() {
        let c = exp.chars().nth(i).unwrap();

        if c.is_alphabetic() {
            function.push(c);
            i += 1;
        } else {
            break;
        }
    }

    if !FUNCTIONS.contains(&function.as_str()) {
        exit_with_error(format!("Invalid function: {} at position {}", function, i+1));
    }

    (function, i)
}
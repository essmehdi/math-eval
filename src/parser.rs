use crate::{tokenizer::Token, utils::exit_with_error};

#[derive(Debug)]
pub struct OperationTree {
    pub val: Token,
    pub left: Option<Box<OperationTree>>,
    pub right: Option<Box<OperationTree>>,
}

pub fn parse_expression(tokens: &Vec<Token>, i: usize) -> (OperationTree, usize) {
    let (operand, new_i) = parse_operand(&tokens, i);
    let next_token = tokens.get(new_i);

    match next_token {
        Some(Token::Operator('+')) | Some(Token::Operator('-')) => {
            let (next_operand, new_i) = parse_expression(tokens, new_i + 1);
            let tree = OperationTree {
                val: next_token.unwrap().clone(),
                left: Some(Box::new(operand)),
                right: Some(Box::new(next_operand)),
            };
            (tree, new_i)
        },
        _ => {
            (operand, new_i)
        }
    }
}

pub fn parse_operand(tokens: &Vec<Token>, i: usize) -> (OperationTree, usize) {
    let (term, new_i) = parse_term(tokens, i);
    let next_token = tokens.get(new_i);

    match next_token {
        Some(Token::Operator('*')) | Some(Token::Operator('/')) => {
            let (next_term, new_i) = parse_operand(tokens, new_i + 1);
            let tree = OperationTree {
                val: next_token.unwrap().clone(),
                left: Some(Box::new(term)),
                right: Some(Box::new(next_term)),
            };
            (tree, new_i)
        },
        _ => {
            (term, new_i)
        }
    }
}

pub fn parse_term(tokens: &Vec<Token>, i: usize) -> (OperationTree, usize) {
    let token = tokens.get(i);
    match token {
        Some(Token::Number(_)) => {
            let tree = OperationTree {
                val: token.unwrap().clone(),
                left: None,
                right: None,
            };
            (tree, i + 1)
        },
        Some(Token::Function(func)) => {
            let next_token = tokens.get(i + 1);
            match next_token {
                Some(Token::Parenthesis('(')) => {
                    let (tree, new_i) = parse_expression(tokens, i + 2);
                    let next_token = tokens.get(new_i);
                    match next_token {
                        Some(Token::Separator) => {
                            if get_function_params_number(func) != 2 {
                                exit_with_error(format!("Expected closing parenthesis, found {}", next_token.unwrap()));
                                unreachable!()
                            }
                            let (next_tree, new_i) = parse_expression(tokens, new_i + 1);
                            let next_token = tokens.get(new_i);
                            match next_token {
                                Some(Token::Parenthesis(')')) => {
                                    let tree = OperationTree {
                                        val: token.unwrap().clone(),
                                        left: Some(Box::new(tree)),
                                        right: Some(Box::new(next_tree)),
                                    };
                                    (tree, new_i + 1)
                                },
                                _ => {
                                    exit_with_error(format!("Expected closing parenthesis, found {}", next_token.unwrap_or(&Token::EOF)));
                                    unreachable!()
                                }
                            }
                        },
                        Some(Token::Parenthesis(')')) => {
                            let tree = OperationTree {
                                val: token.unwrap().clone(),
                                left: Some(Box::new(tree)),
                                right: None,
                            };
                            (tree, new_i + 1)
                        },
                        _ => {
                            exit_with_error(format!("Expected separator or closing parenthesis, found {}", next_token.unwrap_or(&Token::EOF)));
                            unreachable!()
                        }
                    }
                },
                _ => {
                    exit_with_error(format!("Expected opening parenthesis, found {}", next_token.unwrap_or(&Token::EOF)));
                    unreachable!()
                }
            }
        },
        Some(Token::Parenthesis('(')) => {
            let (tree, new_i) = parse_expression(tokens, i + 1);
            let next_token = tokens.get(new_i);
            match next_token {
                Some(Token::Parenthesis(')')) => {
                    (tree, new_i + 1)
                },
                _ => {
                    exit_with_error(format!("Expected closing parenthesis, found {}", next_token.unwrap_or(&Token::EOF)));
                    unreachable!()
                }
            }
        },
        _ => {
            exit_with_error(format!("Expected number, function or opening parenthesis, found {}", token.unwrap_or(&Token::EOF)));
            unreachable!()
        }
    }
}

fn get_function_params_number(function: &str) -> usize {
    match function {
        "sin" | "cos" | "tan" | "asin" | "acos" | "atan" | "sqrt" => 1,
        "pow" => 2,
        _ => {
            exit_with_error(format!("Invalid function: {}", function));
            unreachable!()
        },
    }
}
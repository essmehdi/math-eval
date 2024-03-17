use crate::parser::OperationTree;
use crate::tokenizer::Token;
use crate::utils::exit_with_error;

pub fn evaluate_tree(tree: &OperationTree) -> f64 {
    match &tree.val {
        Token::Number(n) => *n,
        Token::Operator(op) => {
            let left = evaluate_tree(tree.left.as_ref().unwrap());
            let right = evaluate_tree(tree.right.as_ref().unwrap());
            match op {
                '+' => left + right,
                '-' => left - right,
                '*' => left * right,
                '/' => left / right,
                _ => {
                    exit_with_error(format!("Invalid operator: {}", op));
                    unreachable!()
                },
            }
        },
        Token::Function(function) => {
            let operand = evaluate_tree(tree.left.as_ref().unwrap());
            match function.as_str() {
                "sin" => operand.sin(),
                "cos" => operand.cos(),
                "tan" => operand.tan(),
                "asin" => operand.asin(),
                "acos" => operand.acos(),
                "atan" => operand.atan(),
                "sqrt" => operand.sqrt(),
                "pow" => {
                    let exponent = evaluate_tree(tree.right.as_ref().unwrap());
                    operand.powf(exponent)
                },
                _ => {
                    exit_with_error(format!("Invalid function: {}", function));
                    unreachable!()
                }
            }
        },
        _ => {
            exit_with_error(format!("Invalid token: {:?}", tree.val));
            unreachable!()
        }
    }
}
mod tokenizer;
mod utils;
mod parser;
mod evaluator;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        println!("You should provide a single expression after the command: {} <expression>", args[0]);
        return;
    }

    let expression = args[1].clone();
    let expression = expression.replace(" ", "");

    let tokens = tokenizer::tokenize_expression(&expression);
    let (tree, _) = parser::parse_expression(&tokens, 0);   

    let result = evaluator::evaluate_tree(&tree);
    println!("{}", result);
}

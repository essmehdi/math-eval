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

    println!("\nTokenizing...\n");

    let tokens = tokenizer::tokenize_expression(&expression);
    for token in tokens.iter() {
        println!("{:?}", token);
    }

    println!("\nParsing...\n");

    let (tree, _) = parser::parse_expression(&tokens, 0);   
    println!("{:?}", &tree);

    println!("\nEvaluating...\n");

    let result = evaluator::evaluate_tree(&tree);
    println!("Result: {}", result);

}

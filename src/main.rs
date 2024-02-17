use crate::tokens::{string_to_rpn, Token, tokenize};

mod tokens;
mod solver;
mod operators;
mod syntaxtree;

fn main() {
    let string = "2 ^^ 2 + 3 - 4";
    let vf = tokenize!(string);
    let rpn = string_to_rpn!(string);
    if let Ok(rpn) = rpn {
        let token = solver::solve_reverse_polish_notation(&rpn);
        println!("\n{}\n\n", Token::tokens_to_string(&rpn));
        let tree = syntaxtree::reverse_polish_string_to_tree(&rpn);
        for tr in tree {
            tr.print();
        }

        println!("\n");
        if let Some(ref token) = token {
            println!("Result: {:?}", token.value);
        }
    } else {
        println!("Error. Bad expression.")
    }
}

pub mod ast;
pub mod tokens;
pub mod cell_operations;
// pub mod evaluate_operations;
use lalrpop_util::lalrpop_mod;
use logos::Logos;
use crate::cell_operations::{Sheet};
use std::io;

lalrpop_mod!(pub grammar); // include the generated parser

/* STUFF TO DO:
O - initialise
O - ask input
O - If error, report respective error, restart
O - If not error:
    O - Check required cells:
        O - If out of range, report error (In extension: Suggest resizing)
        O - Initialise if not done yet
    O - Give new function to - cell
    O - Give old and new function to - evaluate()
    O - If result is error (loop) then report error.
restart
*/

struct SheetNames {
    map: Vec<(String, u32)>
}

impl SheetNames {
    numFromName
}




fn main() -> Result<(), Box<dyn std::error::Error>>{

    let r: u64 = std::env::args().nth(1)
        .expect("Row number not entered (First arg missing)")
        .parse().expect("Invalid input for Row number (First arg)");
    
    let c: u64 = std::env::args().nth(2)
        .expect("Column number not entered (Second arg missing)")
        .parse().expect("Invalid input for Column number (Second arg)");

//sheets: &Vec<Rc<RefCell<Sheet>>>

    // let sheets: Vec<Rc<RefCell<Sheet>>> = Vec



    let file = std::env::args().nth(1).unwrap_or("test.txt".into());
    let source_code = std::fs::read_to_string(file)?;
    // let source_code = "11+4+56";

    let lexer = tokens::Token::lexer(&source_code).spanned()
    .map(|(token_result, span)| {
        let token = token_result?; // Propagate LexicalError
        Ok((span.start, token, span.end)) // (usize, Token, usize)
    });
    
    let parser = grammar::CommandParser::new();
    let ast = parser.parse(5, lexer)?;

    println!("{:?}", ast);
    Ok(())
    // println!("{:?}", calc1::ExprParser::new().parse(Lexer::new("22")));
    // println!("{:?}", calc1::ExprParser::new().parse(Lexer::new("22+5")));
    // println!("{:?}", calc1::ExprParser::new().parse(Lexer::new("22384756239862-(4+4)")));
    // println!("{:?}", calc1::ExprParser::new().parse(Lexer::new("22*5-6*5")));
}

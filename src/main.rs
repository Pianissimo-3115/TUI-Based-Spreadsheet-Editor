pub mod ast;
pub mod tokens;
// pub mod cell_operations;
// pub mod evaluate_operations;
use lalrpop_util::lalrpop_mod;
use logos::Logos;

lalrpop_mod!(pub grammar); // include the generated parser


fn main() -> Result<(), Box<dyn std::error::Error>>{
    let file = std::env::args().nth(1).unwrap_or("test.txt".into());
    let source_code = std::fs::read_to_string(file)?;
    // let source_code = "11+4+56";

    let lexer = tokens::Token::lexer(&source_code).spanned()
    .map(|(token_result, span)| {
        let token = token_result?; // Propagate LexicalError
        Ok((span.start, token, span.end)) // (usize, Token, usize)
    });
    
    let parser = grammar::CommandParser::new();
    let ast = parser.parse(lexer)?;

    println!("{:?}", ast);
    Ok(())
    // println!("{:?}", calc1::ExprParser::new().parse(Lexer::new("22")));
    // println!("{:?}", calc1::ExprParser::new().parse(Lexer::new("22+5")));
    // println!("{:?}", calc1::ExprParser::new().parse(Lexer::new("22384756239862-(4+4)")));
    // println!("{:?}", calc1::ExprParser::new().parse(Lexer::new("22*5-6*5")));
}

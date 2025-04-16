pub mod ast;
pub mod tokens;
pub mod cell_operations;
pub mod evaluate_operations;
use lalrpop_util::lalrpop_mod;
use lalrpop_util::ParseError;
use logos::Logos;
use crate::cell_operations::{ CellFunc, Sheet, ValueType};
use crate::evaluate_operations::evaluate;
use crate::tokens::LexicalError;
use std::io::{self, Write};
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp;


//NOTE: PLEASE HAR JAGA usize KAR DO, bohot zyada conversions karne pad rahe hai


lalrpop_mod!(pub grammar); // include the generated parser

/* STUFF TO DO:
0 - initialise
0 - ask input
0 - If error, report respective error, restart
O - If not error:
    0 - Check required cells:
        0 - If out of range, report error (In extension: Suggest resizing)
        0 - Initialise if not done yet (should now happen automatically)
    0 - Give new function to - cell
    O - Give old and new function to - evaluate()
    O - If result is error (loop) then report error.
restart
*/

// struct SheetNames {
//     map: Vec<(String, u32)>
// }

// impl SheetNames {
//     fn numFromName(&self, name: str) -> u32 {
//         for i in 0..self
//     }
// }



fn display_sheet(col: u32, row: u32, sheet: &Sheet)
{
    let row_max = cmp::min(row+10, sheet.rows);
    let col_max = cmp::min(col+10, sheet.columns);

    print!("      ");
    for i in col..col_max {
        let mut curr = String::new();
        let mut curr_col = i + 1;
        while curr_col > 0
        {

            curr.push((('A' as u8) + ((curr_col-1) % 26) as u8) as char);
            
            curr_col -= 1;
            curr_col /= 26;
        }
        print!("{:>6}", curr);
    }
    print!("\n");
    for i in row..row_max {
        print!("{i:>6}");
        for j in col..col_max {
            let val =  sheet.val_at(j as usize, i as usize);
            match val {
                ValueType::IntegerValue(x) => print!("{:>6}", x),
                ValueType::FloatValue(n) => print!("{:>6}", n),
                ValueType::String(s) => print!("{:>6}", s),
            }
            
        }
        print!("\n")
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>>{

    let r: u32 = std::env::args().nth(1)
        .expect("Row number not entered (First arg missing)")
        .parse().expect("Invalid input for Row number (First arg)");
    
    let c: u32 = std::env::args().nth(2)
        .expect("Column number not entered (Second arg missing)")
        .parse().expect("Invalid input for Column number (Second arg)");

//sheets: &Vec<Rc<RefCell<Sheet>>>

    let mut sheets: Vec<Rc<RefCell<Sheet>>> = vec![Rc::new(RefCell::new(Sheet::new(0, String::from("sheet0"), c, r)))];

    let mut exit : bool = false;

    let mut curr_col: usize= 0;
    let mut curr_row: usize = 0;
    let mut show_window: bool = true;
    let mut last_err_msg = String::from("ok");
    
    'mainloop: while !exit {
        if show_window {
            // let curr_sheet = ;
            display_sheet(curr_col as u32, curr_row as u32, &sheets[0].borrow());
        }
        let mut inp = String::new();

        print!("({}) >> ", last_err_msg);
        io::stdout().flush().unwrap();

        io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read line"); //NOTE (to self): Better error message

        let lexer = tokens::Token::lexer(&inp).spanned()
        .map(|(token_result, span)| {
            let token = token_result?; // Propagate LexicalError
            Ok((span.start, token, span.end)) // (usize, Token, usize)
        });
        
        let parser = grammar::CommandParser::new();

        let (ast, dep_vec) = match parser.parse(0, lexer) {  //NOTE: Error messages are temporary.
            Ok(x) => x,
            Err(ParseError::User{error: LexicalError::InvalidToken}) => {last_err_msg = String::from("Invalid Token"); continue},
            Err(ParseError::User{error: LexicalError::InvalidInteger(x)}) => {last_err_msg = String::from(format!("Invalid Integer {:?}", x)); continue}, 
            Err(e) => {last_err_msg = String::from(format!("This error: {:?}", e)); continue}
        };
        println!("{:?}", dep_vec);
        println!("{:?}", ast);


        match ast {
            ast::Command::DisplayCmd(d_cmd) => {
                let curr_sheet = &sheets[0].borrow();
                match d_cmd {
                    ast::DisplayCommand::EnableOut => show_window = true,
                    ast::DisplayCommand::DisableOut => show_window = false,
                    ast::DisplayCommand::ScrollTo(addr) => 
                    {
                        if (addr.row >= curr_sheet.rows) | (addr.col >= curr_sheet.columns) {
                            last_err_msg = String::from("Address out of bounds");
                            continue
                        }
                        curr_col = cmp::max(0, cmp::min(addr.col as i64, curr_sheet.columns as i64 - 10)) as usize;
                        curr_row = cmp::max(0, cmp::min(addr.row as i64, curr_sheet.rows as i64 - 10)) as usize; 
                    },

                    ast::DisplayCommand::MoveUp => curr_row = cmp::max(0, cmp::min(curr_row as i64 +1 , curr_sheet.rows as i64 - 10)) as usize,
                    ast::DisplayCommand::MoveDown => curr_row = cmp::max(0, cmp::min(curr_row as i64 -1 , curr_sheet.rows as i64 - 10)) as usize,
                    ast::DisplayCommand::MoveRight => curr_col = cmp::max(0, cmp::min(curr_col as i64 +1 , curr_sheet.columns as i64 - 10)) as usize,
                    ast::DisplayCommand::MoveLeft => curr_col = cmp::max(0, cmp::min(curr_col as i64 -1 , curr_sheet.columns as i64 - 10)) as usize,
                }},
            ast::Command::Quit => exit = true,
            ast::Command::AssignCmd(a, b_ex) => {  //NOTE: All validity checks for addresses will be more complicated when we implement multiple sheets.
                let old_func: Option<CellFunc>;
                {
                let curr_sheet = &sheets[0].borrow();
                if a.row >= curr_sheet.rows {
                    last_err_msg = String::from("Target address row out of range"); //NOTE: Error messages are temporary.
                    continue 'mainloop;
                }
                if a.col >= curr_sheet.columns {
                    last_err_msg = String::from("Target address column out of range"); //NOTE: Error messages are temporary.
                    continue 'mainloop;
                }

                for dep in &dep_vec {
                    match dep {
                        ast::ParentType::Single(a_1) => {
                            if a_1.row >= curr_sheet.rows {
                                last_err_msg = String::from("Address row out of range"); //NOTE: Error messages are temporary.
                                continue 'mainloop;
                            }
                            if a_1.col >= curr_sheet.columns {
                                last_err_msg = String::from("Address column out of range"); //NOTE: Error messages are temporary.
                                continue 'mainloop;
                            }
                        },
                        ast::ParentType::Range(a_1, a_2) => {
                            if a_1.row >= curr_sheet.rows {
                                last_err_msg = String::from("Range start address row out of range"); //NOTE: Error messages are temporary.
                                continue 'mainloop;
                            }
                            if a_1.col >= curr_sheet.columns {
                                last_err_msg = String::from("Range start address column out of range"); //NOTE: Error messages are temporary.
                                continue 'mainloop;
                            }
                            if a_2.row >= curr_sheet.rows {
                                last_err_msg = String::from("Range end address row out of range"); //NOTE: Error messages are temporary.
                                continue 'mainloop;
                            }
                            if a_2.col >= curr_sheet.columns {
                                last_err_msg = String::from("Range end address column out of range"); //NOTE: Error messages are temporary.
                                continue 'mainloop;
                            }
                        },
                    }
                }

                let target_cell_rc = Rc::clone(&mut (curr_sheet.data[a.col as usize].borrow_mut()[a.row as usize]));
                let mut target_cell_ref = target_cell_rc.borrow_mut();
                old_func = (&target_cell_ref).cell_func.clone();
                (&mut target_cell_ref).cell_func = Some(CellFunc{expression: *b_ex, destination: Rc::downgrade(&target_cell_rc)});
            }

                evaluate(&mut sheets, &a, &old_func)?;                
            }

        }
        
        last_err_msg = String::from("ok");
    }

    Ok(())
}

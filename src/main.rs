pub mod ast;
pub mod tokens;
pub mod cell_operations;
pub mod evaluate_operations;
use lalrpop_util::lalrpop_mod;
use lalrpop_util::ParseError;
use logos::Logos;
use crate::cell_operations::{Cell, CellFunc, Sheet, ValueType};
use crate::evaluate_operations::evaluate;
use crate::tokens::LexicalError;
use crate::ast::{Expr,Addr, ParentType};
use std::io::{self, Write, BufWriter};
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp;
use std::time::Instant;
use std::fs::File;
// use serde::Serialize;
use csv::Reader;

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

struct Settings{
    cell_width: u32,
    formula_width: u32
}
impl Settings {
    fn new() -> Self {
        Settings{
            cell_width: 9,
            formula_width: 15
        }
    }
}

fn import_csv(csv_name: &str, sheet_idx: u32) -> Result<Sheet, String>
{
    let temp: String;
    if let Some(tempp) = csv_name.strip_suffix(".csv")
    {
        temp = tempp.to_string();
    }
    else
    {
        temp = csv_name.to_string();
    }
    
    let mut csv_data: Vec<Vec<String>> = vec![];
    if let Ok(mut rdr) = Reader::from_path(csv_name)
    {
        for result in rdr.records()
        {
            if let Ok(record) = result
            {
                let row: Vec<String> = record.iter().map(|s| s.to_string()).collect();
                csv_data.push(row);
            }
            else
            {
                return Err("Error reading csv".to_string());
            }
        }
        let sheet: Sheet = Sheet::new(sheet_idx, temp, csv_data[0].len() as u32, csv_data.len() as u32);
        for row in 0..csv_data.len()
        {
            for col in 0..csv_data[0].len()
            {
                if csv_data[row][col] == ""
                {
                    continue;
                }
                let mut cell = cell_operations::Cell::new(Addr{sheet: sheet_idx, row: row as u32, col: col as u32});
                let raw_val = csv_data[row][col].clone();

                if let Ok(val) = raw_val.parse::<i32>()
                {
                    cell.cell_func = Some(cell_operations::CellFunc::new(Expr::Integer(val)));
                    cell.valid = true;
                    cell.value = cell_operations::ValueType::IntegerValue(val);
                }

                else if let Ok(val) = raw_val.parse::<f64>()
                {
                    cell.cell_func = Some(cell_operations::CellFunc::new(Expr::Float(val)));
                    cell.valid = true;
                    cell.value = cell_operations::ValueType::FloatValue(val);
                }

                else if let Ok(val) = raw_val.parse::<bool>() 
                {
                    cell.cell_func = Some(cell_operations::CellFunc::new(Expr::Bool(val)));
                    cell.valid = true;
                    cell.value = cell_operations::ValueType::BoolValue(val);
                } 

                else 
                {
                    cell.cell_func = Some(cell_operations::CellFunc::new(Expr::String(raw_val.clone())));
                    cell.valid = true;
                    cell.value = cell_operations::ValueType::String(raw_val);
                }
                
                sheet.data[col].borrow_mut().cells[row] = Rc::new(RefCell::new(cell));
            }
        }
        Ok(sheet)
    }
    else 
    {
        return Err("Error reading csv".to_string());
    }
}

fn export_csv(sheet: &Sheet) -> Result<(), String> 
{
    if let Ok(file) = File::create(sheet.sheet_name.clone() + ".csv")
    {
        let mut writer = BufWriter::new(file);
        let mut csv_data : Vec<Vec<String>> = vec![];
        for col in &sheet.data
        {
            csv_data.push(vec![]);
            if col.borrow().cells.len() == 0
            {
                for _i in 0..sheet.rows
                {
                    if let Some(last) = csv_data.last_mut()
                    {
                        (*last).push("<EMPTY>".to_string());
                    }
                }
            }
            else
            {
                let curr_rows: usize = col.borrow().cells.len();
                let row: &Vec<Rc<RefCell<Cell>>> = &col.borrow().cells;
                for i in 0..curr_rows
                {
                    let value = Rc::clone(&row[i as usize]).borrow().value.clone();
                    if let Some(last) = csv_data.last_mut()
                    {
                        (*last).push(value.to_string());
                    }

                }
                for _i in curr_rows..sheet.rows as usize
                {
                    if let Some(last) = csv_data.last_mut()
                    {
                        (*last).push("<EMPTY>".to_string());
                    }
                }
            }
        }
        for row in 0..csv_data[0].len()
        {
            for col in 0..csv_data.len()
            {
                if csv_data[col][row] == "<EMPTY>"
                {
                    if let Ok(()) = write!(writer, "{}", "")
                    {}
                    else 
                    {
                        return Err("Error in writing csv".to_string());
                    }
                }
                if let Ok(()) = write!(writer, "{}", csv_data[col][row])
                {}
                else 
                {
                    return Err("Error in writing csv".to_string());
                }
                if row != csv_data[0].len()-1
                {
                    if let Ok(()) = write!(writer, ",")
                    {}
                    else 
                    {
                        return Err("Error in writing csv".to_string());
                    }
                }
            }
            if let Ok(()) = writeln!(writer)
            {}
            else 
            {
                return Err("Error in writing csv".to_string());
            }
        }

        Ok(())
    }
    else 
    {
        return Err("Error in creating csv".to_string());
    }
}


fn copy_cell_value(addr1:Addr, addr2:Addr, sheets: &Vec<Rc<RefCell<Sheet>>>)
{
    let sheet_ref = &sheets[addr1.sheet as usize];
    let sheet = sheet_ref.borrow();
    let column_ref = &sheet.data[addr1.col as usize];
    let column = column_ref.borrow();
    let cell_rc = Rc::clone(&column[addr1.row as usize]);
    // drop(column);
    let cell = cell_rc.borrow();
    let value = cell.value.clone();
    
    let sheet_ref2 = &sheets[addr2.sheet as usize];
    let sheet2 = sheet_ref2.borrow();
    let column_ref2 = &sheet2.data[addr2.col as usize];
    let column2 = column_ref2.borrow();
    let cell_rc2 = Rc::clone(&column2[addr2.row as usize]);
    // drop(column);
    let mut cell2 = cell_rc2.borrow_mut();
    
    cell2.value = value;
}

fn copy_range_value(addr1:Addr, addr2:Addr, sheets: &Vec<Rc<RefCell<Sheet>>>)
{
    for i in addr1.col..=addr2.col
    {
        for j in addr1.row..=addr2.row
        {
            copy_cell_value(Addr{sheet: addr1.sheet, row: j, col: i}, Addr{sheet: addr2.sheet, row: j, col: i}, sheets);
        }
    }
}

fn copy_cell_function(addr1:Addr, addr2:Addr, sheets: &Vec<Rc<RefCell<Sheet>>>)
{
    let sheet_ref = &sheets[addr1.sheet as usize];
    let sheet = sheet_ref.borrow();
    let column_ref = &sheet.data[addr1.col as usize];
    let column = column_ref.borrow();
    let cell_rc = Rc::clone(&column[addr1.row as usize]);
    // drop(column);
    let cell = cell_rc.borrow();
    let func = cell.cell_func.clone();
    
    let sheet_ref2 = &sheets[addr2.sheet as usize];
    let sheet2 = sheet_ref2.borrow();
    let column_ref2 = &sheet2.data[addr2.col as usize];
    let column2 = column_ref2.borrow();
    let cell_rc2 = Rc::clone(&column2[addr2.row as usize]);
    // drop(column);
    let mut cell2 = cell_rc2.borrow_mut();
    
    cell2.cell_func = func; 
}

fn copy_range_function(addr1:Addr, addr2:Addr, sheets: &Vec<Rc<RefCell<Sheet>>>)
{
    for i in addr1.col..=addr2.col
    {
        for j in addr1.row..=addr2.row
        {
            copy_cell_function(Addr{sheet: addr1.sheet, row: j, col: i}, Addr{sheet: addr2.sheet, row: j, col: i}, sheets);
        }
    }
}

fn duplicate_sheet(sheets: &mut Vec<Rc<RefCell<Sheet>>>, sheet_number: usize, sheet_name: String) -> Result<(),String>  // sheet_number and sheet_name correspond to the old sheet that has been copied
{
    if sheet_number >= sheets.len()
    {
        return Err("The sheet you are trying to copy does not exit".to_string());
    }
    let mut new_sheet = sheets[sheet_number].borrow().clone();
    new_sheet.sheet_name = sheet_name;
    new_sheet.sheet_idx = sheets.len() as u32;
    

    for col in new_sheet.data.iter()
    {
        for row in col.borrow().cells.iter()
        {
            let mut cell: std::cell::RefMut<'_, Cell> = row.borrow_mut();
            cell.addr.sheet = sheet_number as u32;
            let dep_list_ref: Option<&CellFunc> = cell.cell_func.as_ref();
            if let Some(dep_listt) = dep_list_ref
            {
                let dep_list: Vec<ParentType> = dep_listt.expression.get_dependency_list();
                for dep in dep_list.iter()
                {
                    match dep {
                        ParentType::Single(a_1) => {
                            let sheet_ref =&(*sheets)[a_1.sheet as usize];
                            let sheet = sheet_ref.borrow();
                            let column_ref = &sheet.data[a_1.col as usize];
                            let column = column_ref.borrow();
                            let cell_rc = Rc::clone(&column[a_1.row as usize]);
                            // drop(column);
                            let mut parent_cell = cell_rc.borrow_mut();
                            parent_cell.children.insert(cell.addr.clone());
                            if a_1.sheet == sheet_number as u32 
                            {
                                let old_addr = Addr{sheet: sheet_number as u32, row: cell.addr.row as u32, col: cell.addr.col as u32};
                                parent_cell.children.remove(&old_addr);
                            }
                        }
                        ParentType::Range(a_1, a_2) => {
                            for i in a_1.col..=a_2.col
                            {
                                for j in a_1.row..=a_2.row
                                {
                                    let sheet_ref =&(*sheets)[a_1.sheet as usize];
                                    let sheet = sheet_ref.borrow();
                                    let column_ref = &sheet.data[i as usize];
                                    let column = column_ref.borrow();
                                    let cell_rc = Rc::clone(&column[j as usize]);
                                    // drop(column);
                                    let mut parent_cell = cell_rc.borrow_mut();
                                    parent_cell.children.insert(cell.addr.clone());
                                    if a_1.sheet == sheet_number as u32 
                                    {
                                        let old_addr = Addr{sheet: sheet_number as u32, row: j as u32, col: i as u32};
                                        parent_cell.children.remove(&old_addr);
                                    }
                                }
                            }
                        },
                    }
                }

                
            }
            else
            {
                continue;
            }


            // idhar hum bache change karenge
            let mut toinsert = vec![];
            for address in cell.children.iter()
            {
                if address.sheet == sheet_number as u32
                {
                    toinsert.push(address.clone());
                }
            }
            
            cell.children.clear();

            for addr in toinsert.iter()
            {
                let new_addr = Addr{sheet: new_sheet.sheet_idx as u32, row: addr.row as u32, col: addr.col as u32};
                cell.children.insert(new_addr);
            }
            // aur idhar cell_func change karenge
            if let Some(func) = cell.cell_func.clone()
            { 
                let temp = update_cell_func(func.expression,sheet_number as u32, new_sheet.sheet_idx);            
                cell.cell_func = Some(CellFunc::new(temp));
            }
            
        }
    }
    Ok(())
}

fn update_cell_func(exp: Expr, sheet_num: u32, sheet_idx: u32) -> Expr
{
    match exp 
    {
        Expr::Bool(val) => Expr::Bool(val),
        Expr::Float(val) => Expr::Float(val),
        Expr::Integer(val) => Expr::Integer(val),
        Expr::String(val) => Expr::String(val),
        Expr::Wildcard => Expr::Wildcard,
        Expr::MonoOp(a,b) => 
        {
            let expr = update_cell_func(*b, sheet_num, sheet_idx);
            let exprbox = Box::new(expr);
            Expr::MonoOp(a,exprbox)
        },
        Expr::BinOp(a,b,c) =>
        {
            let expr1 = update_cell_func(*b, sheet_num, sheet_idx);
            let expr2 = update_cell_func(*c, sheet_num, sheet_idx);
            let exprbox1 = Box::new(expr1);
            let exprbox2 = Box::new(expr2);
            Expr::BinOp(a,exprbox1,exprbox2)
        },
        Expr::InfixOp(a,b,c) =>
        {
            let expr1 = update_cell_func(*a, sheet_num, sheet_idx);
            let expr2 = update_cell_func(*c, sheet_num, sheet_idx);
            let exprbox1 = Box::new(expr1);
            let exprbox2 = Box::new(expr2);
            Expr::InfixOp(exprbox1, b, exprbox2)
        },
        Expr::TernaryOp(a,b,c,d) =>
        {
            let expr1 = update_cell_func(*b, sheet_num, sheet_idx);
            let expr2 = update_cell_func(*c, sheet_num, sheet_idx);
            let expr3 = update_cell_func(*d, sheet_num, sheet_idx);
            let exprbox1 = Box::new(expr1);
            let exprbox2 = Box::new(expr2);
            let exprbox3 = Box::new(expr3);
            Expr::TernaryOp(a,exprbox1, exprbox2, exprbox3)
        },
        Expr::RangeOp{op, start, end, cond} =>
        {
            if sheet_num == start.sheet
            {
                let new_start = Addr{sheet:sheet_idx, row:start.row,col:start.col};
                let new_end = Addr{sheet:sheet_idx, row:end.row,col:end.col};
                let new_cond = update_cell_func(*cond, sheet_num, sheet_idx);
                Expr::RangeOp{op, start: new_start, end: new_end, cond: Box::new(new_cond)}
            }
            else 
            {
               Expr::RangeOp { op, start, end,cond }
            }
        }
        Expr::Cell(addr) => 
        {
            if addr.sheet == sheet_num
            {
                let new_addr = Addr{sheet:sheet_idx, row: addr.row, col: addr.col};
                Expr::Cell(new_addr)
            }
            else
            {
                Expr::Cell(addr)
            }
        }
        
    }
}

fn display_sheet(col: u32, row: u32, sheet: &Sheet, settings: &Settings, showformulas: bool)
{
    let row_max = cmp::min(row+10, sheet.rows);
    let col_max = cmp::min(col+10, sheet.columns);
    let width = settings.cell_width as usize;
    
    print!("      ");
    for i in col..col_max {
        let mut curr = String::new();
        let mut curr_col = i + 1;
        while curr_col > 0
        {

            curr.push(((b'A') + ((curr_col-1) % 26) as u8) as char);
            
            curr_col -= 1;
            curr_col /= 26;
        }
        print!("{:>width$}", curr.chars().rev().collect::<String>());
    }
    println!();
    for i in row..row_max {
        print!("{:>width$}", i+1);
        for j in col..col_max {

            if showformulas
            {
                sheet.expr_at(j as usize, i as usize, settings.formula_width as usize);
            }
            else
            {
                let colref = sheet.data[j as usize].borrow();
                if i as usize >= colref.cells.len()
                {
                    print!("{:>width$}", "~");
                    continue
                } 
                else
                {
                    let cell = colref.cells[i as usize].borrow();
                    if cell.valid {
                        let val =  &cell.value;
                        match val {
                            ValueType::BoolValue(b) => print!("{:>width$}", b),
                            ValueType::IntegerValue(x) => print!("{:>width$}", x),
                            ValueType::FloatValue(n) => print!("{:>width$.2}", n, width = width),
                            ValueType::String(s) => print!("{:>width$}", s),
                        }
                    }
                    else {
                        print!("{:>width$}", "~ERR")
                    }
                }  
            }
        }
        println!()
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>>{

    let r: u32 = std::env::args().nth(1)
        .expect("Row number not entered (First arg missing)")
        .parse().expect("Invalid input for Row number (First arg)");
    
    let c: u32 = std::env::args().nth(2)
        .expect("Column number not entered (Second arg missing)")
        .parse().expect("Invalid input for Column number (Second arg)");

    // let r: u32 = 3; ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////NOTE: For testing, remove later
    // let c: u32 = 3; ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////NOTE: For testing, remove later
    //sheets: &Vec<Rc<RefCell<Sheet>>>

    let mut sheets: Vec<Rc<RefCell<Sheet>>> = vec![Rc::new(RefCell::new(Sheet::new(0, String::from("sheet0"), c, r)))];

    let mut exit : bool = false;

    let mut curr_col: usize= 0;
    let mut curr_row: usize = 0;
    let mut show_window: bool = true;
    let mut last_err_msg = String::from("ok");
    let settings = Settings::new();
    let mut last_time = 0;
    'mainloop: while !exit {
        let mut start = Instant::now();
        if show_window {
            // let curr_sheet = ;
            display_sheet(curr_col as u32, curr_row as u32, &sheets[0].borrow(),  &settings, false);
        }
        let mut inp = String::new();
        print!("[{}.0] ", last_time);
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
            Err(ParseError::User{error: LexicalError::InvalidToken}) => 
            {
                last_err_msg = String::from("Invalid Token"); 
                last_time = 0;
                continue
            },
            Err(ParseError::User{error: LexicalError::InvalidInteger(x)}) => 
            {   
                last_err_msg = format!("Invalid Integer {:?}", x); 
                last_time = 0;
                continue
            }, 
            Err(e) => 
            {
                last_err_msg = format!("This error: {:?}", e); 
                last_time = 0;
                continue
            }
        };
        // println!("{:?}", dep_vec);
        // println!("{:?}", ast);


        match ast {
            ast::Command::DisplayCmd(d_cmd) => {
                let curr_sheet = &sheets[0].borrow();
                match d_cmd {
                    ast::DisplayCommand::EnableOut => show_window = true,
                    ast::DisplayCommand::DisableOut => show_window = false,
                    ast::DisplayCommand::ScrollTo(addr) => 
                    {
                        if (addr.row >= curr_sheet.rows) | (addr.col >= curr_sheet.columns) {
                            last_time = 0;
                            last_err_msg = String::from("Address out of bounds");
                            continue
                        }
                        curr_col = cmp::max(0, cmp::min(addr.col as i64, curr_sheet.columns as i64 - 10)) as usize;
                        curr_row = cmp::max(0, cmp::min(addr.row as i64, curr_sheet.rows as i64 - 10)) as usize; 
                    },

                    ast::DisplayCommand::MoveUp => curr_row = cmp::max(0, cmp::min(curr_row as i64 -1 , curr_sheet.rows as i64 - 10)) as usize,
                    ast::DisplayCommand::MoveDown => curr_row = cmp::max(0, cmp::min(curr_row as i64 +1 , curr_sheet.rows as i64 - 10)) as usize,
                    ast::DisplayCommand::MoveRight => curr_col = cmp::max(0, cmp::min(curr_col as i64 +1 , curr_sheet.columns as i64 - 10)) as usize,
                    ast::DisplayCommand::MoveLeft => curr_col = cmp::max(0, cmp::min(curr_col as i64 -1 , curr_sheet.columns as i64 - 10)) as usize,
                }},
            ast::Command::Quit => exit = true,
            ast::Command::AssignCmd(a, b_ex) => {  //NOTE: All validity checks for addresses will be more complicated when we implement multiple sheets.
                let old_func: Option<CellFunc>;
                {
                let curr_sheet = &sheets[0].borrow();
                if a.row >= curr_sheet.rows {
                    last_time = 0;
                    last_err_msg = String::from("Target address row out of range"); //NOTE: Error messages are temporary.
                    continue 'mainloop;
                }
                if a.col >= curr_sheet.columns {
                    last_time = 0;
                    last_err_msg = String::from("Target address column out of range"); //NOTE: Error messages are temporary.
                    continue 'mainloop;
                }
                let mut col = curr_sheet.data[a.col as usize].borrow_mut();
                if col.cells.len() <= a.row as usize
                {
                    let mut p = col.cells.len() as u32;
                    col.cells.resize_with(a.row as usize + 1, || {p += 1; Rc::new(RefCell::new(Cell::new(ast::Addr{sheet: curr_sheet.sheet_idx, row: p, col: a.col})))});
                }
                drop(col);

                for dep in &dep_vec {
                    match dep {
                        ast::ParentType::Single(a_1) => {
                            if a_1.row >= curr_sheet.rows {
                                last_time = 0;
                                last_err_msg = String::from("Address row out of range"); //NOTE: Error messages are temporary.
                                continue 'mainloop;
                            }
                            if a_1.col >= curr_sheet.columns {
                                last_time = 0;
                                last_err_msg = String::from("Address column out of range"); //NOTE: Error messages are temporary.
                                continue 'mainloop;
                            }
                            let mut col = curr_sheet.data[a_1.col as usize].borrow_mut();
                            if col.cells.len() <= a_1.row  as usize
                            {
                                let mut p = col.cells.len() as u32;
                                col.cells.resize_with(a_1.row as usize + 1, || {p += 1; Rc::new(RefCell::new(Cell::new(ast::Addr{sheet: curr_sheet.sheet_idx, row: p, col: a_1.col})))});
                            }
                            drop(col);
                        },
                        ast::ParentType::Range(a_1, a_2) => {
                            if a_1.row >= curr_sheet.rows {
                                last_time = 0;
                                last_err_msg = String::from("Range start address row out of range"); //NOTE: Error messages are temporary.
                                continue 'mainloop;
                            }
                            if a_1.col >= curr_sheet.columns {
                                last_time = 0;
                                last_err_msg = String::from("Range start address column out of range"); //NOTE: Error messages are temporary.
                                continue 'mainloop;
                            }
                            if a_2.row >= curr_sheet.rows {
                                last_time = 0;
                                last_err_msg = String::from("Range end address row out of range"); //NOTE: Error messages are temporary.
                                continue 'mainloop;
                            }
                            if a_2.col >= curr_sheet.columns {
                                last_time = 0;
                                last_err_msg = String::from("Range end address column out of range"); //NOTE: Error messages are temporary.
                                continue 'mainloop;
                            }
                            if a_1.col > a_2.col {
                                last_time = 0;
                                last_err_msg = String::from("Range start column higher than end column"); //NOTE: Error messages are temporary.
                                continue 'mainloop;
                            }
                            if a_1.row > a_2.row {
                                last_time = 0;
                                last_err_msg = String::from("Range start row higher than end row"); //NOTE: Error messages are temporary.
                                continue 'mainloop;
                            }
                            for i in a_1.col..=a_2.col {
                                let mut col = curr_sheet.data[i as usize].borrow_mut();
                                if col.cells.len() <= a_2.row as usize
                                {
                                    let mut p = col.cells.len() as u32;
                                    col.cells.resize_with(a_2.row as usize + 1, || {p += 1; Rc::new(RefCell::new(Cell::new(ast::Addr{sheet: curr_sheet.sheet_idx, row: p, col: i})))});
                                }
                                drop(col);
                            }
                        },
                    }
                }

                let target_cell_rc = Rc::clone(& (curr_sheet.data[a.col as usize].borrow_mut()[a.row as usize]));
                let mut target_cell_ref = target_cell_rc.borrow_mut();
                old_func = (target_cell_ref).cell_func.clone();
                (target_cell_ref).cell_func = Some(CellFunc{expression: *b_ex});
                // println!("{}", target_cell_rc.try_borrow_mut().is_ok());
                drop(target_cell_ref);

            }
            start = Instant::now();
                // println!("{}", Rc::clone(& (&sheets[0].borrow().data[a.col as usize].borrow_mut()[a.row as usize])).try_borrow_mut().is_ok());
                if let Err(strr) = evaluate(&mut sheets, &a, &old_func)
                {
                    last_time = start.elapsed().as_secs();
                    last_err_msg = strr;
                    continue 'mainloop;
                    
                }              
            }

        }
        last_time = start.elapsed().as_secs();
        last_err_msg = String::from("ok");
    }

    Ok(())
}

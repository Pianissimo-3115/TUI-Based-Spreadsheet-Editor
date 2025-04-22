pub mod ast;
pub mod tokensexpr;
pub mod tokenscmds;
pub mod cell_operations;
pub mod evaluate_operations;
use lalrpop_util::lalrpop_mod;
use lalrpop_util::ParseError;
use logos::Logos;
use crate::cell_operations::{Cell, CellFunc, Sheet, ValueType};
use crate::evaluate_operations::evaluate;
// use crate::tokenscmds;
// use crate::tokensexpr;
use std::io::{self, Write, BufWriter};
use std::rc::Rc;
use std::cell::{RefCell, Ref};
use std::cmp;
use std::time::Instant;
use std::fs::File;
// use serde::Serialize;
use csv::Reader;

//NOTE: PLEASE HAR JAGA usize KAR DO, bohot zyada conversions karne pad rahe hai


lalrpop_mod!(pub grammarexpr); // include the generated parser
lalrpop_mod!(pub grammarcmds); // include the generated parser

/* STUFF TO DO:
0 - initialise
0 - ask input
0 - If error, report respective error, restart
0 - If not error:
    0 - Check required cells:
        0 - If out of range, report error (In extension: Suggest resizing)
        0 - Initialise if not done yet (should now happen automatically)
    0 - Give new function to - cell
    0 - Give old and new function to - evaluate()
    0 - If result is error (loop) then report error.
restart
*/

pub struct SheetStorage {
    pub map: Vec<(String, usize)>,
    pub data: Vec<Rc<RefCell<Sheet>>>   //NOTE: This should be made int Option<Rc<...>>
}

impl SheetStorage {
    pub fn new() -> Self {
        SheetStorage{
            map: vec![],
            data: vec![]
        }
    }

    pub fn numFromName(&self, name: &str) -> Option<usize> {
        for (curr_name, num) in &self.map {
            if curr_name == name {
                return Some(*num);
            }
        };
        return None
    }

    pub fn newSheet(&mut self, name: &str, cols: usize, rows: usize) -> Option<usize> {
        for (curr_name, _num) in &self.map {
            if curr_name == name {
                return None;
            }
        };
        let new_num = self.data.len();
        let new_sheet_ref = RefCell::new(Sheet::new(new_num as u32, cols as u32, rows as u32));
        self.data.push(Rc::new(new_sheet_ref));
        self.map.push((String::from(name), new_num));
        return Some(new_num)
    }

    pub fn addSheet(&mut self, name: &str, sheet: Sheet) -> Option<usize> { //Assumes that sheet_idx would be same as data.len()
        for (curr_name, _num) in &self.map {
            if curr_name == name {
                return None;
            }
        };
        let new_num = self.data.len();
        let new_sheet_ref = RefCell::new(sheet);
        self.data.push(Rc::new(new_sheet_ref));
        self.map.push((String::from(name), new_num));
        return Some(new_num)
    }

    pub fn removeSheet(&mut self, name: &str) -> Option<usize> {

        for i in 0..self.map.len() {
            if self.map[i].0 == name {
                let removed_num = self.map[i].1;
                self.data[removed_num] = Rc::new(RefCell::new(Sheet::new(999, 0, 0))); //NOTE: This very bad bad fix later
                self.map.remove(i);
                return Some(removed_num)
            }
        };
        return None;
    }
    pub fn renameSheet(&mut self, name: &str, name_new: &str) -> Option<usize> {

        for i in 0..self.map.len() {
            if self.map[i].0 == name {
                let renamed_num = self.map[i].1;
                if self.numFromName(name_new).is_none() {
                    self.map[i] = (String::from(name_new), renamed_num);
                    return Some(renamed_num)
                } else { return None }
            }
        };
        return None;
    }
}


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

// fn import_csv(csv_name: &str, sheet_idx: u32) -> Result<Sheet, String>
// {

//     let csv_data: Vec<Vec<String>> = vec![];
//     if let Ok(rdr) = Reader::from_path(csv_name)
//     {
//         for result in rdr.records()
//         {
//             if let Ok(record) = result
//             {
//                 let row = record.iter().map(|s| s.to_string()).collect();
//                 csv_data.push(row);
//             }
//             else
//             {
//                 return Err("Error reading csv".to_string());
//             }
//         }
//         let mut sheet = Sheet::new(sheet_idx, csv_data[0].len() as u32, csv_data.len() as u32);
//         for row in 0..csv_data.len()
//         {
//             for col in 0..csv_data[0].len()
//             {
//                 if csv_data[row][col] == ""
//                 {
//                     continue;
//                 }
//                 let mut cell = cell_operations::Cell::new(ast::Addr{sheet: sheet_idx, row: row as u32, col: col as u32});
//                 let raw_val = csv_data[row][col];
//                 if let Ok(val) = raw_val.parse::<i32>()
//                 {
//                     cell.cell_func = Some(cell_operations::CellFunc::new(ast::Expr::Integer(val)));
//                     cell.valid = true;
//                     cell.value = cell_operations::ValueType::IntegerValue(val);
//                 }
//                 else if let Ok(val) = raw_val.parse::<f64>()
//                 {
//                     cell.cell_func = Some(cell_operations::CellFunc::new(ast::Expr::Float(val)));
//                     cell.valid = true;
//                     cell.value = cell_operations::ValueType::FloatValue(val);
//                 }
//                 else if let Ok(val) = raw_val.parse::<bool>() {
//                     cell.cell_func = Some(cell_operations::CellFunc::new(ast::Expr::Bool(val)));
//                     cell.valid = true;
//                     cell.value = cell_operations::ValueType::BoolValue(val);
//                 } else {
//                     cell.cell_func = Some(cell_operations::CellFunc::new(ast::Expr::String(raw_val.clone())));
//                     cell.valid = true;
//                     cell.value = cell_operations::ValueType::String(raw_val);
//                 }
//                 sheet.data[col].borrow_mut().cells[row] = Rc::new(RefCell::new(cell));
//             }
//         }
//     }
//     else 
//     {
//         return Err("Error reading csv".to_string());
//     }
//     return Err("Not implemented.".to_string());
// }

fn export_csv(sheet: &Sheet, name: &str) -> Result<(), String> 
{
    if let Ok(file) = File::create(String::from(name) + ".csv")
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

    let mut sheetstore = SheetStorage::new();
    sheetstore.newSheet("sheet0", c as usize, r as usize);

    // let mut sheets: Vec<Rc<RefCell<Sheet>>> = vec![Rc::new(RefCell::new(Sheet::new(0, String::from("sheet0"), c, r)))];

    let mut exit : bool = false;

    let mut curr_col: usize= 0;
    let mut curr_row: usize = 0;
    let mut curr_sheet_number: usize = 0;
    let mut show_window: bool = true;
    let mut last_err_msg = String::from("ok");
    let settings = Settings::new();
    let mut last_time = 0;
    'mainloop: while !exit {
        let mut start = Instant::now();
        if show_window {
            // let curr_sheet = ;
            display_sheet(curr_col as u32, curr_row as u32, &sheetstore.data[curr_sheet_number].borrow(),  &settings, false);
        }
        let mut inp = String::new();
        print!("[{}.0] ", last_time);
        print!("({}) >> ", last_err_msg);
        io::stdout().flush().unwrap();

        io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read line"); //NOTE (to self): Better error message

        let ast;
        let dep_vec;

        if inp.len() == 0 {
            continue 'mainloop
        }
        if inp.chars().next() == Some(':') {
            let inp_smol = inp.chars().skip(1).collect::<String>();
            let lexer = tokenscmds::Token::lexer(&inp_smol).spanned()
            .map(|(token_result, span)| {
                let token = token_result?; // Propagate LexicalError
                Ok((span.start, token, span.end)) // (usize, Token, usize)
            });
            let parser = grammarcmds::CommandParser::new();
            (ast, dep_vec) = match parser.parse(curr_sheet_number as u32, &sheetstore, lexer) {  //NOTE: Error messages are temporary.
                Ok(x) => x,
                Err(ParseError::User{error: tokenscmds::LexicalError::InvalidToken}) => 
                {
                    last_err_msg = String::from("Invalid Token"); 
                    last_time = 0;
                    continue
                },
                Err(ParseError::User{error: tokenscmds::LexicalError::InvalidInteger(x)}) => 
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
        }
        else
        {
            let lexer = tokensexpr::Token::lexer(&inp).spanned()
            .map(|(token_result, span)| {
                let token = token_result?; // Propagate LexicalError
                Ok((span.start, token, span.end)) // (usize, Token, usize)
            });
            let parser = grammarexpr::AssignParser::new();
            (ast, dep_vec) = match parser.parse(curr_sheet_number as u32, &sheetstore, lexer) {  //NOTE: Error messages are temporary.
                Ok(x) => x,
                Err(ParseError::User{error: tokensexpr::LexicalError::InvalidToken}) => 
                {
                    last_err_msg = String::from("Invalid Token"); 
                    last_time = 0;
                    continue
                },
                Err(ParseError::User{error: tokensexpr::LexicalError::InvalidInteger(x)}) => 
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
        }


        // println!("{:?}", dep_vec);
        // println!("{:?}", ast);


        match ast {
            ast::Command::OtherCmd(cmd) => { match cmd {
                    ast::OtherCommand::AddSheet(s, c, r) => {
                        let res = sheetstore.newSheet(s.as_str(), c, r);
                        if res.is_none() {
                            last_err_msg = format!("Sheet name \"{}\" already exists.", s);
                        } else { last_err_msg = String::from("ok") }
                    }
                    ast::OtherCommand::RemoveSheet(s) => {
                        let res = sheetstore.removeSheet(s.as_str());
                        if res.is_none() {
                            last_err_msg = format!("Sheet name \"{}\" not found.", s);
                        } else { last_err_msg = String::from("ok") }
                    }
                    ast::OtherCommand::RenameSheet(s, snew) => {
                        let res = sheetstore.renameSheet(s.as_str(), snew.as_str());
                        if res.is_none() {
                            last_err_msg = format!("Either Sheet name \"{}\" not found OR Sheet name \"{}\" already exists.", s, snew);
                        } else { last_err_msg = String::from("ok") }
                    }
                    ast::OtherCommand::DuplicateSheet(s, snew_op) => todo!(),
                    
                    ast::OtherCommand::ExportCsv(s) => {
                        let s_num = sheetstore.numFromName(s.as_str());
                        match s_num {
                            Some(x) => {
                                export_csv(&sheetstore.data[x].borrow(), s.as_str());
                                last_err_msg = String::from("ok");
                            }
                            None => last_err_msg = format!("Sheet name \"{}\" not found.", s)
                        }
                    }
                    ast::OtherCommand::LoadCsv(path, opt_s) => todo!(), //{
                    //     match opt_s {
                    //         None => {
                    //             let name_opt = path.strip_suffix(".csv");
                    //             match name_opt {
                    //                 Some(name) => {
                    //                     if sheetstore.numFromName(name).is_none() {
                    //                         let imp_result = import_csv(&path, sheetstore.data.len() as u32);
                    //                         match imp_result {
                    //                             Ok(x) => {
                    //                                 sheetstore.addSheet(name, x); //Since we have alreayd verified that name does not exist already, this should happen successfully
                    //                                 last_err_msg = String::from("ok");
                    //                             },
                    //                             Err(e) => last_err_msg = format!("Error occured during import: {}", e)
                    //                         }
                    //                     }
                    //                     else {
                    //                         last_err_msg = format!("Sheet name \"{}\" already exist.", name)
                    //                     }
                    //                 },
                    //                 None => last_err_msg = format!("Invalid filepath (does not end in .csv): \"{}\"", path)
                    //             }
                    //         },
                    //         Some(name) => {
                    //             if sheetstore.numFromName(name.as_str()).is_none() {
                    //                 let imp_result = import_csv(name.as_str(), sheetstore.data.len() as u32);
                    //                 match imp_result {
                    //                     Ok(x) => {
                    //                         sheetstore.addSheet(name.as_str(), x); //Since we have alreayd verified that name does not exist already, this should happen successfully
                    //                         last_err_msg = String::from("ok");
                    //                     },
                    //                     Err(e) => last_err_msg = format!("Error occured during import: {}", e)
                    //                 }
                    //             }
                    //             else {
                    //                 last_err_msg = format!("Sheet name \"{}\" already exist.", name)
                    //             };
                    //         }
                    //     }
                    // }
                    ast::OtherCommand::Resize(s, c, r) => {
                        match sheetstore.numFromName(s.as_str()) {
                            Some(sheet_num) => {
                                sheetstore.data[sheet_num].borrow_mut().resize(r, c);  //NOTE: r aur c ka order har jag asame kar dena chahiye ajeeb lag raha
                                last_err_msg = String::from("ok");
                            } 
                            None => last_err_msg = format!("Sheet name \"{}\" not found.", s)
                        }
                    }  
                };
                continue
            }
            ast::Command::DisplayCmd(d_cmd) => {
                let curr_sheet = &sheetstore.data[curr_sheet_number].borrow();
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
                        curr_sheet_number = addr.sheet as usize;
                        let curr_sheet = &sheetstore.data[curr_sheet_number].borrow();
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
                let cell_sheet = &sheetstore.data[a.sheet].borrow();
                if a.row >= cell_sheet.rows {
                    last_time = 0;
                    last_err_msg = String::from("Target address row out of range"); //NOTE: Error messages are temporary.
                    continue 'mainloop;
                }
                if a.col >= cell_sheet.columns {
                    last_time = 0;
                    last_err_msg = String::from("Target address column out of range"); //NOTE: Error messages are temporary.
                    continue 'mainloop;
                }
                let mut col = cell_sheet.data[a.col as usize].borrow_mut();
                if col.cells.len() <= a.row as usize
                {
                    let mut p = col.cells.len() as u32;
                    col.cells.resize_with(a.row as usize + 1, || {p += 1; Rc::new(RefCell::new(Cell::new(ast::Addr{sheet: cell_sheet.sheet_idx, row: p, col: a.col})))});
                }
                drop(col);

                for dep in &dep_vec {
                    match dep {
                        ast::ParentType::Single(a_1) => {
                            let cell_sheet = &sheetstore.data[a_1.sheet as usize].borrow();
                            if a_1.row >= cell_sheet.rows {
                                last_time = 0;
                                last_err_msg = String::from("Address row out of range"); //NOTE: Error messages are temporary.
                                continue 'mainloop;
                            }
                            if a_1.col >= cell_sheet.columns {
                                last_time = 0;
                                last_err_msg = String::from("Address column out of range"); //NOTE: Error messages are temporary.
                                continue 'mainloop;
                            }
                            let mut col = cell_sheet.data[a_1.col as usize].borrow_mut();
                            if col.cells.len() <= a_1.row  as usize
                            {
                                let mut p = col.cells.len() as u32;
                                col.cells.resize_with(a_1.row as usize + 1, || {p += 1; Rc::new(RefCell::new(Cell::new(ast::Addr{sheet: cell_sheet.sheet_idx, row: p, col: a_1.col})))});
                            }
                            drop(col);
                        },
                        ast::ParentType::Range(a_1, a_2) => {
                            
                            let cell_sheet = { 
                                if a_1.sheet == a_2.sheet {
                                    &sheetstore.data[a_1.sheet as usize].borrow()
                                }
                                else {
                                    last_time = 0;
                                    last_err_msg = String::from("Range addresses must belong to the same sheet.");
                                    continue 'mainloop
                                }
                            };

                            if a_1.row >= cell_sheet.rows {
                                last_time = 0;
                                last_err_msg = String::from("Range start address row out of range"); //NOTE: Error messages are temporary.
                                continue 'mainloop;
                            }
                            if a_1.col >= cell_sheet.columns {
                                last_time = 0;
                                last_err_msg = String::from("Range start address column out of range"); //NOTE: Error messages are temporary.
                                continue 'mainloop;
                            }
                            if a_2.row >= cell_sheet.rows {
                                last_time = 0;
                                last_err_msg = String::from("Range end address row out of range"); //NOTE: Error messages are temporary.
                                continue 'mainloop;
                            }
                            if a_2.col >= cell_sheet.columns {
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
                                let mut col = cell_sheet.data[i as usize].borrow_mut();
                                if col.cells.len() <= a_2.row as usize
                                {
                                    let mut p = col.cells.len() as u32;
                                    col.cells.resize_with(a_2.row as usize + 1, || {p += 1; Rc::new(RefCell::new(Cell::new(ast::Addr{sheet: cell_sheet.sheet_idx, row: p, col: i})))});
                                }
                                drop(col);
                            }
                        },
                    }
                }

                let target_sheet = &sheetstore.data[a.sheet as usize].borrow();
                let target_cell_rc = Rc::clone(& (target_sheet.data[a.col as usize].borrow_mut()[a.row as usize]));
                let mut target_cell_ref = target_cell_rc.borrow_mut();
                old_func = (target_cell_ref).cell_func.clone();
                (target_cell_ref).cell_func = Some(CellFunc{expression: *b_ex});
                // println!("{}", target_cell_rc.try_borrow_mut().is_ok());
                drop(target_cell_ref);

            }
            start = Instant::now();
                // println!("{}", Rc::clone(& (&sheets[0].borrow().data[a.col as usize].borrow_mut()[a.row as usize])).try_borrow_mut().is_ok());
                if let Err(strr) = evaluate(&mut sheetstore.data, &a, &old_func)
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

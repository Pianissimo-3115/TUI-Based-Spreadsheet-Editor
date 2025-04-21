// use std::fmt::Error;
// use std::ptr::with_exposed_provenance;
// use std::cmp;
use std::thread;
use std::time::Duration;
use crate::ast::{Addr, BinaryFunction, Expr, MonoFunction, ParentType, RangeFunction};
use crate::cell_operations::{Sheet,Cell,CellFunc,ValueType,Column};
#[allow(unused_imports)]
use std::rc::{Rc, Weak};
#[allow(unused_imports)]
use std::cell::RefCell;
use std::collections::HashMap;
// use crate::cell_operations::CellFunc;
fn min_eval(data: &[RefCell<Column>], range: ((u32,u32),(u32,u32))) -> Result<ValueType, String> 
{
    
    let cell1: (u32, u32) = range.0;
    let cell2: (u32, u32) = range.1;
    let mut mini = f64::MAX;
    let mut isfloat = false;
    for col in cell1.1..=cell2.1 
    {
        for row in cell1.0..=cell2.0 
        {
            
            let temp1: std::cell::Ref<'_, Column> = data[col as usize].borrow();
            let temp2: Rc<RefCell<Cell>> = Rc::clone(&temp1[row as usize]);
            let temp: std::cell::Ref<'_, Cell> = temp2.borrow();
            if !temp.valid 
            {
                return Err(format!("Invalid cell at ({}, {})", col, row));
            }
            else if let ValueType::IntegerValue(value) = temp.value 
            {
                if (value as f64) < mini 
                {
                    mini = value as f64;
                }
            }
            else if let ValueType::FloatValue(value) = temp.value 
            {
                isfloat = true;
                if value < mini 
                {
                    mini = value;
                }
            }
            else 
            {
                return Err(format!("cell at ({}, {}) does not have numeral Type, but used in MIN function", col, row));
            }            
        }
    }
    if isfloat
    {
        Ok(ValueType::FloatValue(mini))
    }
    else 
    {
        Ok(ValueType::IntegerValue(mini as i32))
    }
}

fn max_eval(data: &[RefCell<Column>], range: ((u32,u32),(u32,u32))) -> Result<ValueType, String> 
{
    
    let cell1: (u32, u32) = range.0;
    let cell2: (u32, u32) = range.1;
    let mut maxi = f64::MIN;
    let mut isfloat = false;
    for col in cell1.1..=cell2.1 
    {
        for row in cell1.0..=cell2.0 
        {
            
            let temp1: std::cell::Ref<'_, Column> = data[col as usize].borrow();
            let temp2: Rc<RefCell<Cell>> = Rc::clone(&temp1[row as usize]);
            let temp: std::cell::Ref<'_, Cell> = temp2.borrow();
            if !temp.valid 
            {
                return Err(format!("Invalid cell at ({}, {})", col, row));
            }
            else if let ValueType::IntegerValue(value) = temp.value 
            {
                if (value as f64) > maxi 
                {
                    maxi = value as f64;
                }
            }
            else if let ValueType::FloatValue(value) = temp.value 
            {
                isfloat = true;
                if value > maxi 
                {
                    maxi = value;
                }
            }
            else 
            {
                return Err(format!("cell at ({}, {}) does not have numeral Type, but used in MAX function", col, row));
            }            
        }
    }
    if isfloat 
    {
        Ok(ValueType::FloatValue(maxi))
    }
    else 
    {
        Ok(ValueType::IntegerValue(maxi as i32))
    }
}

fn sum_eval(data: &[RefCell<Column>], range: ((u32,u32),(u32,u32))) -> Result<ValueType, String> 
{
    
    let cell1: (u32, u32) = range.0;
    let cell2: (u32, u32) = range.1;
    let mut summ = 0 as f64;
    let mut isfloat = false;
    for col in cell1.1..=cell2.1 
    {
        for row in cell1.0..=cell2.0 
        {
            
            let temp1: std::cell::Ref<'_, Column> = data[col as usize].borrow();
            let temp2: Rc<RefCell<Cell>> = Rc::clone(&temp1[row as usize]);
            let temp: std::cell::Ref<'_, Cell> = temp2.borrow();
            if !temp.valid
            {
                return Err(format!("Invalid cell at ({}, {})", col, row));
            }
            else if let ValueType::IntegerValue(value) = temp.value 
            {
                summ += value as f64;
            }
            else if let ValueType::FloatValue(value) = temp.value 
            {
                isfloat = true;
                summ+= value;
            }
            else 
            {
                return Err(format!("cell at ({}, {}) does not have numeral Type, but used in SUM function", col, row));
            }        
        }
    }
    if isfloat 
    {
        Ok(ValueType::FloatValue(summ))
    }
    else 
    {
        Ok(ValueType::IntegerValue(summ as i32))
    }
}

fn avg_eval(data: &[RefCell<Column>], range: ((u32,u32),(u32,u32))) -> Result<ValueType, String> 
{
    
    let cell1: (u32, u32) = range.0;
    let cell2: (u32, u32) = range.1;
    let mut summ = 0 as f64;
    let mut count = 0;
    for col in cell1.1..=cell2.1 
    {
        for row in cell1.0..=cell2.0 
        {
            
            let temp1: std::cell::Ref<'_, Column> = data[col as usize].borrow();
            let temp2: Rc<RefCell<Cell>> = Rc::clone(&temp1[row as usize]);
            let temp: std::cell::Ref<'_, Cell> = temp2.borrow();
            if !temp.valid
            {
                return Err(format!("Invalid cell at ({}, {})", col, row));
            }
            else if let ValueType::IntegerValue(value) = temp.value 
            {
                summ += value as f64;
                count += 1;
            }
            else if let ValueType::FloatValue(value) = temp.value 
            {
                summ += value;
                count += 1; 
            }
            else 
            {
                return Err(format!("cell at ({}, {}) does not have numeral Type, but used in AVG function", col, row));
            }        
        }
    }
    if count == 0 
    {
        Err("No valid cells in range".to_string())
    } 
    else 
    {
        Ok(ValueType::FloatValue(summ / (count as f64)))
    }
}

fn stdev_eval(data: &[RefCell<Column>], range: ((u32,u32),(u32,u32))) -> Result<ValueType, String> 
{
    let cell1: (u32, u32) = range.0;
    let cell2: (u32, u32) = range.1;
    let mut summ = 0 as f64;
    let mut count = 0;
    for col in cell1.1..=cell2.1 
    {
        for row in cell1.0..=cell2.0 
        {
            
            let temp1 = data[col as usize].borrow();
            let temp2 = Rc::clone(&temp1[row as usize]);
            let temp = temp2.borrow();
            if !temp.valid 
            {
                return Err(format!("Invalid cell at ({}, {})", col, row));
            }
            else if let ValueType::IntegerValue(value) = temp.value 
            {
                summ += value as f64;
                count += 1;
            }
            else if let ValueType::FloatValue(value) = temp.value 
            {
                summ += value;
                count += 1; 
            }
            else 
            {
                return Err(format!("cell at ({}, {}) does not have numeral Type, but used in AVG function", col, row));
            }        
        }
    }

    if count == 0 {
        return Err("No valid cells in range".to_string());
    }

    let mean = summ / (count as f64);
    let mut sum_squared_diff = 0.0;

    for col in cell1.1..=cell2.1 {
        for row in cell1.0..=cell2.0 {
            let temp1: std::cell::Ref<'_, Column> = data[col as usize].borrow();
            let temp2: Rc<RefCell<Cell>> = Rc::clone(&temp1[row as usize]);
            let temp: std::cell::Ref<'_, Cell> = temp2.borrow();
            if !temp.valid
            {
                return Err(format!("Invalid cell at ({}, {})", col, row));
            }
            else if let ValueType::IntegerValue(value) = temp.value 
            {
                sum_squared_diff += (value as f64 - mean).powi(2);
            }
            else if let ValueType::FloatValue(value) = temp.value 
            {
                sum_squared_diff += (value - mean).powi(2);
            }
            else 
            {
                return Err(format!("cell at ({}, {}) does not have numeral Type, but used in STDEV function", col, row));
            } 
        }
    }

    let stdev = (sum_squared_diff / (count as f64)).sqrt();
    Ok(ValueType::FloatValue(stdev))
}

fn sleep(seconds: f64)
{
    thread::sleep(Duration::from_secs_f64(seconds));
}

fn remove_old_dependencies(cell: &Addr,sheets: &mut Vec<Rc<RefCell<Sheet>>>, dependencies: Vec<ParentType>) -> Result<(),String>       // DEPENDENCIES OF OLD_FUNC
{
    for i in dependencies
    {
        match i 
        {
            ParentType::Single(addr) => 
            {
                // if let Addr { sheet, row, col } = addr 
                // {
                //     let temp1 = sheets[current_sheet as usize].borrow().clone();
                //     let temp2 = temp1.as_ref();
                //     let c = temp2.data[col as usize].borrow();
                //     let cell_rc = &c[row as usize];
                //     let mut parent_cell = cell_rc.borrow_mut();
                //     let addr_of_cell = cell.upgrade().unwrap().borrow().addr.clone();

                //     parent_cell.children.remove(&(cell.upgrade().unwrap().borrow().addr));       

                
                // } 
                let Addr { sheet:sheet_num, row, col } = addr;
                let sheet_ref = &(*sheets)[sheet_num as usize];
                let sheet = sheet_ref.borrow();

                let column_ref = &sheet.data[col as usize];
                let column = column_ref.borrow_mut();

                let cell_rc = Rc::clone(&column[row as usize]);
                drop(column);
                let mut parent_cell = cell_rc.borrow_mut();
                // let temp1 = (*sheets)[sheet_num as usize].borrow();
                // let sheet = Rc::clone(&temp1);
                // let temp2 = sheet.data[col as usize].borrow();
                // let parent_cell = Rc::clone(&temp2[row as usize]);
                // let mut parent_cell = parent_cell.borrow_mut();

                parent_cell.children.remove(cell);
                drop(parent_cell);  // is this needed? // Yes, to release the borrow before the next iteration
            },
            ParentType::Range(start, end) => 
            {
                let Addr{sheet:s1, row:r1, col:c1} = start;
                let Addr{sheet:s2, row:r2, col:c2} = end;
                if s1 != s2 
                {
                    return Err("Should not happen!!!".to_string());
                }
                for i in c1..=c2 
                {
                    for j in r1..=r2 
                    {
                        let sheet_ref = &(*sheets)[s1 as usize];
                        let sheet = sheet_ref.borrow();

                        let column_ref = &sheet.data[i as usize];
                        let column = column_ref.borrow_mut();

                        let cell_rc = Rc::clone(& column[j as usize]);
                        drop(column);
                        let mut parent_cell = cell_rc.borrow_mut();
                        parent_cell.children.remove(cell);
                        drop(parent_cell);  // is this needed? // Yes, to release the borrow before the next iteration
                    }
                }
            },
        }
    }
    Ok(())
} 


fn eval(expr: &Expr, sheets: &Vec<Rc<RefCell<Sheet>>>) -> Result<ValueType,String> 
{
    match expr 
    {
        Expr::Integer(n) => Ok(ValueType::IntegerValue(*n)),
        Expr::Float(n) => Ok(ValueType::FloatValue(*n)),
        Expr::Cell(addr) =>
        {
            let Addr { sheet:sheet_num, row, col } = addr;
            let sheet_num = *sheet_num;
            let col = *col;
            let row = *row;
            let sheet_ref =&(*sheets)[sheet_num as usize];
            let sheet = sheet_ref.borrow();
            let column_ref = &sheet.data[col as usize];
            let column = column_ref.borrow_mut();
            let cell_rc = Rc::clone(&column[row as usize]);
            drop(column);
            let parent_cell = cell_rc.borrow();
            Ok(parent_cell.value.clone())
        }
        Expr::MonoOp(fun, exp) =>
        {
            match fun 
            {
                MonoFunction::Sleep =>
                {
                    let sleep_val = eval(exp, sheets)?;
                    match sleep_val {
                        ValueType::IntegerValue(sec) => 
                            {
                                if sec < 0
                                {
                                    return Err("Negative sleep time".to_string());
                                }
                                sleep(sec as f64)
                            },
                        ValueType::FloatValue(sec) => 
                        {
                            if sec < 0.0
                            {
                                return Err("Negative sleep time".to_string());
                            }
                            sleep(sec)
                        },
                        _ => return Err("Invalid argument for sleep".to_string()),
                    }
                    Ok(sleep_val)
                }
            }
        }
        Expr::RangeOp{op,start, end} =>
        {
            match op 
            {
                RangeFunction::Min => 
                {
                    let sheet_index = start.sheet as usize;
                    let sheet = (*sheets)[sheet_index].borrow().clone();
                    let range = ((start.row, start.col), (end.row,end.col));
                    min_eval(&sheet.data, range)
                },
                RangeFunction::Max => 
                {
                    let sheet_index = start.sheet as usize;
                    let sheet = (*sheets)[sheet_index].borrow().clone();
                    let range = ((start.row, start.col), (end.row,end.col));
                    max_eval(&sheet.data, range)
                },
                RangeFunction::Sum => 
                {
                    let sheet_index = start.sheet as usize;
                    let sheet = (*sheets)[sheet_index].borrow().clone();
                    let range = ((start.row, start.col), (end.row,end.col));
                    sum_eval(&sheet.data, range)
                },
                RangeFunction::Avg => 
                {
                    let sheet_index = start.sheet as usize;
                    let sheet = (*sheets)[sheet_index].borrow().clone();
                    let range = ((start.row, start.col), (end.row,end.col));
                    avg_eval(&sheet.data, range)
                },
                RangeFunction::Stdev => 
                {
                    let sheet_index = start.sheet as usize;
                    let sheet = (*sheets)[sheet_index].borrow().clone();
                    let range = ((start.row, start.col), (end.row,end.col));
                    stdev_eval(&sheet.data, range)
                },
            }
        }
        Expr:: BinOp(exp1,func , exp2 ) =>
        {
            match func 
            {
                BinaryFunction:: Mul =>
                {
                    let left = eval(exp1, sheets)?;
                    let right = eval(exp2, sheets)?;
                    match (left,right) 
                    {
                        (ValueType::FloatValue(n), ValueType::FloatValue(m)) =>
                        {
                            Ok(ValueType::FloatValue(n*m))
                        }
                        (ValueType::IntegerValue(n), ValueType::FloatValue(m)) =>
                        {
                            Ok(ValueType::FloatValue((n as f64)*m))
                        }
                        (ValueType::FloatValue(n), ValueType::IntegerValue(m)) =>
                        {
                            Ok(ValueType::FloatValue(n*(m as f64)))
                        }
                        (ValueType::IntegerValue(n), ValueType::IntegerValue(m)) =>
                        {
                            Ok(ValueType::IntegerValue(n*m))
                        }
                        (_,_) =>
                        {
                            Err("String used in Multiplication".to_string())
                        }
                    }
                },
                BinaryFunction::Add =>
                {
                    let left = eval(exp1, sheets)?;
                    let right = eval(exp2, sheets)?;
                    match (left, right) {
                        (ValueType::FloatValue(n), ValueType::FloatValue(m)) =>
                        {
                            Ok(ValueType::FloatValue(n + m))
                        }
                        (ValueType::IntegerValue(n), ValueType::FloatValue(m)) =>
                        {
                            Ok(ValueType::FloatValue(n as f64 + m))
                        }
                        (ValueType::FloatValue(n), ValueType::IntegerValue(m)) =>
                        {
                            Ok(ValueType::FloatValue(n + m as f64))
                        }
                        (ValueType::IntegerValue(n), ValueType::IntegerValue(m)) =>
                        {
                            Ok(ValueType::IntegerValue(n + m))
                        }
                        (_, _) =>
                        {
                            Err("String used in Addition".to_string())
                        }
                    }
                },
                BinaryFunction::Sub =>
                {
                    let left = eval(exp1, sheets)?;
                    let right = eval(exp2, sheets)?;
                    match (left, right) {
                        (ValueType::FloatValue(n), ValueType::FloatValue(m)) =>
                        {
                            Ok(ValueType::FloatValue(n - m))
                        }
                        (ValueType::IntegerValue(n), ValueType::FloatValue(m)) =>
                        {
                            Ok(ValueType::FloatValue(n as f64 - m))
                        }
                        (ValueType::FloatValue(n), ValueType::IntegerValue(m)) =>
                        {
                            Ok(ValueType::FloatValue(n - m as f64))
                        }
                        (ValueType::IntegerValue(n), ValueType::IntegerValue(m)) =>
                        {
                            Ok(ValueType::IntegerValue(n - m))
                        }
                        (_, _) =>
                        {
                            Err("String used in Subtraction".to_string())
                        }
                    }
                },
                BinaryFunction::Div =>
                {
                    let left = eval(exp1, sheets)?;
                    let right = eval(exp2, sheets)?;
                    match (left, right) 
                    {
                        (ValueType::FloatValue(n), ValueType::FloatValue(m)) => 
                        {
                            if m == 0.0 
                            {
                                Err("Division by zero".to_string())
                            } 
                            else {
                                Ok(ValueType::FloatValue(n / m))
                            }
                        },
                        (ValueType::IntegerValue(n), ValueType::FloatValue(m)) => 
                        {
                            if m == 0.0 
                            {
                                Err("Division by zero".to_string())
                            } 
                            else {
                                Ok(ValueType::FloatValue(n as f64 / m))
                            }
                        },
                        (ValueType::FloatValue(n), ValueType::IntegerValue(m)) => 
                        {
                            if m == 0 
                            {
                                Err("Division by zero".to_string())
                            } 
                            else {
                                Ok(ValueType::FloatValue(n / m as f64))
                            }
                        },
                        (ValueType::IntegerValue(n), ValueType::IntegerValue(m)) => 
                        {
                            if m == 0 
                            {
                                Err("Division by zero".to_string())
                            } 
                            else {
                                Ok(ValueType::IntegerValue(n / m))
                            }
                        },
                        (_, _) =>
                        {
                            Err("String used in Division".to_string())
                        }
                    }
                }
            }
        }
    }
}

// this would be a recursive function just like eval of an ast
fn calculate(cell_rc:Rc<RefCell<Cell>>, sheets: &Vec<Rc<RefCell<Sheet>>>) -> Result<(),String>
{
    let temp: Rc<RefCell<Cell>> = Rc::clone(&cell_rc);
    let mut cell: std::cell::RefMut<'_, Cell> = temp.borrow_mut();
    let cell_func: &Option<CellFunc> = &cell.cell_func;
    match cell_func
    {
        Some(func) =>
        {   
            let expr = &func.expression;
            let temp = eval(expr, sheets);
            if let Err(err) = temp 
            {
                cell.valid = false;
                drop(cell);
                return Err(err);
            }
            else {
                let temp = temp.unwrap();
                cell.value = temp;
                cell.valid = true;
                drop(cell);
            }
            // cell.value = temp;
            Ok(())
        }
        None => 
        {
            let r1= cell.addr.row;
            let c1= cell.addr.col;
            drop(cell);
            Err(format!("No function associated to the cell at ({}, {})",r1, c1))
        }
    }  
}

fn update_parent_avls(cell:&Addr, sheets: &mut Vec<Rc<RefCell<Sheet>>>, dependencies: Vec<ParentType>) -> Result<(),String>
{
    for i in dependencies
    {
        match i 
        {
            ParentType::Single(addr) => 
            { 
                // let sheet = *(*sheets)[addr.sheet as usize].borrow();

                
                // let parent_cell = *sheet.data[addr.col as usize].borrow()[addr.row as usize].clone();
                // let mut parent_cell = *parent_cell.borrow_mut();
                let sheet_ref = &(*sheets)[addr.sheet as usize];
                let sheet = sheet_ref.borrow();

                let column_ref = &sheet.data[addr.col as usize];
                let column = column_ref.borrow_mut();

                let cell_rc = Rc::clone(&column[addr.row as usize]);
                drop(column);
                let mut parent_cell = cell_rc.borrow_mut();
                parent_cell.children.insert((cell).clone());
                drop(parent_cell); 
            },
            ParentType::Range(start, end) => 
            {
                let Addr{sheet:s1, row:r1, col:c1} = start;
                let Addr{sheet:s2, row:r2, col:c2} = end;
                if s1 != s2 
                {
                    return Err("Should not happen!!!".to_string());
                }
                for i in c1..=c2 
                {
                    for j in r1..=r2 
                    {
                        let sheet_ref = &(*sheets)[s1 as usize];
                        let sheet = sheet_ref.borrow();

                        let column_ref = &sheet.data[i as usize];
                        let column = column_ref.borrow_mut();

                        let cell_rc = Rc::clone(&column[j as usize]);
                        drop(column);
                        let mut parent_cell = cell_rc.borrow_mut();
                        parent_cell.children.insert((cell).clone());
                        drop(parent_cell);
                    }
                }
            },
        }
        
    }
    Ok(())
}


fn dfs(sheets: &Vec<Rc<RefCell<Sheet>>>,current_cell: &Addr, visited: &mut HashMap<Addr,bool>, rec_stack: &mut HashMap<Addr,bool>, stack: &mut Vec<Addr>) -> Result<(),String>     
{
    rec_stack.insert(current_cell.clone(), true); 

    let sheet_ref = &(*sheets)[current_cell.sheet as usize];
    let sheet = sheet_ref.borrow();

    let column_ref = &sheet.data[current_cell.col as usize];
    let column = column_ref.borrow_mut();

    let cell_rc = Rc::clone(&column[current_cell.row as usize]);
    drop(column);
    let curr_cell = cell_rc.borrow();

    let ordered_set = curr_cell.children.clone();
    for i in &ordered_set
    {
        if rec_stack.contains_key(i) 
        {
            return Err(format!("Cyclic dependency detected at cell ({}, {})", i.row+1, i.col+1));
        }
        else if visited.contains_key(i) 
        {
            continue;
        }
        else 
        {
            dfs(sheets, i, visited, rec_stack, stack)?;
        }
    }
    visited.insert(current_cell.clone(), true); 
    rec_stack.remove(current_cell);
    stack.push(current_cell.clone()); 
    Ok(())
}

fn topological_sort(sheets: &Vec<Rc<RefCell<Sheet>>>, addr:&Addr) -> Result<Vec<Addr>,String> 
{
    let mut visited: HashMap<Addr, bool> = HashMap::new();
    let mut rec_stack: HashMap<Addr, bool> = HashMap::new();
    let mut stack: Vec<Addr> = Vec::new();
    dfs(sheets, addr, &mut visited, &mut rec_stack, &mut stack)?;
    Ok(stack)
}


fn update_children(sheets: &Vec<Rc<RefCell<Sheet>>>, cell: &Addr) -> Result<(), String> 
{
    let ret = topological_sort(sheets, cell)?;
    // let negative_in_sleep = false;
    for i in ret.iter().rev()
    {
        let sheet_ref = &(*sheets)[i.sheet as usize];
        let sheet = sheet_ref.borrow();

        let column_ref = &sheet.data[i.col as usize];
        let column = column_ref.borrow_mut();

        let cell_rc = Rc::clone(& column[i.row as usize]);
        drop(column);
        let curr_cell = cell_rc.borrow();
        let checker = curr_cell.cell_func.is_some();
        drop(curr_cell);
        // match curr_cell.cell_func
        // {
        //     Some(_) => 
        //     {
        //         calculate(&mut curr_cell, &sheets)?
        //     },
        //     None => Ok(()),
        // }
        if checker
        {
            calculate(cell_rc, sheets)?;
        }
    }

    Ok(())
    
}

pub fn evaluate(sheets: &mut Vec<Rc<RefCell<Sheet>>>, cell: &Addr, old_func: &Option<CellFunc>) -> Result<(), String>   /////// OWNERSHIP NAHI LENI THI!!!!!!!!
{
    // println!("{}", Rc::clone(& (&sheets[0].borrow().data[cell.col as usize].borrow_mut()[cell.row as usize])).try_borrow_mut().is_ok());
    let cell_rc = {
        let sheet_ref = &(*sheets)[cell.sheet as usize];
        let sheet = sheet_ref.borrow();
        let column_ref = &sheet.data[cell.col as usize];
        let column = column_ref.borrow_mut();
        Rc::clone(&column[cell.row as usize])
        // drop(column);
    };
    // println!("{}", Rc::clone(& (&sheets[0].borrow().data[cell.col as usize].borrow_mut()[cell.row as usize])).try_borrow_mut().is_ok());
    let curr_cell = cell_rc.borrow();
    // let roww = curr_cell.addr.row.clone();
    // let coll = curr_cell.addr.col.clone();
    // println!("2{}", Rc::clone(& (&sheets[0].borrow().data[cell.col as usize].borrow_mut()[cell.row as usize])).try_borrow_mut().is_ok());
    let cell_funcc = curr_cell.cell_func.clone();
    drop(curr_cell);
    // println!("1{}", Rc::clone(& (&sheets[0].borrow().data[cell.col as usize].borrow_mut()[cell.row as usize])).try_borrow_mut().is_ok());

    let old_dependencies =  match old_func {
        Some(x) => x.expression.get_dependency_list(),
        None => vec![]
    };                                                                  ////// ISKO THODA DEKH LENA
    // let dependencies = curr_cell.cell_func.unwrap().clone().expression.get_dependency_list();       ////// ISKO THODA DEKH LENA
    // println!("{}", Rc::clone(& (&sheets[0].borrow().data[cell.col as usize].borrow_mut()[cell.row as usize])).try_borrow_mut().is_ok());
    let dependencies = match &cell_funcc {
        Some(func) => func.expression.get_dependency_list(),
        None => {
            vec![]
        }
    };
    // println!("{}", Rc::clone(& (&sheets[0].borrow().data[cell.col as usize].borrow_mut()[cell.row as usize])).try_borrow_mut().is_ok());
    
    remove_old_dependencies(cell, sheets, old_dependencies)?;
    // println!("{}", Rc::clone(& (&sheets[0].borrow().data[cell.col as usize].borrow_mut()[cell.row as usize])).try_borrow_mut().is_ok());

    update_parent_avls(cell, sheets, dependencies)?;
    let temp = update_children(sheets, cell);
    if let Err(strr) = temp 
    {
        // kya sleep me negative hai to restore karni hai values? //Yes
        if strr.contains("Cyclic dependency detected") || strr.contains("Negative sleep time") //NOTE: Isko theek karna hai
        {
            // we need to restore the values 
            // let func = curr_cell.cell_func.clone();
            let mut curr_cell = cell_rc.borrow_mut();
            curr_cell.cell_func = old_func.clone();
            drop(curr_cell);            ////////////////////////////////////////////////////
            evaluate(sheets,cell, &cell_funcc)?;
        }
        return Err(strr);
    }    
    Ok(())
}
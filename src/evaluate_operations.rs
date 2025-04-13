use std::fmt::Error;
// use std::ptr::with_exposed_provenance;
// use std::cmp;
use std::thread;
use std::time::Duration;
use crate::ast::{Addr, BinaryFunction, Expr, MonoFunction, ParentType, RangeFunction};
use crate::cell_operations::{Sheet,Cell,CellFunc,ValueType};
#[allow(unused_imports)]
use std::rc::{Rc, Weak};
#[allow(unused_imports)]
use std::cell::RefCell;
// use crate::cell_operations::CellFunc;
fn min_eval(data: &Vec<RefCell<Vec<Rc<RefCell<Cell>>>>>, range: ((u32,u32),(u32,u32))) -> Result<ValueType, String> 
{
    
    let cell1: (u32, u32) = range.0;
    let cell2: (u32, u32) = range.1;
    let mut mini = f64::MAX;
    let mut isfloat = false;
    for col in cell1.1..=cell2.1 
    {
        for row in cell1.0..=cell2.0 
        {
            
            let temp1: std::cell::Ref<'_, Vec<Rc<RefCell<Cell>>>> = data[col as usize].borrow();
            let temp2: Rc<RefCell<Cell>> = Rc::clone(&temp1[row as usize]);
            let temp: std::cell::Ref<'_, Cell> = temp2.borrow();
            if temp.valid == false 
            {
                return Err(format!("Invalid cell at ({}, {})", col, row));
            }
            else if let ValueType::IntegerValue(value) = (&temp).value 
            {
                if (value as f64) < mini 
                {
                    mini = value as f64;
                }
            }
            else if let ValueType::FloatValue(value) = (&temp).value 
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
    if isfloat == true 
    {
        return Ok(ValueType::FloatValue(mini));
    }
    else 
    {
        return Ok(ValueType::IntegerValue(mini as i32));
    }
}

fn max_eval(data: &Vec<RefCell<Vec<Rc<RefCell<Cell>>>>>, range: ((u32,u32),(u32,u32))) -> Result<ValueType, String> 
{
    
    let cell1: (u32, u32) = range.0;
    let cell2: (u32, u32) = range.1;
    let mut maxi = f64::MIN;
    let mut isfloat = false;
    for col in cell1.1..=cell2.1 
    {
        for row in cell1.0..=cell2.0 
        {
            
            let temp1: std::cell::Ref<'_, Vec<Rc<RefCell<Cell>>>> = data[col as usize].borrow();
            let temp2: Rc<RefCell<Cell>> = Rc::clone(&temp1[row as usize]);
            let temp: std::cell::Ref<'_, Cell> = temp2.borrow();
            if temp.valid == false 
            {
                return Err(format!("Invalid cell at ({}, {})", col, row));
            }
            else if let ValueType::IntegerValue(value) = (&temp).value 
            {
                if (value as f64) > maxi 
                {
                    maxi = value as f64;
                }
            }
            else if let ValueType::FloatValue(value) = (&temp).value 
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
    if isfloat == true 
    {
        return Ok(ValueType::FloatValue(maxi));
    }
    else 
    {
        return Ok(ValueType::IntegerValue(maxi as i32));
    }
}

fn sum_eval(data: &Vec<RefCell<Vec<Rc<RefCell<Cell>>>>>, range: ((u32,u32),(u32,u32))) -> Result<ValueType, String> 
{
    
    let cell1: (u32, u32) = range.0;
    let cell2: (u32, u32) = range.1;
    let mut summ = 0 as f64;
    let mut isfloat = false;
    for col in cell1.1..=cell2.1 
    {
        for row in cell1.0..=cell2.0 
        {
            
            let temp1: std::cell::Ref<'_, Vec<Rc<RefCell<Cell>>>> = data[col as usize].borrow();
            let temp2: Rc<RefCell<Cell>> = Rc::clone(&temp1[row as usize]);
            let temp: std::cell::Ref<'_, Cell> = temp2.borrow();
            if temp.valid == false 
            {
                return Err(format!("Invalid cell at ({}, {})", col, row));
            }
            else if let ValueType::IntegerValue(value) = (&temp).value 
            {
                summ += value as f64;
            }
            else if let ValueType::FloatValue(value) = (&temp).value 
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
    if isfloat == true 
    {
        return Ok(ValueType::FloatValue(summ));
    }
    else 
    {
        return Ok(ValueType::IntegerValue(summ as i32));
    }
}

fn avg_eval(data: &Vec<RefCell<Vec<Rc<RefCell<Cell>>>>>, range: ((u32,u32),(u32,u32))) -> Result<ValueType, String> 
{
    
    let cell1: (u32, u32) = range.0;
    let cell2: (u32, u32) = range.1;
    let mut summ = 0 as f64;
    let mut count = 0;
    for col in cell1.1..=cell2.1 
    {
        for row in cell1.0..=cell2.0 
        {
            
            let temp1: std::cell::Ref<'_, Vec<Rc<RefCell<Cell>>>> = data[col as usize].borrow();
            let temp2: Rc<RefCell<Cell>> = Rc::clone(&temp1[row as usize]);
            let temp: std::cell::Ref<'_, Cell> = temp2.borrow();
            if temp.valid == false 
            {
                return Err(format!("Invalid cell at ({}, {})", col, row));
            }
            else if let ValueType::IntegerValue(value) = (&temp).value 
            {
                summ += value as f64;
                count += 1;
            }
            else if let ValueType::FloatValue(value) = (&temp).value 
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

fn stdev_eval(data: &Vec<RefCell<Vec<Rc<RefCell<Cell>>>>>, range: ((u32,u32),(u32,u32))) -> Result<ValueType, String> 
{
    let cell1: (u32, u32) = range.0;
    let cell2: (u32, u32) = range.1;
    let mut summ = 0 as f64;
    let mut count = 0;
    for col in cell1.1..=cell2.1 
    {
        for row in cell1.0..=cell2.0 
        {
            
            let temp1: std::cell::Ref<'_, Vec<Rc<RefCell<Cell>>>> = data[col as usize].borrow();
            let temp2: Rc<RefCell<Cell>> = Rc::clone(&temp1[row as usize]);
            let temp: std::cell::Ref<'_, Cell> = temp2.borrow();
            if temp.valid == false 
            {
                return Err(format!("Invalid cell at ({}, {})", col, row));
            }
            else if let ValueType::IntegerValue(value) = (&temp).value 
            {
                summ += value as f64;
                count += 1;
            }
            else if let ValueType::FloatValue(value) = (&temp).value 
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
            let temp1: std::cell::Ref<'_, Vec<Rc<RefCell<Cell>>>> = data[col as usize].borrow();
            let temp2: Rc<RefCell<Cell>> = Rc::clone(&temp1[row as usize]);
            let temp: std::cell::Ref<'_, Cell> = temp2.borrow();
            if temp.valid == false 
            {
                return Err(format!("Invalid cell at ({}, {})", col, row));
            }
            else if let ValueType::IntegerValue(value) = (&temp).value 
            {
                sum_squared_diff += (value as f64 - mean).powi(2);
            }
            else if let ValueType::FloatValue(value) = (&temp).value 
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

fn sleep(seconds: i32) -> ()
{
    thread::sleep(Duration::from_secs(seconds as u64));
}

fn remove_old_dependencies(cell: &Addr,sheets: &mut Vec<RefCell<Rc<Sheet>>>, dependencies: Vec<ParentType>) -> ()
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
                let temp1 = (*sheets)[sheet_num as usize].borrow();
                let sheet = Rc::clone(&temp1);
                let temp2 = sheet.data[col as usize].borrow();
                let parent_cell = Rc::clone(&temp2[row as usize]);
                let mut parent_cell = parent_cell.borrow_mut();
                parent_cell.children.remove(&(cell));
            },
            ParentType::Range(start, end) => 
            {
                let Addr{sheet:s1, row:r1, col:c1} = start;
                let Addr{sheet:s2, row:r2, col:c2} = end;
                if s1 != s2 
                {
                    panic!("Should not happen!!!");
                }
                for i in c1..=c2 
                {
                    for j in r1..=r2 
                    {
                        let temp1 = (*sheets)[s1 as usize].borrow();
                        let sheet = Rc::clone(&temp1);
                        let temp2 = sheet.data[i as usize].borrow();
                        let parent_cell = Rc::clone(&temp2[j as usize]);
                        let mut parent_cell = parent_cell.borrow_mut();
                        parent_cell.children.remove(&(cell));
                    }
                }
            },
        }
    }

} 

fn eval(expr: &Expr, sheets: &Vec<RefCell<Rc<Sheet>>>) -> Result<ValueType,String> 
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
            let temp1 = (*sheets)[sheet_num as usize].borrow();
            let sheet = Rc::clone(&temp1);
            let temp2 = sheet.data[col as usize].borrow();
            let parent_cell = Rc::clone(&temp2[row as usize]);
            let parent_cell = parent_cell.borrow();
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
                        ValueType::IntegerValue(sec) => sleep(sec),
                        ValueType::FloatValue(sec) => sleep(sec as i32),
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
                    let sheet_index = 0;
                    let sheet = (*sheets)[sheet_index].borrow().clone();
                    let range = ((start.row, start.col), (end.row,end.col));
                    min_eval(&sheet.data, range)
                },
                RangeFunction::Max => 
                {
                    let sheet_index = 0;
                    let sheet = (*sheets)[sheet_index].borrow().clone();
                    let range = ((start.row, start.col), (end.row,end.col));
                    max_eval(&sheet.data, range)
                },
                RangeFunction::Sum => 
                {
                    let sheet_index = 0;
                    let sheet = (*sheets)[sheet_index].borrow().clone();
                    let range = ((start.row, start.col), (end.row,end.col));
                    sum_eval(&sheet.data, range)
                },
                RangeFunction::Avg => 
                {
                    let sheet_index = 0;
                    let sheet = (*sheets)[sheet_index].borrow().clone();
                    let range = ((start.row, start.col), (end.row,end.col));
                    avg_eval(&sheet.data, range)
                },
                RangeFunction::Stdev => 
                {
                    let sheet_index = 0;
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
fn calculate(cell:&mut Cell, sheets: &Vec<RefCell<Rc<Sheet>>>) -> Result<i32,String>
{
    let cell_func: &Option<CellFunc> = &cell.cell_func;
    match cell_func
    {
        Some(func) =>
        {   
            let expr = &func.expression;
            let temp = eval(expr, sheets).unwrap();
            cell.value = temp;
            return Ok(1);
        }
        None => 
        {
            return Err(format!("No function associated to the cell at ({}, {})",cell.row, cell.col));
        }
    }  
}

fn update_parent_avls(cell:&Addr, sheets: &mut Vec<RefCell<Rc<Sheet>>>, dependencies: Vec<ParentType>)
{
    for i in dependencies
    {
        match i 
        {
            ParentType::Single(addr) => 
            { 
                let Addr { sheet:sheet_num, row, col } = addr;
                let temp1 = (*sheets)[sheet_num as usize].borrow();
                let sheet = Rc::clone(&temp1);
                let temp2 = sheet.data[col as usize].borrow();
                let parent_cell = Rc::clone(&temp2[row as usize]);
                let mut parent_cell = parent_cell.borrow_mut();
                parent_cell.children.insert((cell).clone());
            },
            ParentType::Range(start, end) => 
            {
                let Addr{sheet:s1, row:r1, col:c1} = start;
                let Addr{sheet:s2, row:r2, col:c2} = end;
                if s1 != s2 
                {
                    panic!("Should not happen!!!");
                }
                for i in c1..=c2 
                {
                    for j in r1..=r2 
                    {
                        let temp1 = (*sheets)[s1 as usize].borrow();
                        let sheet = Rc::clone(&temp1);
                        let temp2 = sheet.data[i as usize].borrow();
                        let parent_cell = Rc::clone(&temp2[j as usize]);
                        let mut parent_cell = parent_cell.borrow_mut();
                        parent_cell.children.insert((cell).clone());
                    }
                }
            },
        }
        
    }
}


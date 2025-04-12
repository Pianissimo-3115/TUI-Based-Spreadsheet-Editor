// use std::cmp;
use std::thread;
use std::time::Duration;
use crate::ast::{Addr, ParentType};
use crate::cell_operations::{Cell,CellFunc,ValueType};
#[allow(unused_imports)]
use std::rc::{Rc, Weak};
#[allow(unused_imports)]
use std::cell::RefCell;
// use crate::cell_operations::CellFunc;
fn min_eval(data: &mut Vec<RefCell<Vec<Rc<RefCell<Cell>>>>>, range: ((u32,u32),(u32,u32))) -> Result<ValueType, String> 
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

fn max_eval(data: &mut Vec<RefCell<Vec<Rc<RefCell<Cell>>>>>, range: ((u32,u32),(u32,u32))) -> Result<ValueType, String> 
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

fn sum_eval(data: &mut Vec<RefCell<Vec<Rc<RefCell<Cell>>>>>, range: ((u32,u32),(u32,u32))) -> Result<ValueType, String> 
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

fn avg_eval(data: &mut Vec<RefCell<Vec<Rc<RefCell<Cell>>>>>, range: ((u32,u32),(u32,u32))) -> Result<ValueType, String> 
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

fn stdev_eval(data: &mut Vec<RefCell<Vec<Rc<RefCell<Cell>>>>>, range: ((u32,u32),(u32,u32))) -> Result<ValueType, String> 
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

fn remove_old_dependencies(sheets: &mut Vec<RefCell<Rc<Vec<RefCell<Vec<Rc<RefCell<Cell>>>>>>>>, dependencies: Vec<ParentType>) -> ()
{
    for i in dependencies
    {
        match i 
        {
            ParentType::Single(addr) => 
            {
                if let Addr::Local { row, col } = addr 
                {
                    
                } 
                else if let Addr::Global { sheet, row, col } = addr 
                {
                    
                } 
            },
            ParentType::Range(start, end) => 
            {
                
            },
        }
    }

} 





fn calculate(data: &Vec<&mut Vec<&mut Cell>>, cell: &mut Cell) -> Result<i32, String> 
{
    let cell_func = &cell.CellFunc;
    
}
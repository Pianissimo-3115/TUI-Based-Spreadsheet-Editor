use std::cmp;
use std::thread;
use std::time::Duration;
use crate::cell_operations::{Cell,Cell_func};
use crate::cell_operations::Cell_func;
fn min_eval(data: &Vec<&mut Vec<&mut Cell>>, func: &Cell_func) -> Result<i32, String> 
{
    let cell1 = &func.Cell1;
    let cell2 = &func.Cell2;
    let mut mini = i32::MAX;
    for col in {cell1.col_name..=cell2.col_name} {
        for row in {cell1.row_num..=cell2.row_num} {
            if(data[col as usize][row as usize].valid == false) {
                return Err(format!("Invalid cell at ({}, {})", col, row));
            }
            mini = cmp::min(mini, data[col as usize][row as usize].value);
            
        }
    }
    Ok(mini)
}

fn max_eval(data: &Vec<&mut Vec<&mut Cell>>, func: &Cell_func) -> Result<i32, String> 
{
    let cell1 = &func.Cell1;
    let cell2 = &func.Cell2;
    let mut maxi = i32::MIN;
    for col in {cell1.col_name..=cell2.col_name} {
        for row in {cell1.row_num..=cell2.row_num} {
            if(data[col as usize][row as usize].valid == false) {
                return Err(format!("Invalid cell at ({}, {})", col, row));
            }
            maxi = cmp::max(maxi, data[col as usize][row as usize].value);
            
        }
    }
    Ok(maxi)
}

fn sum_eval(data: &Vec<&mut Vec<&mut Cell>>, func: &Cell_func) -> Result<i32, String> 
{
    let cell1 = &func.Cell1;
    let cell2 = &func.Cell2;
    let mut sum = 0;
    for col in {cell1.col_name..=cell2.col_name} {
        for row in {cell1.row_num..=cell2.row_num} {
            if(data[col as usize][row as usize].valid == false) {
                return Err(format!("Invalid cell at ({}, {})", col, row));
            }
            sum += data[col as usize][row as usize].value;
            
        }
    }
    Ok(sum)
}

fn avg_eval(data: &Vec<&mut Vec<&mut Cell>>, func: &Cell_func) -> Result<f64, String> 
{
    let cell1 = &func.cell1;
    let cell2 = &func.cell2;
    let mut sum = 0.0;
    let mut count = 0.0;

    for col in cell1.col_name..=cell2.col_name {
        for row in cell1.row_num..=cell2.row_num {
            let cell = &data[col as usize][row as usize];
            if !cell.valid {
                return Err(format!("Invalid cell at ({}, {})", col, row));
            }
            sum += cell.value;
            count += 1.0;
        }
    }

    if count == 0.0 {
        return Err("No valid cells in range".to_string());
    }

    let avg = sum / count;
    Ok(avg)
}

fn stdev_eval(data: &Vec<&mut Vec<&mut Cell>>, func: &Cell_func) -> Result<f64, String> 
{
    let cell1 = &func.cell1;
    let cell2 = &func.cell2;
    let mut sum: f64 = 0.0;
    let mut count: f64 = 0.0;


    for col in cell1.col_name..=cell2.col_name {
        for row in cell1.row_num..=cell2.row_num {
            let cell = &data[col as usize][row as usize];
            if !cell.valid {
                return Err(format!("Invalid cell at ({}, {})", col, row));
            }
            sum += cell.value;
            count += 1.0;
        }
    }

    if count == 0.0 {
        return Err("No valid cells in range".to_string());
    }

    let mean = sum / count;
    let mut sum_squared_diff = 0.0;

    for col in cell1.col_name..=cell2.col_name {
        for row in cell1.row_num..=cell2.row_num {
            let cell = &data[col as usize][row as usize];
            if !cell.valid {
                return Err(format!("Invalid cell at ({}, {})", col, row));
            }
            let diff = cell.value - mean;
            sum_squared_diff += diff * diff;
        }
    }

    let stdev = (sum_squared_diff / count).sqrt();
    Ok(stdev)
}

fn sleep(seconds: i32) -> ()
{
    thread::sleep(Duration::from_secs(seconds as u64));
}

fn remove_old_dependencies(data: &Vec<&mut Vec<&mut Cell>>, func: &Cell_func) -> ()
{
    
} 





fn calculate(data: &Vec<&mut Vec<&mut Cell>>, cell: &mut Cell) -> Result<i32, String> 
{
    let cell_func = &cell.cell_func;
    
}
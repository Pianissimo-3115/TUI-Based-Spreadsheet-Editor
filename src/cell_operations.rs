///////////////// ONLY COMPLETED TOKENS FOR NUMERAL CELL FUNCS; STRING CELL FUNCS NOT DONE
///////////////// HAVE TO MAKE LEXER BY OWN 😢  FOR COMPLEX FUNCTIONS AS PROPOSED       // ban gaya yay
use crate::ast::{Expr, Addr};
use std::cell::RefCell;
use std::collections::BTreeSet;
#[allow(unused_imports)]
use std::rc::{Rc, Weak};


#[derive(Debug, Clone)]
pub enum ValueType 
{
    IntegerValue(i32),
    FloatValue(f64),
    String(String),
}
impl std::fmt::Display for ValueType 
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        match self 
        {
            ValueType::IntegerValue(n) => write!(f, "{}", n),
            ValueType::FloatValue(n) => write!(f, "{}", n),
            ValueType::String(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Clone)]
pub struct CellFunc 
{
    // pub dependency_list: Vec<Weak<RefCell<Cell>>>,
    pub expression: Expr,
    pub destination: Weak<RefCell<Cell>>, // USE OF Weak<T> is DOUBTFUL // @viraaz11: kyu chahiye      // @Pianissimo3115: HATA SKTE AS WELL
    pub value: ValueType,           // HATA SKTE
}

impl std::fmt::Display for CellFunc 
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        write!(f, "{:?}", self.expression)
    }
}

impl std::fmt::Debug for CellFunc 
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        writeln!(f, "CellFunc {{")?;

        writeln!(f, "\texpression: {:?},", self.expression)?;
        writeln!(f, "\tvalue: {:?},", self.value)?;

        write!(f, "\tdestination: ")?;
        if let Some(dest_rc) = self.destination.upgrade() 
        {
            let dest = dest_rc.borrow();
            writeln!(f, "Cell({}, {})", dest.row, dest.col)?;
        } 
        else 
        {
            writeln!(f, "None (dropped)")?;
        }

        writeln!(f, "}}")
    }
}
impl CellFunc
{
    pub fn new(expression: Expr, destination: Weak<RefCell<Cell>>) -> Self 
    {
        CellFunc 
        {
            expression,
            destination,
            value: ValueType::IntegerValue(0), // Default value; will be updated later
        }
    }
}

#[derive(Debug, Clone)]
pub struct Cell 
{
    pub row: u32,
    pub col: u32,
    pub addr: Addr,
    pub value: ValueType,
    pub cell_func: Option<CellFunc>,
    pub children: BTreeSet<Addr>, // USE OF Weak<T> is DOUBTFUL
    pub valid: bool,
}

impl Cell 
{
    pub fn new(row: u32, col: u32, addr: Addr) -> Self 
    {
        Cell 
        {
            row,
            col,
            addr,
            value: ValueType::IntegerValue(0),
            cell_func: None,
            valid: true,
            children: BTreeSet::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Sheet
{
    pub data: Vec<RefCell<Vec<Rc<RefCell<Cell>>>>>,
    pub rows: u32,
    pub columns: u32,
    pub sheet_idx: u32,
    pub sheet_name: String
}
impl Sheet
{
    pub fn new(sheet_idx: u32, sheet_name: String, col: u32) -> Self 
    {
        Sheet 
        {
            data: vec![RefCell::new(vec![]); col as usize], 
            rows: 0,
            columns: col,
            sheet_idx,
            sheet_name
        }
    }
    pub fn resize(&mut self, row_num: u32, col_num:u32)         /////////////////////////////////////////// PENDING
    {
        if(self.data.len() < row_num as usize) 
        {
            for _ in self.data.len()..row_num as usize 
            {
                let temp: Vec<Rc<RefCell<Cell>>> = vec![];
                self.data.push(RefCell::new(temp));
            }
            if col_num < self.columns 
            {
                // to do by arjun 
            }
        } 
    }
}
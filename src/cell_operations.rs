///////////////// ONLY COMPLETED TOKENS FOR NUMERAL CELL FUNCS; STRING CELL FUNCS NOT DONE
///////////////// HAVE TO MAKE LEXER BY OWN 😢  FOR COMPLEX FUNCTIONS AS PROPOSED
use ast::Expr;
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::rc::{Rc, Weak};

#[derive(Debug, Clone)]
pub enum ValueType 
{
    Number(i32),
    String(String),
}
impl fmt::Display for ValueType 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        match self 
        {
            ValueType::Number(n) => write!(f, "{}", n),
            ValueType::String(s) => write!(f, "{}", s),
        }
    }
}

pub struct CellFunc 
{
    // pub dependency_list: Vec<Weak<RefCell<Cell>>>,
    pub expression: Expr,
    pub destination: Weak<RefCell<Cell>>, // USE OF Weak<T> is DOUBTFUL
    pub value: ValueType,
}

impl fmt::Display for CellFunc 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        write!(f, "{:?}", self.expression)
    }
}

impl fmt::Debug for CellFunc 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
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

        writeln!(f, "\tdependency_list: [")?;
        for dep in &self.dependency_list 
        {
            write!(f, "\t\t")?;
            if let Some(dep_rc) = dep.upgrade() 
            {
                let dep = dep_rc.borrow();
                writeln!(f, "Cell({}, {}),", dep.row, dep.column)?;
            } 
            else 
            {
                writeln!(f, "None (dropped),")?;
            }
        }
        writeln!(f, "\t]")?;

        writeln!(f, "}}")
    }
}

pub struct Cell 
{
    pub row: u32,
    pub col: u32,
    pub value: ValueType,
    pub cell_func: CellFunc,
    pub children: BTreeSet<Weak<RefCell<Cell>>>, // USE OF Weak<T> is DOUBTFUL
}

impl Cell 
{
    pub fn new(row: u32, col: u32, value: ValueType, cell_func: CellFunc) -> Self 
    {
        Cell 
        {
            row,
            col,
            value,
            cell_func,
            children: BTreeSet::new(),
        }
    }
}

#![allow(unused)]

pub enum ParserError{
    NumberTooLargeAt(String, u32, u32),
}


// pub enum CellRef {
//     /// Reference within the current sheet
//     Local {
//         row: usize,
//         col: usize,
//     },
//     /// Reference to a different sheet
//     Global {
//         sheet: usize,
//         row: usize,
//         col: usize,
//     },
// }

#[derive(Debug)]
pub enum Command {
    DisplayCmd(DisplayCommand),  //Note: IS Box<DisplayCommand> better? Display Command is a finite data type, but expr was not.
    AssignCmd(Addr, Box<Expr>),
    Quit,
}

#[derive(Debug)]
pub enum DisplayCommand {
    EnableOut,
    DisableOut,
    ScrollTo(Addr),
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i32),
    Cell(Addr),
    MonoOp(MonoFunction, Box<Expr>),
    RangeOp{op: RangeFunction, start: Addr, end: Addr}, //Note: Should addr be under Box<>?
    BinOp(Box<Expr>, BinaryFunction, Box<Expr>),
}
pub enum ParentType {
    Single(Addr),
    Range(Addr, Addr),
}
impl std::fmt::Debug for ParentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParentType:: Single(addr) => write!(f, "Single({:?})", addr),
            ParentType::Range(start, end) => write!(f, "Range({:?}, {:?})", start, end),
        }
    }
}
impl Expr
{
    // pub enum ParentType {
    //     Single(Addr),
    //     Range(Addr, Addr),
    // }


    pub fn get_dependency_list (&self) -> Vec<ParentType> {
        match self {
            Expr::Number(_) => vec![],
            Expr::Cell(addr) => vec![ParentType::Single(addr.clone())],
            Expr::MonoOp(_, expr) => expr.get_dependency_list(),
            Expr::RangeOp{start, end, ..} => vec![ParentType::Range(start.clone(), end.clone())],
            Expr::BinOp(left, _, right) => {
                let mut deps = left.get_dependency_list();
                deps.append(&mut right.get_dependency_list());
                deps
            }
        }
    }

}

#[derive(Debug, Clone)]
pub enum Addr {
    Local { row: u32, col: u32}, //NOTE: check all different int datatypes used a at different places and verify.
    Global {sheet: String, row: u32, col: u32} //NOTE: sheet should be String or str or &str or something else?!?.
}

// #[derive(Debug)]
// pub enum Range {

// }

#[derive(Debug, Clone)]
pub enum MonoFunction {
    Sleep,
}

#[derive(Debug, Clone)]
pub enum RangeFunction {
    Sum,
    Avg,
    Max,
    Min,
    Stdev,
}

#[derive(Debug, Clone)]
pub enum BinaryFunction {
    Mul,
    Div,
    Add,
    Sub,
}
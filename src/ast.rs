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

#[derive(Debug)]
pub enum Expr {
    Number(i32),
    Cell(Addr),
    MonoOp(Monofunction, Box<Expr>),
    RangeOp(op: RangeFunction, start: Addr, end: Addr) //Note: Should addr be under Box<>?
    BinOp(Box<Expr>, BinaryFunction, Box<Expr>),
}

#[derive(Debug)]
pub enum Addr {
    Local { row: u32, col: u32}, //NOTE: check all different int datatypes used a at different places and verify.
    Global {sheet: String, row: u32, col: u32} //NOTE: sheet should be String or str or &str or something else?!?.
}

// #[derive(Debug)]
// pub enum Range {

// }

#[derive(Debug)]
pub enum MonoFunction {
    Sleep,
}

#[derive(Debug)]
pub enum RangeFunction {
    Sum,
    Avg,
    Max,
    Min,
    Stdev,
}

#[derive(Debug)]
pub enum BinaryFunction {
    Mul,
    Div,
    Add,
    Sub,
}
// fn main() {


// }

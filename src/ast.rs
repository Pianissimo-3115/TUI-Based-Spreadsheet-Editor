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
    Op(Box<Expr>, BinaryOp, Box<Expr>),
}

#[derive(Debug)]
pub enum Addr {
    Local { row: u32, col: u32}, //NOTE: check all different int datatypes used a at different places and verify.
    Global {sheet: String, row: u32, col: u32} //NOTE: sheet should be String or str or &str or something else?!?.
}

#[derive(Debug)]
pub enum BinaryOp {
    Mul,
    Div,
    Add,
    Sub,
}
// fn main() {


// }

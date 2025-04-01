///////////////// ONLY COMPLETED TOKENS FOR NUMERAL CELL FUNCS; STRING CELL FUNCS NOT DONE
///////////////// HAVE TO MAKE LEXER BY OWN 😢  FOR COMPLEX FUNCTIONS AS PROPOSED



use std::cell::RefCell;
use std::rc::Rc;
use data_structures::avl_tree::AVL;

enum valueType {
    intType(i32),
    floatType(f32),
    stringType(String),
    boolType(bool),
}

#[derive(Debug, Clone)]
pub struct Cell 
{
    pub col_name: i32,
    pub row_name: i32,
    pub value: Option<RefCell<valueType>>,
    pub valid: i32,
    pub func: Option<Rc<RefCell<CellFunc>>>,
    pub children: Option<Rc<RefCell<AVL>>>,
}



// DISABLE_OUT, ENABLE_OUT, SCROLL, etc are not CELL_FUNC
enum CellFunc
{
    FIX(Expression),
    SLEEP(Expression),
    MIN(Expression, Expression),
    RANGE_MIN(i32, i32, i32, i32),
    MAX(Expression, Expression),
    RANGE_MAX(i32, i32),
    SUM(i32, i32, i32, i32),
    AVG(i32, i32, i32, i32)
}

enum Expression
{
    ADD(Expression, Term),
    SUB(Expression, Term),
    TERM(Term)
}

enum Term
{
    MUL(Term, Factor),
    DIV(Term, Factor),
    FACTOR(Factor)
}

enum Factor             /////////////////////////////////////////////////////// COULD IMPLEMENT EXPONENT EASILY
{
    PARENTHESIZE()
}

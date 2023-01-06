#[derive(Debug)]
pub enum Expr<'a> {
    Add(&'a Expr<'a>, &'a Expr<'a>),
    Integer(&'a str),
}

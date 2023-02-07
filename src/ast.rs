#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryOp {
    Minus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnaryExpr {
    pub op: UnaryOp,
    pub expr: Expression,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    CmpL,
    CmpR,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryExpr {
    pub op: BinaryOp,
    pub lhs: Expression,
    pub rhs: Expression,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Literal(String),
    Variable(String),
    Unary(Box<UnaryExpr>),
    Binary(Box<BinaryExpr>),
    Call(String, Vec<Expression>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct If {
    pub condition: Expression,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct While {
    pub condition: Expression,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Let {
    pub variable: String,
    pub value: Expression,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Let(Box<Let>),
    If(Box<If>),
    While(Box<While>),
    Print(Expression),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub namespace: String,
    pub name: String,
    pub body: Vec<Statement>,
}

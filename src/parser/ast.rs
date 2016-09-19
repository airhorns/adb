
#[derive(Debug)]
pub enum Statement {
    Select(SelectStatement),
    Insert(InsertStatement)
}

#[derive(Debug)]
pub struct SelectStatement {
    pub selections: Vec<SelectItem>,
    pub from_relation: Vec<Relation>,
    pub where_condition: Option<Box<Expression>>
}

#[derive(Debug)]
pub struct InsertStatement {
    pub into_relation: Relation
}

#[derive(Debug)]
pub enum SelectItem {
    Expression(Expression),
    SelectAll(Option<QualifiedName>)
}

#[derive(Debug)]
pub enum Relation {
    Named(QualifiedName)
}

#[derive(Debug)]
pub enum Expression {
    NullLiteral,
    NumberLiteral(String),
    BooleanLiteral(String),
    QualifiedNameReference(QualifiedName),
    ParenthesizedExpression(Box<Expression>),
    BooleanExpression {
        left: Box<Expression>,
        right: Box<Expression>,
        operator: ComparisonOperator
    },
}

pub type QualifiedName = Vec<Identifier>;
pub type Identifier = String;

#[derive(Debug)]
pub enum ComparisonOperator {
    Equals,
    NotEquals,
    GreaterThan,
    GreaterThanEquals,
    LessThan,
    LessThanEquals
}

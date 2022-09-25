use std::pin::Pin;

use crate::{scanner::token::{Primitive, Token}, error_reporting::interp_err::InterpException};

use super::ast_traits::Accept;

#[derive(Clone, Debug)]
pub struct Expr {
    pub left: Box<Pin<Option<Expr>>>,
    pub right: Box<Pin<Option<Expr>>>,
    pub operator: Token,
}

#[derive(Clone, Debug)]
pub struct Ternary {
    pub condition: Box<ExprPossibilities>,
    pub false_cond: Option<Box<ExprPossibilities>>,
    pub true_cond: Option<Box<ExprPossibilities>>,
}

#[derive(Clone, Debug)]
pub struct Binary {
    pub left: Box<ExprPossibilities>,
    pub right: Box<ExprPossibilities>,
    pub operator: Token,
}

#[derive(Clone, Debug)]
pub struct Grouping {
    pub expr: Box<ExprPossibilities>,
}

#[derive(Clone, Debug)]
pub struct Literal {
    pub literal: Primitive,
}

#[derive(Clone, Debug)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<ExprPossibilities>,
}

#[derive(Clone, Debug)]
pub enum ExprPossibilities {
    Expr(Expr),
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Ternary(Ternary),
    Unary(Unary),
}

impl Accept<Option<String>> for ExprPossibilities {}

impl Accept<Result<Primitive, InterpException>> for ExprPossibilities {}

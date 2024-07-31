use std::fmt::Debug;

use crate::token::Token;

pub enum Children {
    None,
    One(Box<AST>),
    Two(Box<AST>, Box<AST>),
}
pub struct AST {
    pub expression: Token,
    pub children: Children,
}

impl Debug for AST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.children {
            Children::None => todo!(),
            Children::One(child) => write!(f, "({:?} {:?})", self.expression, child),
            Children::Two(left, right) => {
                write!(f, "({:?} {:?} {:?})", left, self.expression, right)
            }
        }
    }
}

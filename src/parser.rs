use crate::ast::Children;
use crate::ast::AST;
use crate::token::Token;
use crate::token::Token::*;

pub fn construct<'a>(stream: impl Iterator<Item = Token>) -> AST {
    let stream = stream.peekable();

    todo!()
}

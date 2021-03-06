use std::marker::PhantomData;
pub mod postfix;
pub mod infix;
pub mod prefix;
mod internal_types;

mod symbol;
use symbol::Symbol;
mod operator;
use operator::Operator;
pub type VariableMap = std::collections::HashMap<String,isize>;

#[derive(Debug,PartialEq)]
pub enum ExpressionError{
    ParseError,
    ToManyArguments,
    ToFewArgements,
}
use ExpressionError::*;
pub type Postfix<'a> = Expression<'a,internal_types::Postfix>;
pub type Prefix<'a> = Expression<'a,internal_types::Prefix>;
pub type Infix<'a> = Expression<'a,internal_types::Infix>;

#[derive(Clone)]
pub struct Expression<'a,T:internal_types::ExpressionType>{
    expression:Vec<Symbol<'a>>,
    _phantom:Option<PhantomData<T>>,
}
impl <'a,T:internal_types::ExpressionType> Expression<'a,T>{
    pub fn new(expression:&'a str)->Self{
        let expression = expression
            .trim()
            .split(' ')
            .map(|term| term.trim())
            .filter(|term| !term.is_empty())
            .map(|term| Symbol::parse(term))
            .collect();
        Self{expression,_phantom:None}
    }
    fn new_raw(expression:Vec<Symbol<'a>>)->Self{
        Self{expression,_phantom:None}
    }
}




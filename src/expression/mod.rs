use std::marker::PhantomData;
pub mod postfix;
pub mod infix;
mod internal_types;

pub type VariableMap = std::collections::HashMap<String,isize>;

#[derive(Debug,PartialEq)]
pub enum ExpressionError{
    ParseError,
    ToManyArguments,
    ToFewArgements,
}
use ExpressionError::*;
pub type Postfix<'a> = Expression<'a,internal_types::Postfix>;

pub struct Expression<'a,T:internal_types::ExpressionType>{
    expression:&'a str,
    _phantom:Option<PhantomData<T>>,
}
impl <'a,T:internal_types::ExpressionType> Expression<'a,T>{
    pub fn new(expression:&'a str)->Self{
        let expression = expression.trim();
        Self{expression,_phantom:None}
    }
}




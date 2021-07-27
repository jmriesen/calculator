use super::ExpressionError;
pub enum Symbol<'a>{
    Operator(super::Operator),
    Literal(isize),
    Variable(&'a str),
}
use Symbol::*;
impl<'a> Symbol<'a>{
    pub fn parse(symbol:&'a str)->Result<Self,ExpressionError>{
        use std::str::FromStr;
        if let Some(op) = super::Operator::new(symbol){
            Ok(Operator(op))
        }else if let Ok(numb) = isize::from_str(symbol) {
            Ok(Literal(numb))
        }else{
            Ok(Variable(symbol))
        }
    }
}


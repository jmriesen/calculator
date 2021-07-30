#[derive(PartialEq,Clone,Debug)]
pub enum Symbol<'a>{
    Operator(super::Operator),
    Literal(isize),
    Variable(&'a str),
    Parenthesis{
        opening:bool
    }
}
use Symbol::*;
impl<'a> Symbol<'a>{
    pub fn parse(symbol:&'a str)->Self{
        use std::str::FromStr;
        if let Some(op) = super::Operator::new(symbol){
            Operator(op)
        }else if let Ok(numb) = isize::from_str(symbol) {
            Literal(numb)
        }else if symbol == "(" {
            Parenthesis{opening:true}
        }else if symbol == ")" {
            Parenthesis{opening:false}
        }else{
            Variable(symbol)
        }
    }
}


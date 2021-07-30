#[derive(PartialEq,Clone,Debug)]
pub enum Operator{
    Add,
    Sub,
    Mul,
    Div
}

use Operator::*;
impl Operator{
    pub fn new(opp:&str)->Option<Self>{
        match opp {
            "+" =>Some(Add),
            "-" =>Some(Sub),
            "*" =>Some(Mul),
            "/" =>Some(Div),
            _ => None,
        }
    }
    pub fn priority(&self)->usize{
        match self{
            Add | Sub => 1,
            Mul | Div => 2,
        }
    }
    pub fn compute(&self,first:isize, second:isize)->isize{
        match self{
            Add => first + second,
            Sub => first - second,
            Mul => first * second,
            Div => first / second,
        }
    }
}

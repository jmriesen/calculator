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
    pub fn to_string(&self)->&'static str{
        match self{
            Add => "+",
            Sub => "-",
            Mul => "*",
            Div => "/",
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

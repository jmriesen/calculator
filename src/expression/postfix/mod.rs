
#[cfg(test)]
mod tests;
use super::*;


enum Atomics{
    Operator(fn(isize,isize) -> isize,),
    Operand(isize),
}
use Atomics::*;

impl <'a> Postfix<'a>{
    pub fn evaluate(&'a self,variables:&VariableMap)->Result<isize,ExpressionError>{
        use std::str::FromStr;
        let mut stack = vec![];
        let expression = self.expression.split(' ');
        for term in expression {
            let term = term.trim();
            if !term.is_empty(){
                let atom = match term {
                    "+" => Operator(|first, second| second + first),
                    "-" => Operator(|first, second| second - first),
                    "*" => Operator(|first, second| second * first),
                    "/" => Operator(|first, second| second / first),
                    _   =>Operand({
                        let numb = isize::from_str(term);
                        if let Ok(numb) = numb{
                            numb
                        }else{
                            *variables.get(term).ok_or(ParseError)?
                        }
                    }),
                };
                match atom{
                    Operator(oper) => {
                        let first  = stack.pop().ok_or(ToFewArgements)?;
                        let second = stack.pop().ok_or(ToFewArgements)?;
                        stack.push(oper(first,second));
                    }
                    Operand(numb) => stack.push(numb)
                }
            }
        }
        match stack.len(){
            0 => Err(ParseError),
            1 => Ok(stack.pop().unwrap()),
            _ => Err(ToManyArguments),
        }
    }
}

//TODO I am wondering if I should test this function directly
//Currently it is only being used as a way of evaluating infix
/*
impl <'a> From<&Infix<'a>> for Postfix<'a> {
    fn from(item: &Infix<'a>) -> Self {
        Postfix::new("6")
    }
}
*/

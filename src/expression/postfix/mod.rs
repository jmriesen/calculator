
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
        for term in &self.expression {
                let atom = match *term {
                    "+" => Operator(|first, second| second + first),
                    "-" => Operator(|first, second| second - first),
                    "*" => Operator(|first, second| second * first),
                    "/" => Operator(|first, second| second / first),
                    _   =>Operand({
                        let numb = isize::from_str(term);
                        if let Ok(numb) = numb{
                            numb
                        }else{
                            *variables.get(*term).ok_or(ParseError)?
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
        match stack.len(){
            0 => Err(ParseError),
            1 => Ok(stack.pop().unwrap()),
            _ => Err(ToManyArguments),
        }
    }
}

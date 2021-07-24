
#[cfg(test)]
mod tests;

struct Postfix<'a>{
    expression:&'a str,
}

#[derive(Debug,PartialEq)]
enum ExpressionError{
    ParseError,
    ToManyArguments,
    ToFewArgements,
}
use ExpressionError::*;

type VariableMap = std::collections::HashMap<String,isize>;

enum Atomics{
    Operator(fn(isize,isize) -> isize,),
    Operand(isize),
}
use Atomics::*;

impl <'a>Postfix<'a>{
    fn new(expression:&'a str)->Self{
        let expression = expression.trim();
        Postfix{expression}
    }
    fn evaluate(&'a self,variables:&VariableMap)->Result<isize,ExpressionError>{
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


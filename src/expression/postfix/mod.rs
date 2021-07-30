
#[cfg(test)]
mod tests;
use super::*;

use Symbol::*;

impl <'a> Postfix<'a>{
    pub fn evaluate(&'a self,variables:&VariableMap)->Result<isize,ExpressionError>{
        let mut stack = vec![];
        for symbol in &self.expression {

            let partial_result = match symbol {
                Literal(numb) => *numb,
                Variable(var) => *variables.get(*var).ok_or(ParseError)?,
                Operator(op) => {
                    let second  = stack.pop().ok_or(ToFewArgements)?;
                    let first = stack.pop().ok_or(ToFewArgements)?;
                    op.compute(first,second)
                },
                Parenthesis{opening:_} =>{Err(ParseError)?}
            };
            stack.push(partial_result);
        }
        match stack.len(){
            0 => Err(ParseError),
            1 => Ok(stack.pop().unwrap()),
            _ => Err(ToManyArguments),
        }
    }
}

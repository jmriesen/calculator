use super::*;

#[cfg(test)]
mod tests;

impl <'a> Infix<'a>{
    pub fn evaluate(&'a self,variables:&VariableMap)->Result<isize,ExpressionError>{
        Postfix::from(self.clone()).evaluate(variables)
    }
}

//TODO consider parsing earlier since all espresstions need to be parsed. 
impl<'a> From<Infix<'a>> for Postfix<'a>{
    fn from(prefix:Infix<'a>)->Postfix{
        let mut opperators : Vec<Symbol<'a>> = vec![];
        let mut new_expresstion = vec![];

        for symbol in prefix.expression {
            use Symbol::*;
            match symbol{
                Operator(opp)=>{
                    while let Some(symbol) = opperators.last() {
                        match symbol{
                            Operator(opperator) if opperator.priority()>=opp.priority()=>{
                                let last = opperators.pop().unwrap();
                                new_expresstion.push(last);
                            }
                            Operator(_) | Parenthesis{opening:true} => {break;},
                            _ => unreachable!(),
                        }
                    }
                    opperators.push(Operator(opp));
               },
                Parenthesis{opening:true} => opperators.push(symbol),
                Parenthesis{opening:false} =>{
                    while let Some(opperator) = opperators.pop(){
                        match opperator{
                            Operator(_) => new_expresstion.push(opperator),
                            Parenthesis{opening:true} => {},
                            _ => unreachable!(),
                        }
                    }
                },
                Literal(_) | Variable(_)=>{
                    new_expresstion.push(symbol);
                },
            }
        }

        opperators.reverse();
        new_expresstion.append(&mut opperators);
        Postfix::new_raw(
            new_expresstion
        )
    }
}

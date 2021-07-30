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
        let mut opperators : Vec<Operator> = vec![];
        let mut new_expresstion = vec![];

        for symbol in prefix.expression {
            use Symbol::*;
            match symbol{
                Operator(opp)=>{
                    while let Some(opperator) = opperators.last() {
                        if opperator.priority()>=opp.priority(){
                            let last = opperators.pop().unwrap();
                            new_expresstion.push(Operator(last));
                        }else{
                            break;
                        }
                    }
                    opperators.push(opp);
               },
                Parenthesis =>{/* TODO complete implementation.*/},
                Literal(_) | Variable(_)=>{
                    new_expresstion.push(symbol);
                },
            }
        }

        opperators.reverse();
        new_expresstion.append(
            &mut opperators.into_iter()
                .map(|op| Symbol::Operator(op))
                .collect()
        );
        println!("final:{:?}",new_expresstion);
        Postfix::new_raw(
            new_expresstion
        )
    }
}

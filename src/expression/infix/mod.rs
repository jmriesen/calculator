use super::*;

#[cfg(test)]
mod tests;

impl <'a> Infix<'a>{
    pub fn evaluate(&'a self,variables:&VariableMap)->Result<isize,ExpressionError>{
        Postfix::from(self.clone()).evaluate(variables)
    }
}

impl<'a> From<Infix<'a>> for Postfix<'a>{
    fn from(prefix:Infix)->Postfix{
        let mut opperators :Vec<Operator> = vec![];
        let mut new_expresstion  = vec![];

        for term in &prefix.expression {
            if let Some(opp) = Operator::new(term){
                while let Some(opperator) = opperators.last() {
                    if opperator.priority()>=opp.priority(){
                        let last = opperators.pop().unwrap();
                        new_expresstion.push(last.to_string());
                    }else{
                        break;
                    }
                }
                opperators.push(opp);
            }else{
                new_expresstion.push(*term);
            }
        }

        opperators.reverse();
        new_expresstion.append(
            &mut opperators.iter()
                .map(|op| op.to_string())
                .collect()
        );
        Postfix::new_raw(
            new_expresstion
        )
    }
}

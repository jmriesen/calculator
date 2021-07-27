use super::*;
use std::collections::HashMap;

#[cfg(test)]
mod tests;

impl <'a> Infix<'a>{
    pub fn evaluate(&'a self,variables:&VariableMap)->Result<isize,ExpressionError>{
        Postfix::from(self.clone()).evaluate(variables)
    }
}

impl<'a> From<Infix<'a>> for Postfix<'a>{
    fn from(prefix:Infix)->Postfix{
        //TODO remove this duplication pull operators out into separate types..
        let mut priorites = HashMap::new();
        priorites.insert("+",1);
        priorites.insert("-",1);
        priorites.insert("*",2);
        priorites.insert("/",2);

        let mut opperators = vec![];
        let mut new_expresstion= vec![];
        println!("before :{:?}",prefix.expression);
        for term in &prefix.expression {
            println!("term {}",*term);
            match *term {
                "+" | "-" | "*" | "/" => {
                    while let Some(opperator) = opperators.last() {
                        if priorites.get(opperator).unwrap()>=priorites.get(*term).unwrap(){
                            new_expresstion.push(opperators.pop().unwrap());
                        }else{
                            break;
                        }
                    }
                    opperators.push(*term);

                    println!("stack :{:?}",opperators);
                },
                _ => new_expresstion.push(*term)
            }
            println!("expression:{:?}",new_expresstion);
        }
        opperators.reverse();
        new_expresstion.append(&mut opperators);
        println!("after :{:?}",new_expresstion);
        Postfix::new_raw(
            new_expresstion
        )
    }
}

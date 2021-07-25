#[cfg(test)]
mod tests;
pub struct Infix<'a>{
    expression:&'a str,
}
impl <'a>Infix<'a>{
    pub fn new(expression:&'a str)->Self{
        let expression = expression.trim();
        Infix{expression}
    }
/*
    pub fn evaluate(&'a self,variables:&VariableMap)->Result<isize,ExpressionError>{
        Postfix::from(self).evaluate(variables)
    }
    */
}

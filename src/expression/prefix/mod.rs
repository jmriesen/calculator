use super::*;
mod tests;

impl <'a> Prefix<'a>{
    pub fn evaluate(&'a self,variables:&VariableMap)->Result<isize,ExpressionError>{
        Postfix::from(self.clone()).evaluate(variables)
    }
}

impl<'a> From<Prefix<'a>> for Postfix<'a>{
    fn from(prefix:Prefix)->Postfix{
        Postfix::new_raw(
            prefix.expression
                .into_iter()
                .rev()
                .collect()
        )
    }
}

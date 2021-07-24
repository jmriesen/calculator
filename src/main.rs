fn main() {
    println!("Hello, world!");
}

/*
Syntax of a postfix expression. (Note Spacing)
expression => |num|expression expression operand|
num        => 1|-1|2|-2 ...
operand    => |+|-|*|/|
*/
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_variable_value(){
        let mut variables = VariableMap::new();
        variables.insert(String::from("a"),7);
        assert_eq!(Postfix::new("a").evaluate(&variables), Ok(7));

        variables.insert(String::from("b"),0);
        assert_eq!(Postfix::new("b").evaluate(&variables), Ok(0));
    }

    #[test]
    fn extra_spaces(){
        assert_eq!(Postfix::new("8 ").evaluate(&Default::default()), Ok(8));
        assert_eq!(Postfix::new(" 8").evaluate(&Default::default()), Ok(8));
        assert_eq!(Postfix::new("    8").evaluate(&Default::default()), Ok(8));
        assert_eq!(Postfix::new("\t 8").evaluate(&Default::default()), Ok(8));
        assert_eq!(Postfix::new("8    9 +").evaluate(&Default::default()), Ok(17));
        assert_eq!(Postfix::new("8  \t  9 +").evaluate(&Default::default()), Ok(17));
    }

    #[test]
    fn operate_on_variables(){
        let mut variables = VariableMap::new();
        variables.insert(String::from("a"),5);
        variables.insert(String::from("b"),-7);
        variables.insert(String::from("c"),8);
        variables.insert(String::from("d"),-9);
        assert_eq!(Postfix::new("a b - c * d /").evaluate(&variables), Ok(-10));
    }

    #[test]
    fn can_not_override(){
        let mut variables = VariableMap::new();
        variables.insert(String::from("+"),5);
        variables.insert(String::from("7"),5);
        assert_eq!(Postfix::new("+").evaluate(&variables), Err(ToFewArgements));
        assert_eq!(Postfix::new("7").evaluate(&variables), Ok(7));
        variables.insert(String::from("7"),-6);

    }
    #[test]
    fn multiplication(){
        assert_eq!(Postfix::new("7 8 *").evaluate(&Default::default()), Ok(56));
    }

    #[test]
    fn division(){
        assert_eq!(Postfix::new("60 5 /").evaluate(&Default::default()), Ok(12));
        assert_eq!(Postfix::new("12 5 /").evaluate(&Default::default()), Ok(2));
    }

    #[test]
    fn number_expression() {
        assert_eq!(Postfix::new("7").evaluate(&Default::default()), Ok(7));
        assert_eq!(Postfix::new("8").evaluate(&Default::default()), Ok(8));
        assert_eq!(Postfix::new("-88").evaluate(&Default::default()), Ok(-88));
    }


    #[test]
    fn add(){
        assert_eq!(Postfix::new("7 8 +").evaluate(&Default::default()), Ok(15));
        assert_eq!(Postfix::new("7 7 +").evaluate(&Default::default()), Ok(14));
        assert_eq!(Postfix::new("1 3 + 4 +").evaluate(&Default::default()), Ok(8));
    }

    #[test]
    fn sub(){
        assert_eq!(Postfix::new("7 8 -").evaluate(&Default::default()), Ok(-1));
        assert_eq!(Postfix::new("7 7 -").evaluate(&Default::default()), Ok(0));
        assert_eq!(Postfix::new("1 3 - 4 -").evaluate(&Default::default()), Ok(-6));
    }

    #[test]
    fn mixed_operations(){
        assert_eq!(Postfix::new("1 3 + 4 -").evaluate(&Default::default()), Ok(0));
        assert_eq!(Postfix::new("1 3 - 4 +").evaluate(&Default::default()), Ok(2));
    }

    #[test]
    fn empty(){
        assert_eq!(Postfix::new(" ").evaluate(&Default::default()), Err(ParseError));
        assert_eq!(Postfix::new("").evaluate(&Default::default()), Err(ParseError));
    }
    #[test]
    fn to_many_arguments(){
        assert_eq!(Postfix::new("7 8").evaluate(&Default::default()), Err(ToManyArguments));
        assert_eq!(Postfix::new("7 8 9 +").evaluate(&Default::default()), Err(ToManyArguments));
    }

    #[test]
    fn to_few_arguments(){
        assert_eq!(Postfix::new("+").evaluate(&Default::default()), Err(ToFewArgements));
        assert_eq!(Postfix::new("9 -").evaluate(&Default::default()), Err(ToFewArgements));
        assert_eq!(Postfix::new("1 3 + +").evaluate(&Default::default()), Err(ToFewArgements));
    }
}

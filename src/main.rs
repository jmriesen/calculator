mod expression;
use expression::{Postfix,VariableMap};
fn main() {
    println!("Hello, world!");
    let mut variables = VariableMap::new();
    variables.insert(String::from("a"),7);
    let _ = Postfix::new("a").evaluate(&variables);


}

/*
Syntax of a postfix expression. (Note Spacing)
expression => |num|expression expression operand|
num        => 1|-1|2|-2 ...
operand    => |+|-|*|/|
*/

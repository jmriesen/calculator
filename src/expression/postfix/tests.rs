use super::*;

#[test]
fn should_read_variable_value(){
    let mut variables = VariableMap::new();
    variables.insert(String::from("a"),7);
    assert_eq!(Postfix::new("a").evaluate(&variables), Ok(7));

    variables.insert(String::from("b"),0);
    assert_eq!(Postfix::new("b").evaluate(&variables), Ok(0));
}

#[test]
fn should_ignore_extra_white_space(){
    assert_eq!(Postfix::new("8 ").evaluate(&Default::default()), Ok(8));
    assert_eq!(Postfix::new(" 8").evaluate(&Default::default()), Ok(8));
    assert_eq!(Postfix::new("    8").evaluate(&Default::default()), Ok(8));
    assert_eq!(Postfix::new("\t 8").evaluate(&Default::default()), Ok(8));
    assert_eq!(Postfix::new("8    9 +").evaluate(&Default::default()), Ok(17));
    assert_eq!(Postfix::new("8  \t  9 +").evaluate(&Default::default()), Ok(17));
}

#[test]
fn should_be_able_to_use_variables_in_expretions(){
    let mut variables = VariableMap::new();
    variables.insert(String::from("a"),5);
    variables.insert(String::from("b"),-7);
    variables.insert(String::from("c"),8);
    variables.insert(String::from("d"),-9);
    assert_eq!(Postfix::new("a b - c * d /").evaluate(&variables), Ok(-10));
}

#[test]
fn should_not_be_able_to_redefine_numbers_and_operation_symbols(){
    let mut variables = VariableMap::new();
    variables.insert(String::from("+"),5);
    variables.insert(String::from("7"),5);
    assert_eq!(Postfix::new("+").evaluate(&variables), Err(ToFewArgements));
    assert_eq!(Postfix::new("7").evaluate(&variables), Ok(7));
    variables.insert(String::from("7"),-6);

}
#[test]
fn should_be_able_to_multiply(){
    assert_eq!(Postfix::new("7 8 *").evaluate(&Default::default()), Ok(56));
}

#[test]
fn should_be_able_to_divide(){
    assert_eq!(Postfix::new("60 5 /").evaluate(&Default::default()), Ok(12));
    assert_eq!(Postfix::new("12 5 /").evaluate(&Default::default()), Ok(2));
}

#[test]
fn should_return_a_number_when_given_a_litteral() {
    assert_eq!(Postfix::new("7").evaluate(&Default::default()), Ok(7));
    assert_eq!(Postfix::new("8").evaluate(&Default::default()), Ok(8));
    assert_eq!(Postfix::new("-88").evaluate(&Default::default()), Ok(-88));
}


#[test]
fn should_be_able_to_add_numbers(){
    assert_eq!(Postfix::new("7 8 +").evaluate(&Default::default()), Ok(15));
    assert_eq!(Postfix::new("7 7 +").evaluate(&Default::default()), Ok(14));
    assert_eq!(Postfix::new("1 3 + 4 +").evaluate(&Default::default()), Ok(8));
}

#[test]
fn should_be_able_to_subtract_numbers(){
    assert_eq!(Postfix::new("7 8 -").evaluate(&Default::default()), Ok(-1));
    assert_eq!(Postfix::new("7 7 -").evaluate(&Default::default()), Ok(0));
    assert_eq!(Postfix::new("1 3 - 4 -").evaluate(&Default::default()), Ok(-6));
}

#[test]
fn should_be_able_to_use_all_operations_together(){
    assert_eq!(Postfix::new("1 3 + 4 -").evaluate(&Default::default()), Ok(0));
    assert_eq!(Postfix::new("1 3 - 4 +").evaluate(&Default::default()), Ok(2));
    assert_eq!(Postfix::new("5 6 * 1 3 - 4 + /").evaluate(&Default::default()), Ok(15));
}

#[test]
fn empty_string_should_produce_error(){
    assert_eq!(Postfix::new(" ").evaluate(&Default::default()), Err(ParseError));
    assert_eq!(Postfix::new("").evaluate(&Default::default()), Err(ParseError));
}
#[test]
fn to_many_arguments_should_produce_error(){
    assert_eq!(Postfix::new("7 8").evaluate(&Default::default()), Err(ToManyArguments));
    assert_eq!(Postfix::new("7 8 9 +").evaluate(&Default::default()), Err(ToManyArguments));
}

#[test]
fn to_few_arguments_should_produce_error(){
    assert_eq!(Postfix::new("+").evaluate(&Default::default()), Err(ToFewArgements));
    assert_eq!(Postfix::new("9 -").evaluate(&Default::default()), Err(ToFewArgements));
    assert_eq!(Postfix::new("1 3 + +").evaluate(&Default::default()), Err(ToFewArgements));
}

#[test]
fn should_not_allow_parenthesis(){
    assert_eq!(Postfix::new(")").evaluate(&Default::default()), Err(ParseError));
    assert_eq!(Postfix::new("5 6 ( +").evaluate(&Default::default()), Err(ParseError));
    assert_eq!(Postfix::new("5 6 ) +").evaluate(&Default::default()), Err(ParseError));
    assert_eq!(Postfix::new("5 6 + )").evaluate(&Default::default()), Err(ParseError));
}

use super::*;
#[test]
fn should_return_literals(){
    assert_eq!(Infix::new("7").evaluate(&Default::default()), Ok(7));
    assert_eq!(Infix::new("8").evaluate(&Default::default()), Ok(8));
}

#[test]
fn should_be_able_to_handle_two_term_expresstions(){
    assert_eq!(Infix::new("7 + 8").evaluate(&Default::default()), Ok(15));
    assert_eq!(Infix::new("7 - 8").evaluate(&Default::default()), Ok(-1));
    assert_eq!(Infix::new("7 * 8").evaluate(&Default::default()), Ok(56));
    assert_eq!(Infix::new("9 / 2").evaluate(&Default::default()), Ok(4));
}

#[test]
fn should_multiply_before_adding(){
    assert_eq!(Infix::new("2 * 7 + 8").evaluate(&Default::default()), Ok(22));
    assert_eq!(Infix::new("8 + 7 * 2").evaluate(&Default::default()), Ok(22));
    assert_eq!(Infix::new("4 * 2 * 7 + 2").evaluate(&Default::default()), Ok(58));
    assert_eq!(Infix::new("2 + 4 * 2 * 7").evaluate(&Default::default()), Ok(58));
}

#[test]
fn adding_and_subtracting_should_have_same_priority(){
    assert_eq!(Infix::new("2 - 7 + 8").evaluate(&Default::default()), Ok(3));
    assert_eq!(Infix::new("8 + 2 - 7").evaluate(&Default::default()), Ok(3));
}

#[test]
fn multiplication_and_division_should_have_same_priority(){
    assert_eq!(Infix::new("8 / 2 * 3").evaluate(&Default::default()), Ok(12));
    assert_eq!(Infix::new("3 * 8 / 2").evaluate(&Default::default()), Ok(12));
}
#[test]
fn adding_surrounding_parenthesies_dose_not_change_result(){
    let variables = Default::default();
    assert_eq!(Infix::new("3 + 2").evaluate(&variables),Infix::new("( 3 + 2 )").evaluate(&variables));
    assert_eq!(Infix::new("3 + 2").evaluate(&variables),Infix::new("( ( 3 + 2 ) )").evaluate(&variables));
}

#[test]
fn parenthesise_should_cause_preference(){
    let variables = Default::default();
    assert_eq!(Infix::new("( 3 + 2 ) * 8").evaluate(&variables),Ok(40));
    assert_eq!(Infix::new("( 3 + 7 ) / 5").evaluate(&variables),Ok(2));
}
#[test]
fn should_be_able_to_use_multiple_parenthesises(){
    let variables = Default::default();
    assert_eq!(Infix::new("( ( 3 + 2 ) * ( 7 + 1 ) ) + 5").evaluate(&variables),Ok(45));
}

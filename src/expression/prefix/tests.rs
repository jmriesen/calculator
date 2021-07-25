use super::*;
#[test]
fn should_return_literals(){
    assert_eq!(Prefix::new("7").evaluate(&Default::default()), Ok(7));
    assert_eq!(Prefix::new("8").evaluate(&Default::default()), Ok(8));
}

#[test]
fn should_be_able_to_add_two_numbers(){
    assert_eq!(Prefix::new("+ 7 8").evaluate(&Default::default()), Ok(15));
}

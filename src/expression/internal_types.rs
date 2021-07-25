#[derive(Clone,Copy)]
pub struct Infix;
#[derive(Clone,Copy)]
pub struct Postfix;
#[derive(Clone,Copy)]
pub struct Prefix;

//I am using trait as a way of gate keeping what can be used as and expression.
//Since this trait is only accessible in the mod expression.
pub trait ExpressionType{}

impl ExpressionType for Infix{}
impl ExpressionType for Postfix{}
impl ExpressionType for Prefix{}



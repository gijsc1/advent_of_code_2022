use std::collections::HashMap;
use crate::operation::Operation;

pub type ValType = f64;
pub const ZERO:ValType = 0.0;
pub const ONE:ValType = 1.0;

// pub type ValType = i64;
// pub const ZERO:ValType = 0;
// pub const ONE:ValType = 0;

pub const COMPARE_DELTA:ValType = ONE;
pub fn equals(lhs:ValType,rhs:ValType)->bool{
    (lhs-rhs).abs()<COMPARE_DELTA
}
pub type MonkeyID = String;
pub type MonkeyMap = HashMap<MonkeyID,Operation>;
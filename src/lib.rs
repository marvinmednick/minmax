use std::ops::{Add,Sub,AddAssign};
//use log::{ info, error, debug, /*warn,*/ trace };

use crate::MinMax::Value;

use std::fmt;

#[derive(Debug,Clone,Copy,PartialOrd,Ord,PartialEq,Eq)]
pub enum MinMax<T> where T: Clone {
    Min,
    Value(T),
    Max,
    NA,
}

/// Implement `Display` for `MinMax`.
impl<T: fmt::Display+ std::clone::Clone> fmt::Display for MinMax<T>

{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MinMax::Min => f.pad(&format!("Min")),
            MinMax::Max => f.pad(&format!("Max")),
            MinMax::NA =>  f.pad(&format!("NA")),
            MinMax::Value(ref x) =>  f.pad(&format!("{}", x))
        }
    }
}

/// Implement Add for MinMax
impl<T: std::ops::Add + std::cmp::PartialEq + Add<Output = T>+ std::clone::Clone> Add for MinMax<T> {
    type Output = MinMax<T>;

    fn add(self, other: MinMax<T>) -> MinMax<T> {

        match (self, other ) {
            (MinMax::Min, MinMax::Min)  | (MinMax::Min,MinMax::NA) | (MinMax::NA, MinMax::Min) =>  MinMax::Min,
            (MinMax::NA, MinMax::NA) => MinMax::NA,
            (Value(op1), MinMax::Min) | (Value(op1), MinMax::NA) => Value(op1),
            (MinMax::Min, Value(op2)) | (MinMax::NA, Value(op2)) => Value(op2),
            (MinMax::Max,_) | (_, MinMax::Max) => MinMax::Max,            
            (Value(op1), Value(op2)) => Value(op1+op2),
        }
    }
}

/// Implement Add for MinMax
impl<T: std::ops::Sub + std::cmp::PartialEq + Sub<Output = T>+ std::clone::Clone> Sub for MinMax<T> {
    type Output = MinMax<T>;

    fn sub(self, other: MinMax<T>) -> MinMax<T> {

        match (self, other ) {
            (MinMax::Min, MinMax::Min)  | (MinMax::Min,MinMax::NA) | (MinMax::NA, MinMax::Min) =>  MinMax::Min,
            (MinMax::NA, MinMax::NA) => MinMax::NA,
            (Value(op1), MinMax::Min) | (Value(op1), MinMax::NA) => Value(op1),
            (MinMax::Min, Value(_)) => MinMax::Min,
            (MinMax::NA, Value(_)) => MinMax::Min,
            (MinMax::Max,_) => MinMax::Max,
            (_, MinMax::Max) => MinMax::Min,
            (Value(op1), Value(op2)) => Value(op1-op2),
        }
    }
}


/// Implement AddAssign for MinMax
impl<T: std::ops::Add + std::cmp::PartialEq + Add<Output = T>+std::clone::Clone> AddAssign for MinMax<T> {

    fn add_assign(&mut self, other: Self) {
        *self =  self.clone() + other;
    }
}


// Othere Implmentations for MinMax
impl<T: std::fmt::Display+Clone> MinMax<T> {

    ///Unwrap Value into its contents
    pub fn unwrap_value(&self) -> &T {
        match self {
            Value(obj) => obj,
            _ => panic!("Non-Value minmax {}", self)
        }
    }

    ///Returns if MinMax has a specific value or not
    pub fn  is_value(&self) -> bool {
        match self {
            Value(_obj) => true,
            _ => false,
        }
    }
    
    ///Returns the specfic value of the MinMax if there is one, otherwise the provided default
    pub fn unwrap_value_or<'a>(&'a self, alt_value: &'a T) -> &T {
        match self {
            Value(obj) => obj,
            _ => alt_value,
        }
    }

}



#[cfg(test)]
mod min_max_tests {

    use crate::{MinMax::Value,MinMax::Min,MinMax::Max,MinMax::NA};

    #[test]
    pub fn basic_op() {
        let v1 = Value(5);
        let v2 = Value(6);
        let v3 = v1 + v2;
        assert_eq!(v3,Value(11));
        assert_eq!(v3-Value(3),Value(8));

        assert_eq!(v3+Max,Max);
        assert_eq!(v3+Min,v3);
        // currently as defined as NA + anyting is the anything
        assert_eq!(v3+NA,v3);
    }

    #[test]
    pub fn basic_float_op() {
        let v1 = Value(5.0);
        let v2 = Value(6.0);
        let v3 = v1 + v2;
        assert_eq!(v3,Value(11.0));
        assert_eq!(v3-Value(3.0),Value(8.0));

        assert_eq!(v3+Max,Max);
        assert_eq!(v3+Min,v3);
        // currently as defined as NA + anyting is the anything
        assert_eq!(v3+NA,v3);
    }

    #[test]
    pub fn unwrap_tests() {
        let v = Value(5.0);
        assert_eq!(v.is_value(), true);
        assert_eq!(v.unwrap_value(), &5.0);
        assert_eq!(v.unwrap_value_or(&3.0), &5.0);
        let v = NA;
        assert_eq!(v.is_value(), false);
        assert_eq!(v.unwrap_value_or(&3.0), &3.0);
        let v = Min;
        assert_eq!(v.is_value(), false);
        assert_eq!(v.unwrap_value_or(&3.0), &3.0);
        let v = Min;
        assert_eq!(v.is_value(), false);
        assert_eq!(v.unwrap_value_or(&3.0), &3.0);

    }

}

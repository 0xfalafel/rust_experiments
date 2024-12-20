use std::fmt;
use std::ops::{Add, Sub, Mul, Div};
use duplicate::duplicate_item;

// Percentage
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Percentage {
    pub value: f64
}

impl Percentage {
    pub fn new(value: f64) -> Percentage {
        Percentage { value: value}
    }
}

impl fmt::Display for Percentage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}%", self.value)
    }
}

// Implement operations for Percent

macro_rules! impl_arithmetic_op_for_Percentage {
    ($trait_name:ident $fn_name:ident $op:tt) => {
        
        impl $trait_name<Percentage> for Percentage {
            type Output = Percentage;
        
            fn $fn_name(self, rhs: Percentage) -> Self::Output {
                Percentage::new(self.value $op rhs.value)
            }
        }                
    };
}

/*
impl Add<Percentage> for Percentage {
    type Output = Percentage;

    fn add(self, rhs: Percentage) -> Self::Output {
        Percentage::new(self.value + rhs.value)
    }
}
*/
impl_arithmetic_op_for_Percentage!(Add add +);
impl_arithmetic_op_for_Percentage!(Sub sub -);
impl_arithmetic_op_for_Percentage!(Mul mul *);
impl_arithmetic_op_for_Percentage!(Div div /);


impl Add<f64> for Percentage {
    type Output = f64;

    fn add(self, rhs: f64) -> Self::Output {
        rhs + (rhs * self.value / 100.0)
    }
}

impl Add<Percentage> for f64 {
    type Output = f64;

    fn add(self, rhs: Percentage) -> Self::Output {
        rhs.add(self)
    }
}


impl Add<i32> for Percentage {
    type Output = f64;

    fn add(self, rhs: i32) -> Self::Output {
        f64::from(rhs) + (f64::from(rhs) * self.value / 100.0)
    }
}

impl Add<Percentage> for i32 {
    type Output = f64;

    fn add(self, rhs: Percentage) -> Self::Output {
        rhs.add(self)
    }
}

impl Mul<f64> for Percentage {
    type Output = f64;

    fn mul(self, rhs: f64) -> Self::Output {
        rhs * self.value / 100.0
    }
}

impl Mul<Percentage> for f64 {
    type Output = f64;
    fn mul(self, rhs: Percentage) -> Self::Output {
        rhs.mul(self)
    }
}

#[duplicate_item(Type; [f64]; [i128]; [i32];)]
impl Into<Type> for Percentage {
    fn into(self) -> Type {
        self.value as Type
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        assert_eq!(Percentage::new(15.0) + Percentage::new(22.0), Percentage { value: 37.0});
    }

    #[test]
    fn sub() {
        assert_eq!(Percentage::new(15.0) - Percentage::new(22.0), Percentage { value: -7.0});
    }

    #[test]
    fn mul() { assert_eq!(Percentage::new(-7.0) * Percentage::new(-2.0), Percentage { value: 14.0})}

    #[test]
    fn div() { assert_eq!(Percentage::new(13.0) / Percentage::new(2.0), Percentage { value: 6.5})}

    // #[test]
    // fn div_zero() { assert_eq!(Percentage::new(13.0) / Percentage::new(0.0), Percentage { value: 0.0})}
}
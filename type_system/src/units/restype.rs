use std::fmt;
use std::ops::{Add, Sub, Mul, Div};

use crate::Money;
use crate::Percentage;

#[derive(Debug, Clone, Copy)]
pub enum ResType {
    Int(i32),
    Float(f64),
    Money(Money),
    Percent(Percentage)
}

impl ResType {
    fn is_money(self) -> bool {
        matches!(self, ResType::Money(_))
    }

    fn money(self) -> Option<Money> {
        if let ResType::Money(money) = self {
            return Some(money)
        }
        None
    }

    fn is_float(self) -> bool {
        matches!(self, ResType::Float(_))
    }

    fn is_int(self) -> bool {
        matches!(self, ResType::Int(_))
    }

    fn is_percentage(self) -> bool {
        matches!(self, ResType::Percent(_))
    }
}

impl Into<f64> for ResType {
    fn into(self) -> f64 {
        match self {
            ResType::Float(f) => f,
            ResType::Int(i) => i as f64,
            ResType::Money(m) => m.into(),
            ResType::Percent(p) => p.into()
        }
    }
}

impl Into<i32> for ResType {
    fn into(self) -> i32 {
        match self {
            ResType::Float(f) => f as i32,
            ResType::Int(i) => i,
            ResType::Money(m) => m.into(),
            ResType::Percent(p) => p.into()
        }
    }
}

impl fmt::Display for ResType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResType::Int(i) => write!(f, "{}", i),
            ResType::Float(float) => write!(f, "{}", float),
            ResType::Money(m) =>  write!(f, "{}", m),
            ResType::Percent(p) =>  write!(f, "{}", p),
        }
    }
}

macro_rules! impl_arithmetic_op_for_ResType {
    ($trait_name:ident $fn_name:ident $op:tt) => {

        impl $trait_name <ResType> for ResType {
            type Output = ResType;

            fn $fn_name (self, rhs: ResType) -> Self::Output {
                
                // We have some money, the result will be money
                if self.is_money() || rhs.is_money() {
                    let (money, other) = match (self, rhs) {
                        (a, other) if a.is_money() => (a, other),
                        (a, other) if other.is_money() => (other, a),
                        (_, _) => unreachable!(),
                    };

                    let x: Money = money.money().unwrap();

                    match other {
                        ResType::Percent(p) => {
                            return ResType::Money(x $op p)
                        },
                        ResType::Float(f) => {
                            return ResType::Money(x $op f);
                        },
                        ResType::Money(m) => {
                            return ResType::Money(x $op m);
                        },
                        ResType::Int(i) => {
                            return ResType::Money(x $op i);
                        },
                    }
                }
                

                // We have some float, the result will be a float
                if self.is_float() || rhs.is_float() {
                    let (float, other) = match (self, rhs) {
                        (a, other) if a.is_float() => (a, other),
                        (a, other) if other.is_float() => (other, a),
                        (_, _) => unreachable!(),
                    };

                    let x: f64 = float.into();

                    match other {
                        ResType::Percent(p) => {
                            return ResType::Float(x $op p)
                        },
                        ResType::Float(f) => {
                            return ResType::Float(x $op f);
                        },
                        ResType::Int(i) => {
                            return ResType::Float(x $op f64::from(i));
                        },
                        ResType::Money(_) => {
                            unreachable!("Money should have been catch by the previous code.")
                        },
                    }
                }
                
                // Same thing for the int type
                if self.is_int() || rhs.is_int() {
                    let (int, other) = match (self, rhs) {
                        (a, other) if a.is_int() => (a, other),
                        (a, other) if other.is_int() => (other, a),
                        (_, _) => unreachable!(),
                    };

                    let x: i32 = int.into();

                    match other {
                        ResType::Percent(p) => {
                            return ResType::Float(x $op p)
                        },
                        ResType::Int(i) => {
                            return ResType::Int(x $op i);
                        },
                        _ => unreachable!("Other types should have been catch by the previously."),
                    }
                }
                

                // Finally our percent type
                if self.is_percentage() && rhs.is_percentage() {

                    if let (
                        ResType::Percent(p1),
                        ResType::Percent(p2)
                    ) = (self, rhs) {
                        return ResType::Percent(p1 $op p2)
                    }
                    
                    unreachable!("By construction, we should have 2 percentage type.")
                }
                

                unreachable!("All type should have be cratch by now")
            }
        }
    }
}

impl_arithmetic_op_for_ResType!(Add add +);
impl_arithmetic_op_for_ResType!(Sub sub +);
impl_arithmetic_op_for_ResType!(Mul mul +);
impl_arithmetic_op_for_ResType!(Div div +);

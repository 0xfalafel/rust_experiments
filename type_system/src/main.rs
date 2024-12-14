mod units;

use core::fmt;
use std::ops::{Add, Mul};

use units::percentage::Percentage;
use units::money::{Money, Currency};

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
enum ResType {
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

impl Add<ResType> for ResType {
    type Output = ResType;

    fn add(self, rhs: ResType) -> Self::Output {
        
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
                    return ResType::Money(x + p)
                },
                ResType::Float(f) => {
                    return ResType::Money(x + f);
                },
                ResType::Money(m) => {
                    return ResType::Money(x + m);
                },
                ResType::Int(i) => {
                    return ResType::Money(x + i);
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
                    return ResType::Float(x + p)
                },
                ResType::Float(f) => {
                    return ResType::Float(x + f);
                },
                ResType::Int(i) => {
                    return ResType::Float(x + f64::from(i));
                },
                ResType::Money(_) => {
                    unreachable!("Money should have been catch by the previous code.")
                },
            }
        }
        
        todo!()
    }
}

impl Mul<ResType> for ResType {
    type Output = ResType;

    fn mul(self, rhs: ResType) -> Self::Output {
        
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
                    return ResType::Money(x * p)
                },
                ResType::Float(f) => {
                    return ResType::Money(x * f);
                },
                ResType::Money(m) => {
                    return ResType::Money(x * m);
                },
                ResType::Int(i) => {
                    return ResType::Money(x * i);
                },
            }
        }
        
        todo!()
    }
}

fn main() {
    let a = ResType::Money(Money::new(42.0, Currency::Euros));
    let b = ResType::Percent(Percentage::new(20.0));

    let c = a + b;
    println!("a + b: {}", c);

    println!("12% of 42€: {}", Money::new(42.0, Currency::Euros) * Percentage::new(12.0));

    println!("42€ + 12: {}", Money::new(42.0, Currency::Euros) + 12);
}

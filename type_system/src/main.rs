mod units;
use units::percentage::Percentage;
use units::money::{Money, Currency};

use std::ops::Add;

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
enum ResType {
    Percent(Percentage),
    Money(Money),
    Float(f64)
}

impl ResType {
    fn is_percent(self) -> bool {
        matches!(self, ResType::Percent(_))
    }

    fn is_money(self) -> bool {
        matches!(self, ResType::Money(_))
    }

    fn is_float(self) -> bool {
        matches!(self, ResType::Float(_))
    }
}

// Priority of types:
// Money > Float > Percent
// 5€ > 3.14 > 1 %

impl Add<ResType> for ResType {
    type Output = ResType;

    fn add(self, rhs: ResType) -> Self::Output {
        
        // if one of the types is Money, we return some Money
        if self.is_money() || rhs.is_money() {
            let (money, other) = match (self, rhs) {
                (a, b) if a.is_money() => (a, b),
                (a, b) if b.is_money() => (b, a),
                _ => unreachable!("Either a or b is of type money.")
            };

            if let ResType::Money(m) = money {
                let result = match other {
                    ResType::Money(m2) => ResType::Money(m + m2),
                    ResType::Float(f) => ResType::Money(m + f),
                    ResType::Percent(p) => ResType::Money(m + p)
                };

                return result;
            }
        }

        // if one of the types is Float, we return a Float
        if self.is_float() || rhs.is_float() {
            let (float, other) = match (self, rhs) {
                (a, b) if a.is_float() => (a, b),
                (a, b) if b.is_float() => (b, a),
                _ => unreachable!("Either a or b is of type float.")
            };

            if let ResType::Float(f) = float {
                let result = match other {
                    ResType::Float(f2) => ResType::Float(f + f2),
                    ResType::Percent(p) => ResType::Float(f + p),
                    _ => unreachable!("ResType::Money was handled earlier.")
                };

                return result;
            }
        }

        // Only case reaming is when we have 2 percentages
        if self.is_percent() &&  rhs.is_percent() {
            if let (
                ResType::Percent(p1),
                ResType::Percent(p2)
            ) = (self, rhs) {
                return ResType::Percent(p1 + p2)
            }
        }

        unreachable!("All possible ResType combination have been handeled.")
    }
}


fn main() {
    let a = ResType::Money(Money::new(110.0, Currency::Euros));
    let b = ResType::Percent(Percentage::new(11.0));

    let res = a + b;
    if let ResType::Money(m) = res {
        println!("{}", m);
    }
}

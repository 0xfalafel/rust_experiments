mod units;
use units::percentage::Percentage;
use units::money::{Money, Currency};

use std::ops::Add;

#[allow(unused)]
enum ResType {
    Percent(Percentage),
    Money(Money),
    Float(f64)
}

impl ResType {
    fn is_percent(&self) -> bool {
        matches!(self, ResType::Percent(_))
    }

    fn is_money(&self) -> bool {
        matches!(self, ResType::Money(_))
    }

    fn is_float(&self) -> bool {
        matches!(self, ResType::Float(_))
    }
}

impl Add<ResType> for ResType {
    type Output = ResType;

    fn add(self, rhs: ResType) -> Self::Output {
        
        // if one of the types is Money, we return some Money
        if self.is_money() || rhs.is_money() {
            let (a, b) = match (self, rhs) {
                (a, b) if a.is_money() => (a, b),
                (a, b) if b.is_money() => (b, a),
                _ => unreachable!("Either a or b is of type money.")
            };

            if let ResType::Money(money) = a {
                let result = match b {
                    ResType::Money(m) => ResType::Money(money + m),
                    ResType::Float(f) => ResType::Money(money + f),
                    ResType::Percent(p) => ResType::Money(money + p)
                };

                return result;
            }
        }

        todo!()
    }
}


fn main() {
    println!("42€ + 12%: {}", Money::new(42.0, Currency::Euros) + Percentage::new(12.0));
    // 42€ + 12%: 47.04€
}

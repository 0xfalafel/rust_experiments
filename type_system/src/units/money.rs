use std::fmt;
use std::ops::{Add, Sub, Mul, Div};
use duplicate::duplicate_item;

use crate::Percentage;

// Currency Type
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Currency {
    Euros,
    Dollars
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Currency::Euros => '€',
            Currency::Dollars => '$',
        };
        write!(f, "{}", symbol)
    }
}


// Money Type
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Money {
    pub amount: f64,
    pub currency: Currency
}

impl Money {
    pub fn new(amount: f64, currency: Currency) -> Money {
        Money {amount: amount, currency: currency}
    }

    fn conversion(self, new_currency: Currency) -> Money {
        if self.currency == new_currency {
            return self;
        }

        if self.currency == Currency::Euros && new_currency == Currency::Dollars {
            Money::new(self.amount * 1.05, new_currency)
            
        } else if self.currency == Currency::Dollars && new_currency == Currency::Euros {
            Money::new(self.amount / 1.05, new_currency)
            
        } else {
            unreachable!("Fuck, we need to thing about conversion now :(")
        }
    }
}

/*
    Implement arithmetic operation for Money
    with other types

    i.e 42€ + 10.0
*/

// implement Add for f64, i32
#[duplicate_item(Type; [f64]; [i32];)]

impl Add<Type> for Money {
    type Output = Money;

    fn add(self, rhs: Type) -> Self::Output {
        Money::new(self.amount + f64::from(rhs), self.currency)
    }
}

// implement Sub for f64, i32
#[duplicate_item(Type; [f64]; [i32];)]

impl Sub<Type> for Money {
    type Output = Money;

    fn sub(self, rhs: Type) -> Self::Output {
        Money::new(self.amount - f64::from(rhs), self.currency)
    }
}

// implement Mul for f64, i32
#[duplicate_item(Type; [f64]; [i32];)]

impl Mul<Type> for Money {
    type Output = Money;

    fn mul(self, rhs: Type) -> Self::Output {
        Money::new(self.amount * f64::from(rhs), self.currency)
    }
}

// implement Div for f64, i32
#[duplicate_item(Type; [f64]; [i32];)]

impl Div<Type> for Money {
    type Output = Money;

    fn div(self, rhs: Type) -> Self::Output {
        Money::new(self.amount / f64::from(rhs), self.currency)
    }
}


impl Add<Money> for Money {
    type Output = Money;

    fn add(self, other: Money) -> Self::Output {
        let other = other.conversion(self.currency);
        Money::new(self.amount + other.amount, self.currency)
    }
}

impl Mul<Money> for Money {
    type Output = Money;

    fn mul(self, other: Money) -> Self::Output {
        let other = other.conversion(self.currency);
        Money::new(self.amount * other.amount, self.currency)
    }
}

// Implement Percentage operations
impl Add<Percentage> for Money {
    type Output = Money;

    fn add(self, rhs: Percentage) -> Self::Output {
        Money {
            amount: self.amount + (self.amount * rhs.value / 100.0),
            currency: self.currency
        }
    }
}

impl Mul<Percentage> for Money {
    type Output = Money;

    fn mul(self, rhs: Percentage) -> Self::Output {
        Money {
            amount: self.amount * rhs.value / 100.0,
            currency: self.currency
        }
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.currency {
            Currency::Dollars => write!(f, "${:.2}", self.amount),
            Currency::Euros => write!(f, "{:.2}€", self.amount)         
        }
    }
}

impl Into<f64> for Money {
    fn into(self) -> f64 {
        self.amount
    }
}

impl Into<i32> for Money {
    fn into(self) -> i32 {
        self.amount as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_money () {
        let bob = Money::new(1337.0, Currency::Dollars);
        let alice = Money::new(42.0, Currency::Euros);
        assert_eq!(bob + alice, Money{amount: 1381.1, currency: Currency::Dollars});
    }

    #[test]
    fn add_i32() {
        assert_eq!(Money::new(42.0, Currency::Euros) + 12, Money {amount: 54.0, currency: Currency::Euros});
    }
    
    #[test]
    fn mul_i32() {
        assert_eq!(Money::new(42.0, Currency::Euros) * 12, Money {amount: 504.0, currency: Currency::Euros});
    }

    #[test]
    fn add_f64() {
        assert_eq!(Money::new(7.0, Currency::Euros) + f64::from(6.0), Money {amount: 13.0, currency: Currency::Euros});
    }

    #[test]
    fn mul_f64() {
        assert_eq!(Money::new(7.0, Currency::Euros) * 6.0, Money {amount: 42.0, currency: Currency::Euros});
    }

    #[test]
    fn add_percent() {
        assert_eq!(Money::new(42.0, Currency::Euros) + Percentage::new(12.0), Money {amount: 47.04, currency: Currency::Euros});
    }

    #[test]
    fn percentage_of() {
        assert_eq!(Money::new(42.0, Currency::Euros) * Percentage::new(12.0), Money {amount: 5.04, currency: Currency::Euros});
    }
}
use std::fmt;

// Currency Type
#[allow(unused)]
enum Currency {
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

struct Money {
    amount: f64,
    currency: Currency
}

impl Money {
    pub fn new(amount: f64, currency: Currency) -> Money {
        Money {amount: amount, currency: currency}
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.currency {
            Currency::Dollars => write!(f, "${}", self.amount),
            Currency::Euros => write!(f, "{}€", self.amount)         
        }
    }
}

fn main() {
    let alice = Money::new(42.0, Currency::Euros);
    println! ("Alice has {}", alice);
}

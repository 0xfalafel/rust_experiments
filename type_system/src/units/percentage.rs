use std::fmt;
use std::ops::Add;

// Percentage
#[derive(Debug, PartialEq)]
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

impl Add<Percentage> for Percentage {
    type Output = Percentage;

    fn add(self, rhs: Percentage) -> Self::Output {
        Percentage::new(self.value + rhs.value)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        assert_eq!(Percentage::new(15.0) + Percentage::new(22.0), Percentage { value: 37.0});
    }
}
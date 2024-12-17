use std::fmt;
use std::ops::Add;

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

impl Add<Percentage> for Percentage {
    type Output = Percentage;

    fn add(self, rhs: Percentage) -> Self::Output {
        Percentage::new(self.value + rhs.value)
    }
}

impl Add<Percentage> for f64 {
    type Output = f64;

    fn add(self, rhs: Percentage) -> Self::Output {
        self + (self * rhs.value / 100.0)
    }
}

impl Add<f64> for Percentage {
    type Output = f64;

    fn add(self, rhs: f64) -> Self::Output {
        rhs.add(self)
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
    fn add_f64() {
        assert_eq!(Percentage::new(15.0) + 100.0, 115.0);
    }
}
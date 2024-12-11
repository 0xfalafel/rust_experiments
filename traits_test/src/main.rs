use core::f64;
use std::i128;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ResType {
    Int(i128),
    Float(f64),
}

impl ResType {
    fn get_i128(self) -> i128 {
        match self {
            ResType::Int(val)  => val,
            ResType::Float(val) => val as i128
        }
    }

    fn get_f64(self) -> f64 {
        match self {
            ResType::Int(val)  => val as f64,
            ResType::Float(val) => val
        }
    }

    fn is_float(&self) -> bool {
        matches!(self, ResType::Float(_) )
    }
}

fn arithmetic_operation<T, F>(a: ResType, b: ResType, func: F) -> ResType 
where 
    T: Copy,
    F: Fn(i128, i128) -> i128 +  Fn(f64, f64) -> f64,
{
    if a.is_float() || b.is_float() {
        let res: f64 = func(a.get_f64(), b.get_f64());
        return ResType::Float(res);
    } else {
        let res: i128 = func(a.get_i128(), b.get_i128());
        ResType::Int(res)

    }
}


fn main() {
    let a = ResType::Int(42);
    let b = ResType::Int(13);

    println!("Result ResType: {:?}", arithmetic_operation(a, b, |x,y| x+y));
}
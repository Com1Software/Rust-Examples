use thiserror::Error;

#[derive(Debug, Error)]
enum MathError {
    #[error("division by zero")]
    DivisionByZero,

    #[error("negative square root: {0}")]
    NegativeSqrt(f64),
}

fn divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

fn sqrt(n: f64) -> Result<f64, MathError> {
    if n < 0.0 {
        Err(MathError::NegativeSqrt(n))
    } else {
        Ok(n.sqrt())
    }
}

fn main() {
    println!("{:?}", divide(10.0, 2.0));
    println!("{:?}", divide(10.0, 0.0));

    println!("{:?}", sqrt(25.0));
    println!("{:?}", sqrt(-9.0));
}

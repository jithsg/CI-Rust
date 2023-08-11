//funstions to add, subtract, multiply and divide

pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

pub fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

pub fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

pub fn divide(x: f64, y: f64) -> Option<f64> {
    if y != 0.0 {
        Some(x / y)
    } else {
        None
    }
}

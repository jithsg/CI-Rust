use trust::{add, subtract, multiply, divide};

#[test]
fn test_add() {
    assert_eq!(add(2.0, 3.0), 5.0);
    assert_eq!(add(-2.0, 3.0), 1.0);
}

#[test]
fn test_subtract() {
    assert_eq!(subtract(5.0, 3.0), 2.0);
    assert_eq!(subtract(3.0, 3.0), 0.0);
}

#[test]
fn test_multiply() {
    assert_eq!(multiply(2.0, 3.0), 6.0);
    assert_eq!(multiply(2.0, -3.0), -6.0);
    assert_eq!(multiply(2.0, 0.0), 0.0);
}

#[test]
fn test_divide() {
    assert_eq!(divide(6.0, 2.0), Some(3.0));
    assert_eq!(divide(6.0, -2.0), Some(-3.0));
    assert_eq!(divide(2.0, 0.0), None);
}

/*run all functions in lib.rs */

use trust::add;
use trust::divide;
use trust::multiply;
use trust::subtract;

fn main() {
    let a = 10.0;
    let b = 5.0;

    println!("Addition: {}", add(a, b));
    println!("Subtraction: {}", subtract(a, b));
    println!("Multiplication: {}", multiply(a, b));
    println!("Division: {:?}", divide(a, b));
}

// oh heyy we wanna write a calculator with functions in rust

fn add(a: f64, b: f64) -> f64{
    a + b
}
fn subtract(a: f64, b: f64)  -> f64 {
    a - b
}

fn multiply(a: f64, b: f64) -> f64 {
    a * b
}
fn devide(a: f64, b: f64) -> Result<f64, String > {
        if b == 0.0 {
            Err("Error in deviding on zero".to_string())
        } else {
            Ok(a / b )
        }
}
fn main() {
    let x = 10.0;
    let y = 3.0;

    println!("sum of numbers: {}", add(x, y));
    println!("subtract of numbers: {}", subtract(x, y));
    println!("The multiplication of numbers: {}", multiply(x, y));

    match devide(x, y) {
        Ok(res) => println!("The Division Result: {:.2}", res),
        Err(e) => println!("{}", e),
    }

}

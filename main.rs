const MAX_ATTEMPTS: u8 = 3;
fn main() {
    let user_name = "amin";
    let password = "1234";
    let mut attempts = 0;
    let input_username = "admin";
    let input_password = "1234";

    if input_username == user_name && input_password == password {
        println!("login successful");
    } else {
        attempts += 1;
        println!("failed login, {} attempts used", attempts);
        println!("You have {} attempts left", MAX_ATTEMPTS - attempts);
    }
}

use std::io::{stdin, stdout, Write};

fn main() {
    let (mut num1, mut num2, mut num3, mut num4) =
        (String::new(), String::new(), String::new(), String::new());

    print!("Enter first number[0-9]: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut num1).expect("error");

    print!("Enter second number[0-9]: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut num2).expect("error");

    print!("Enter third number[0-9]: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut num3).expect("error");

    print!("Enter fourth number[0-9]: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut num4).expect("error");

    println!("You entered {}, {}, {}, {}", num1.trim(), num2.trim(), num3.trim(), num4.trim());
}

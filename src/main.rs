use std::io::{stdin, stdout, Write, BufWriter};

fn main() {
    print!("Enter first number[0-9]: ");
    stdout().flush().unwrap();
    let mut num1 = String::new();
    stdin().read_line(&mut num1).expect("error");

    print!("Enter second number[0-9]: ");
    stdout().flush().unwrap();
    let mut num2 = String::new();
    stdin().read_line(&mut num2).expect("error");

    print!("Enter third number[0-9]: ");
    stdout().flush().unwrap();
    let mut num3 = String::new();
    stdin().read_line(&mut num3).expect("error");

    print!("Enter fourth number[0-9]: ");
    stdout().flush().unwrap();
    let mut num4 = String::new();
    stdin().read_line(&mut num4).expect("error");

    let out = stdout();
    let mut out = BufWriter::new(out.lock());

    write!(out, "You entered {} {} {} {}", num1, num2, num3, num4).expect("error");
}

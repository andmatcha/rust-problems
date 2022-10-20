use std::io::{stdin, stdout, BufWriter, Write};

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

    let numbers = [num1, num2, num3, num4];
    let mut new_numbers: Vec<i32> = vec![];

    for number in numbers {
        let new_number: i32 = number.trim().parse().expect("error");
        new_numbers.push(new_number);
    }

    let out = stdout();
    let mut out = BufWriter::new(out.lock());

    writeln!(out, "You entered {:?}", new_numbers).expect("error");
}

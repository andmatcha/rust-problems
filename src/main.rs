mod calc;

use std::io::{stdin, stdout, Write};

fn main() {
    let (mut str_num1, mut str_num2, mut str_num3, mut str_num4) =
        (String::new(), String::new(), String::new(), String::new());

    print!("Enter first str_number[0-9]: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut str_num1).expect("error");

    print!("Enter second str_number[0-9]: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut str_num2).expect("error");

    print!("Enter third str_number[0-9]: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut str_num3).expect("error");

    print!("Enter fourth str_number[0-9]: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut str_num4).expect("error");

    let (num1, num2, num3, num4): (i32, i32, i32, i32) = (
        str_num1.trim().parse().ok().unwrap(),
        str_num2.trim().parse().ok().unwrap(),
        str_num3.trim().parse().ok().unwrap(),
        str_num4.trim().parse().ok().unwrap(),
    );

    println!("You entered {}, {}, {}, {}", num1, num2, num3, num4);

    calc::run();
}

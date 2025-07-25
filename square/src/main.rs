use std::io;

fn read_number_from_input() -> u32 {
    loop {
        println!("Please enter a number.");

        let mut number_input = String::new();

        io::stdin()
            .read_line(&mut number_input)
            .expect("Failed to read line");

        match number_input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input, please enter a valid number."),
        }
    }
}

fn main() {
    let number = read_number_from_input();

    if number == 4 {
        println!("Sorry it does not work")
    } else {
        let square = number * number;
        println!("The square of {} is {}.", number, square);
    }
}

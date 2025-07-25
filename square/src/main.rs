use std::io;

fn main() {
    println!("Please enter a number.");

    let mut number_input = String::new();

    io::stdin()
        .read_line(&mut number_input)
        .expect("Failed to read line");

    let number: u32 = match number_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, please enter a valid number.");
            return;
        }
    };

    if number == 4 {
        println!("Sorry it does not work")
    } else {
        let square = number * number;
        println!("The square of {} is {}.", number, square);
    }
}

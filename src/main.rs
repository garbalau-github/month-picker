use std::io;
use std::process;

fn main() {
    
    const MONTHS: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    const MONTHS_LENGTH: i32 = MONTHS.len() as i32;

    println!("Get month by number!");
    println!("Enter a number: ");

    let mut month: String = String::new();

    io::stdin().read_line(&mut month).expect("Failed to read line");

    let month = convert_to_num(&month);
    let month = month - 1;

    if month > MONTHS_LENGTH {
        println!("There is only 12 month!"); 
        process::exit(0);
    } else if month <= 0 {
        println!("Please, enter a positive number!"); 
        process::exit(0);
    } else {
        println!("This is {}", MONTHS[month as usize]);
    }

}

fn convert_to_num(value: &str) -> i32 {
     match value.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Enter a number, please!");
            process::exit(0);
        }
    }
}
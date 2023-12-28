use std::io;
//Convert temperatures between Fahrenheit and Celsius.
//°C = (°F - 32) × 5/9
fn main() {
    'outer: loop {
        println!("Enter a temperature to convert");
        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");

        let number: i32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        loop {
            println!("Enter a 'f' to convert to Farenheit");
            println!("Enter a 'c' to convert to Celsius");
            let mut sys = String::new();
            io::stdin()
                .read_line(&mut sys)
                .expect("Failed to read line");

            if sys.trim() == "c" {
                let c = farenheit_to_celsius(number);
                println!("The temperature converted to Celsius is {c}");
                break 'outer;
            }
            if sys.trim() == "f" {
                let f = celsius_to_farenheit(number);
                println!("The temperature converted to Farenheit is {f}");
                break 'outer;
            }
            println!("You did not enter 'c' or 'f'");
            continue;
        }
    }
}

fn celsius_to_farenheit(c: i32) -> i32 {
    (c * 9 / 5) + 32
}

fn farenheit_to_celsius(f: i32) -> i32 {
    (f - 32) * 5 / 9
}

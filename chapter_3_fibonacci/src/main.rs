use std::io;
fn main() {
    'outer: loop {
        println!("Find the nth sequence in a Fibonacci series");
        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");

        let number: i64 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let result = get_nth_fibonacci(number);
        println!("Result: {result}");
        break 'outer;
    }
}

fn get_nth_fibonacci(n: i64) -> i64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut prev = 0;
    let mut curr = 1;

    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }

    curr
}

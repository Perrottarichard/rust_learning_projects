fn main() {
    println!("{}", double_char("Richard is doubly awesome"));
    println!("{:?}", digitize(35231));
    println!(
        "{:?}",
        warn_the_sheep(&["sheep", "sheep", "sheep", "wolf", "sheep"])
    );
}

fn double_char(s: &str) -> String {
    let mut ans = String::new();
    for c in s.chars() {
        ans.push(c);
        ans.push(c)
    }
    ans
}

fn digitize(n: u64) -> Vec<u8> {
    let mut ans: Vec<u8> = Vec::new();
    for i in n.to_string().chars() {
        ans.push(i.to_digit(10).unwrap() as u8);
    }
    ans.reverse();
    ans
}

//This is fucking terrible
fn warn_the_sheep(queue: &[&str]) -> String {
    if let Some(last_element) = queue.iter().rev().next() {
        if last_element == &"wolf" {
            return String::from("Pls go away and stop eating my sheep");
        }
    }

    for (count, v) in queue.into_iter().enumerate() {
        if v as &str == "wolf" {
            let message = format!(
                "Oi! Sheep number {}! You are about to be eaten by a wolf!",
                queue.len() - count - 1
            );
            return String::from(&message);
        }
    }
    String::new()
}

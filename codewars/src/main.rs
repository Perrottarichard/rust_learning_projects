fn main() {
    println!("{}", double_char("Richard is doubly awesome"))
}

fn double_char(s: &str) -> String {
    let mut ans = String::new();
    for c in s.chars() {
        ans.push(c);
        ans.push(c)
    }
    ans
}

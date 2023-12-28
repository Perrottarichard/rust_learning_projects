fn main() {
    let days = [
        ("first", "a", "partrige in a pear tree."),
        ("second", "two", "turtle doves,"),
        ("third", "three", "french hens,"),
        ("fourth", "four", "calling birds,"),
        ("fifth", "five", "golden rings,"),
        ("sixth", "six", "geese a-laying,"),
        ("seventh", "seven", "swans a-swimming,"),
        ("eighth", "eight", "maids a-milking,"),
        ("ninth", "nine", "ladies dancing,"),
        ("tenth", "ten", "lords a-leaping,"),
        ("eleventh", "eleven", "pipers piping,"),
        ("twelfth", "twelve", "drummers drumming,"),
    ];
    let mut playback: Vec<(&str, &str, &str)> = Vec::new();
    for day in days {
        println!("On the {} day of Christmas", day.0);
        println!("my true love gave to me,");
        println!("{} {}", day.1, day.2);
        if !playback.is_empty() {
            let reversed_playback: Vec<_> = playback.iter().rev().collect();
            for (index, line) in reversed_playback.iter().enumerate() {
                if index == reversed_playback.len() - 1 {
                    println!("And {} {}", line.1, line.2);
                    break;
                }
                println!("{} {}", line.1, line.2);
            }
        }
        println!("");
        playback.push(day);
    }
}

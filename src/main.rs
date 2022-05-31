use regex::Regex;

fn main() {
    // Regex

    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();

    // User input

    println!("Enter an expression to evaluate:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // Math operations

    let caps = re_add.captures(input.as_str()).unwrap();

    let first_num = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let second_num = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();

    let addition = first_num + second_num;

    println!("{:?}, first {}, second {}", caps, first_num, second_num);

    // Show results

    println!("Result: {}", addition);
}

use regex::Regex;

fn main() {
    // Regex

    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let re_mul = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();

    // User input

    println!("Enter an expression to evaluate:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // Math operations

    // Multiplication

    loop {
        let caps = re_mul.captures(&input.as_str());

        if caps.is_none() {
            break;
        }

        let caps = caps.unwrap();

        let caps_expr = caps.get(0).unwrap().as_str();
        let caps_num1 = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let caps_num2 = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let caps_result = caps_num1 * caps_num2;

        input = input.replace(caps_expr, &caps_result.to_string());
    }

    // Addition

    loop {
        let caps = re_add.captures(input.as_str());

        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();
        let caps_expr = caps.get(0).unwrap().as_str();
        let first_num = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let second_num = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let addition = first_num + second_num;

        input = input.replace(caps_expr, &addition.to_string());
    }

    // Show results

    println!("Result: {}", input);
}

fn main() {
    ///write a classifer that can classify the following
    /// 1. if the number is even or odd
    /// 2. if the number is positive or negative
    /// 3. if the number is a multiple of 3 or 5
    
    let number = 10;
    let mut result = String::new();

    if number % 2 == 0 {
        result.push_str("even");
    } else {
        result.push_str("odd");
    }

    if number > 0 {
        result.push_str(" positive");
    } else {
        result.push_str(" negative");
    }

    if number % 3 == 0 {
        result.push_str(" multiple of 3");
    } else if number % 5 == 0 {
        result.push_str(" multiple of 5");
    }

    println!("{} is {}", number, result);
}

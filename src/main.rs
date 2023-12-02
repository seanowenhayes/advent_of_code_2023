fn main() {
    println!("Hello, world!");
}

fn calculate_line(line: &str) -> i32 {
    let mut digits = line.chars().filter(|c| c.is_digit(10));
    let first = digits.next().unwrap();
    let last = digits.last().unwrap();
    let answer = format!("{}{}", first, last);
    answer.parse::<i32>().unwrap()
}

// tests
#[cfg(test)]
mod tests {
    use crate::calculate_line;

    //1abc2
    #[test]
    fn first_line() {
        assert_eq!(calculate_line("1abc2"), 12);
    }
    //pqr3stu8vwx
    #[test]
    fn second_line() {
        assert_eq!(calculate_line("pqr3stu8vwx"), 38);
    }
    //a1b2c3d4e5f
    #[test]
    fn third_line() {
        assert_eq!(calculate_line("a1b2c3d4e5f"), 15);
    }
    //treb7uchet
    #[test]
    fn fourth_line() {
        assert_eq!(calculate_line("treb7uchet"), 77);
    }
}

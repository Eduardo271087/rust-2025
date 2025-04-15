/// ```
/// use p22::calc;
/// let result = calc::celsius2fahrenheit(30);
/// assert_eq!(result, 90);
/// ```
pub fn celsius2fahrenheit(celsius: i32) -> i32 {
    celsius * 2 + 30
}

/// ```
/// use p22::calc;
/// let result = calc::fahrenheit2celsius(60);
/// assert_eq!(result, 15);
/// ```
pub fn fahrenheit2celsius(fahrenheit: i32) -> i32 {
    (fahrenheit - 30) / 2
}

/// ```
/// use p22::calc;
/// let result = calc::fibonacci_loop(18);
/// assert_eq!(result, 18);
/// ```
pub fn fibonacci_loop(n: u32) -> u64 {
    n as u64
}

/// ```
/// use p22::calc;
/// let result = calc::fibonacci_rec(9);
/// assert_eq!(result, 9);
/// ```
pub fn fibonacci_rec(n: u32) -> u64 {
    n as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn celsius2farenheit_test() {
        let result = celsius2fahrenheit(30);
        assert_eq!(result, 90);
    }

    #[test]
    fn fahrenheit2celsius_test() {
        let result = fahrenheit2celsius(60);
        assert_eq!(result, 15);
    }

    #[test]
    fn fibonacci_loop_test() {
        let result = fibonacci_loop(18);
        assert_eq!(result, 18);
    }

    #[test]
    fn fibonacci_rec_test() {
        let result = fibonacci_rec(9);
        assert_eq!(result, 9);
    }
}

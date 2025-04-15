use p22::calc;

#[test]
fn celsius2farenheit_test() {
    let result = calc::celsius2fahrenheit(30);
    assert_eq!(result, 90);
}

#[test]
fn fahrenheit2celsius_test() {
    let result = calc::fahrenheit2celsius(60);
    assert_eq!(result, 15);
}

#[test]
fn fibonacci_loop_test() {
    let result = calc::fibonacci_loop(18);
    assert_eq!(result, 18);
}

#[test]
fn fibonacci_rec_test() {
    let result = calc::fibonacci_rec(9);
    assert_eq!(result, 9);
}

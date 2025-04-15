#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use p22::calc;
    use test::Bencher;

    #[bench]
    fn celsius2fahrenheit_bench(b: &mut Bencher) {
        b.iter(|| calc::celsius2fahrenheit(30));
    }

    #[bench]
    fn fahrenheit2celsius_bench(b: &mut Bencher) {
        b.iter(|| calc::fahrenheit2celsius(15));
    }

    #[bench]
    fn fibonacci_loop_bench(b: &mut Bencher) {
        b.iter(|| calc::fibonacci_loop(18));
    }

    #[bench]
    fn fibonacci_rec_bench(b: &mut Bencher) {
        b.iter(|| calc::fibonacci_rec(9));
    }
}

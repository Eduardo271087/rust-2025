fn f1(tup: &mut (u32, u32, bool)) -> &mut u32 {
    if tup.2 { &mut tup.1 } else { &mut tup.0 }
}

fn f2(s: &mut [u32], n: usize) -> &mut u32 {
    &mut s[n]
}

fn f3(s: &mut [u32], n: usize) -> &mut u32 {
    s.reverse();

    &mut s[n]
}

fn f4(s: &mut [u32]) -> (&[u32], &[u32], &[u32], &[u32]) {
    todo!();
}

mod tests {
    #[test]
    fn f1_true_test() {
        let expected_value = 7;

        let mut tup = (5, 7, true);
        let result = super::f1(&mut tup);
        assert_eq!(result, &expected_value);
    }

    #[test]
    fn f1_false_test() {
        let expected_value = 5;

        let mut tup = (5, 7, false);
        let result = super::f1(&mut tup);
        assert_eq!(result, &expected_value);
    }

    #[test]
    fn f2_test() {
        let expected_value: u32 = 8;

        let mut v: Vec<u32> = vec![1, 2, 3, 5, 8, 13, 21];
        let result = super::f2(&mut v[0..7], 4);
        assert_eq!(result, &expected_value);
    }

    #[test]
    fn f3_test() {
        let expected_value: u32 = 8;

        let mut v: Vec<u32> = vec![1, 2, 3, 5, 8, 13, 21];
        let result = super::f3(&mut v[0..7], 2);
        assert_eq!(result, &expected_value);
    }
}

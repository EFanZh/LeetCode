pub struct Solution {}

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        const FACTORIALS: [usize; 10] = [1, 1, 2, 6, 24, 120, 720, 5_040, 40_320, 362_880];
        let n = n as usize;
        let mut k = (k - 1) as usize;
        let mut digits_storage = [b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9'];
        let mut digits = &mut digits_storage as &mut [_];

        for base in FACTORIALS[1..n].iter().rev() {
            digits[..=k / base].rotate_right(1);
            k %= base;
            digits = &mut digits[1..];
        }

        String::from_utf8(digits_storage[..n].to_vec()).unwrap()
    }
}

impl super::Solution for Solution {
    fn get_permutation(n: i32, k: i32) -> String {
        Self::get_permutation(n, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

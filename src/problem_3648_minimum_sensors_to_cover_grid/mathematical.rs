pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_sensors(n: i32, m: i32, k: i32) -> i32 {
        let n = n.cast_unsigned();
        let m = m.cast_unsigned();
        let k = k.cast_unsigned();
        let size = 2 * k + 1;

        (n.div_ceil(size) * m.div_ceil(size)).cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_sensors(n: i32, m: i32, k: i32) -> i32 {
        Self::min_sensors(n, m, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

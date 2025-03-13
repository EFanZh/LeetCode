pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn piovt_integer(n: i32) -> i32 {
        let n = n as u32;
        let squared_result = n * (n + 1) / 2;
        let candidate = f64::from(squared_result).sqrt() as u32;

        if candidate * candidate == squared_result {
            candidate as _
        } else {
            -1
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn piovt_integer(n: i32) -> i32 {
        Self::piovt_integer(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

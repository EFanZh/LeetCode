pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
        let n = n.cast_unsigned();
        let time = time.cast_unsigned() % ((n - 1) * 2);

        (if time < n { time + 1 } else { n * 2 - (time + 1) }) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn pass_the_pillow(n: i32, time: i32) -> i32 {
        Self::pass_the_pillow(n, time)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

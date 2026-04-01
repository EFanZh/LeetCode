pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_containers(n: i32, w: i32, max_weight: i32) -> i32 {
        (max_weight.cast_unsigned() / w.cast_unsigned())
            .min(n.cast_unsigned().pow(2))
            .cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_containers(n: i32, w: i32, max_weight: i32) -> i32 {
        Self::max_containers(n, w, max_weight)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

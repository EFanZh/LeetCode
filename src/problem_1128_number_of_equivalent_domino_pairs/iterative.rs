pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let mut counts = [0; 81];

        for domino in &dominoes {
            let [lhs, rhs] = <[_; 2]>::try_from(domino.as_slice()).unwrap();
            let lhs = lhs as usize - 1;
            let rhs = rhs as usize - 1;
            let key = if rhs < lhs { (rhs * 9) + lhs } else { (lhs * 9) + rhs };
            let target = &mut counts[key];

            result += *target;

            *target += 1;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        Self::num_equiv_domino_pairs(dominoes)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

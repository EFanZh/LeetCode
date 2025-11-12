pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn get_indices(s: &str) -> [u8; 26] {
        let mut result = [0; 26];

        (1..)
            .zip(s.bytes())
            .for_each(|(i, c)| result[usize::from(c) - usize::from(b'a')] = i);

        result
    }

    pub fn find_permutation_difference(s: String, t: String) -> i32 {
        let s_indices = Self::get_indices(&s);
        let t_indices = Self::get_indices(&t);

        drop((s, t));

        s_indices
            .iter()
            .zip(&t_indices)
            .map(|(&i, &j)| i32::from(j.abs_diff(i)))
            .sum()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_permutation_difference(s: String, t: String) -> i32 {
        Self::find_permutation_difference(s, t)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

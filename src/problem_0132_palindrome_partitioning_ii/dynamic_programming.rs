pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let s = s.into_bytes();
        let mut palindromes = (0..s.len()).map(|i| (i, vec![i + 1])).collect::<HashMap<_, _>>();

        for i in 1..s.len() {
            let (left, right) = s.split_at(i);

            for (j, _) in left
                .iter()
                .rev()
                .zip(right)
                .take_while(|(lhs, rhs)| lhs == rhs)
                .enumerate()
            {
                palindromes.get_mut(&(i - 1 - j)).unwrap().push(i + 1 + j);
            }

            for (j, _) in left
                .iter()
                .rev()
                .zip(&right[1..])
                .take_while(|(lhs, rhs)| lhs == rhs)
                .enumerate()
            {
                palindromes.get_mut(&(i - 1 - j)).unwrap().push(i + 2 + j);
            }
        }

        let mut cache = vec![0; s.len()];

        for i in (0..s.len() - 1).rev() {
            cache[i] = palindromes[&i]
                .iter()
                .map(|j| cache.get(*j).map_or(0, |x| x + 1))
                .min()
                .unwrap();
        }

        cache[0]
    }
}

impl super::Solution for Solution {
    fn min_cut(s: String) -> i32 {
        Self::min_cut(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

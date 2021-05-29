pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    fn partition_helper(
        data: &[u8],
        palindromes: &HashMap<usize, Vec<usize>>,
        i: usize,
        base: &mut Vec<String>,
        result: &mut Vec<Vec<String>>,
    ) {
        if let Some(nexts) = palindromes.get(&i) {
            for &next in nexts {
                base.push(String::from_utf8(data[i..next].to_vec()).unwrap());

                Self::partition_helper(data, palindromes, next, base, result);

                base.pop();
            }
        } else {
            result.push(base.clone());
        }
    }

    pub fn partition(s: String) -> Vec<Vec<String>> {
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

        let mut result = Vec::new();

        Self::partition_helper(&s, &palindromes, 0, &mut Vec::new(), &mut result);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn partition(s: String) -> Vec<Vec<String>> {
        Self::partition(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

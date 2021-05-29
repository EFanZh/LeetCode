pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::slice::Iter;

impl Solution {
    fn add_operators_helper(
        first: i64,
        second: i64,
        third: i64,
        mut iter: Iter<u8>,
        target: i64,
        base: &mut Vec<u8>,
        result: &mut Vec<String>,
    ) {
        if let Some(&digit) = iter.next() {
            let i = base.len();
            let forth = (digit - b'0').into();

            base.extend(&[b'+', digit]);

            Self::add_operators_helper(first + second * third, 1, forth, iter.clone(), target, base, result);

            base[i] = b'-';

            Self::add_operators_helper(first + second * third, -1, forth, iter.clone(), target, base, result);

            base[i] = b'*';

            Self::add_operators_helper(first, second * third, forth, iter.clone(), target, base, result);

            if third != 0 {
                base.remove(i);

                Self::add_operators_helper(first, second, third * 10 + forth, iter, target, base, result);
            }

            base.truncate(i);
        } else if first + second * third == target {
            result.push(String::from_utf8(base.clone()).unwrap());
        }
    }

    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let mut result = Vec::new();
        let mut iter = num.as_bytes().iter();

        if let Some(&first) = iter.next() {
            let mut base = Vec::with_capacity(iter.len() * 2 - 1);

            base.push(first);

            Self::add_operators_helper(0, 1, (first - b'0').into(), iter, target.into(), &mut base, &mut result);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn add_operators(num: String, target: i32) -> Vec<String> {
        Self::add_operators(num, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

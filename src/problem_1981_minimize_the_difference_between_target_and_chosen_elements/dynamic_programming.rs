pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;
use std::mem;

impl Solution {
    pub fn minimize_the_difference(mat: Vec<Vec<i32>>, target: i32) -> i32 {
        let target = target as u16;
        let mut cache = HashSet::from([0]);
        let mut temp = HashSet::new();

        for row in mat {
            let iter = row.iter().map(|&value| value as u16);
            let mut min_not_less = u16::MAX;

            for &current in &cache {
                for value in iter.clone() {
                    let next = current + value;

                    if next < target {
                        temp.insert(next);
                    } else {
                        min_not_less = min_not_less.min(next);
                    }
                }
            }

            if min_not_less != u16::MAX {
                temp.insert(min_not_less);
            }

            mem::swap(&mut cache, &mut temp);
            temp.clear();
        }

        i32::from(
            cache
                .iter()
                .fold(u16::MAX, |min, value| min.min(value.abs_diff(target))),
        )
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimize_the_difference(mat: Vec<Vec<i32>>, target: i32) -> i32 {
        Self::minimize_the_difference(mat, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

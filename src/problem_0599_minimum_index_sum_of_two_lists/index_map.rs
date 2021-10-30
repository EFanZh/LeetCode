pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let (list1, mut list2) = if list2.len() < list1.len() {
            (list2, list1)
        } else {
            (list1, list2)
        };

        let indices = list1
            .into_iter()
            .enumerate()
            .map(|(i, name)| (name, i))
            .collect::<HashMap<_, _>>();

        let mut min_sum = usize::MAX;

        for (i, name) in list2.iter().enumerate() {
            if i <= min_sum {
                if let Some(j) = indices.get(name) {
                    let sum = i + j;

                    if sum < min_sum {
                        min_sum = sum;
                    }
                }
            }
        }

        let mut i = 0;

        list2.retain(|name| {
            let result = i <= min_sum && indices.get(name).map_or(false, |j| i + j == min_sum);

            i += 1;

            result
        });

        list2
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        Self::find_restaurant(list1, list2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

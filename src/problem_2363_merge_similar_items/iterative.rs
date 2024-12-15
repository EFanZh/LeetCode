pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut items1 = items1;
        let mut items2 = items2;

        if items2.len() < items1.len() {
            mem::swap(&mut items1, &mut items2);
        }

        let sort_key = |item: &Vec<_>| item[0] as u32;

        items1.sort_unstable_by_key(sort_key);

        for item in &mut items2 {
            let [value, weight] = <&mut [_; 2]>::try_from(item.as_mut_slice()).ok().unwrap();

            if let Ok(i) = items1.binary_search_by(|item| u32::cmp(&(item[0] as _), &(*value as _))) {
                *weight += mem::take(&mut items1[i][1]);
            }
        }

        items2.extend(items1.into_iter().filter(|item| item[1] != 0));

        items2.sort_unstable_by_key(sort_key);

        items2
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::merge_similar_items(items1, items2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

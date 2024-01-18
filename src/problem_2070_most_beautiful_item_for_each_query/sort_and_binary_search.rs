pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;

impl Solution {
    pub fn maximum_beauty(items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut items = items
            .into_iter()
            .map(|item| {
                let [price, beauty]: [_; 2] = item.try_into().ok().unwrap();

                (price as u32, beauty as u32)
            })
            .collect::<Box<_>>();

        items.sort_unstable_by_key(|&(price, _)| price);

        let mut max_beauty = 0;

        for (_, target) in &mut *items {
            max_beauty = max_beauty.max(*target);
            *target = max_beauty;
        }

        queries
            .into_iter()
            .map(|query| {
                let i = items.partition_point(|&(price, _)| price <= query as _);

                items.get(i.wrapping_sub(1)).map_or(0, |&(_, beauty)| beauty as _)
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_beauty(items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        Self::maximum_beauty(items, queries)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

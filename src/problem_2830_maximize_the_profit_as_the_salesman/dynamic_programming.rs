pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximize_the_profit(n: i32, offers: Vec<Vec<i32>>) -> i32 {
        let mut offers = offers
            .into_iter()
            .map(|offer| <(_, _, _)>::from(<[_; 3]>::try_from(offer).unwrap().map(|x| x as u32)))
            .collect::<Box<_>>();

        offers.sort_unstable_by_key(|offer| offer.1);

        let mut cache = vec![0_u32; n as u32 as usize].into_boxed_slice();
        let mut processed = 0;
        let mut prev = 0;

        for (start, end, gold) in offers {
            let start = start as usize;
            let end = end as usize;

            cache[processed..=end].fill(prev);

            let candidate = cache.get((start).wrapping_sub(1)).copied().unwrap_or(0) + gold;
            let target = &mut cache[end];

            *target = (*target).max(candidate);
            processed = end + 1;
            prev = *target;
        }

        prev as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximize_the_profit(n: i32, offers: Vec<Vec<i32>>) -> i32 {
        Self::maximize_the_profit(n, offers)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

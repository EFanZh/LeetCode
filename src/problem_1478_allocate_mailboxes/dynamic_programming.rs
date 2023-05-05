pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

// See <https://leetcode.com/problems/allocate-mailboxes/solutions/685403/java-c-python-dp-solution/>.

impl Solution {
    pub fn min_distance(houses: Vec<i32>, k: i32) -> i32 {
        let mut houses = houses;

        houses.sort_unstable_by_key(|&x| x as u32);

        let n = houses.len();
        let k = k as u32 as usize;
        let mut sum = 0;

        let sums = houses
            .iter()
            .map(|&position| {
                sum += position as u32;

                sum
            })
            .collect::<Box<_>>();

        let distance = |start: usize, end: usize| {
            let get = |i| sums.get(i).copied().unwrap_or(0);
            let middle_1 = ((start + end) / 2).wrapping_sub(1);
            let middle_2 = (start + end - 1) / 2;

            (get(end - 1) - get(middle_2)) - (get(middle_1) - get(start.wrapping_sub(1)))
        };

        let mut cache = (1..=n).map(|end| distance(0, end)).collect::<Box<_>>();

        for mail_boxes in 2..=k {
            for end in (mail_boxes..=n).rev() {
                let mut best = u32::MAX;

                for start in mail_boxes - 1..end {
                    best = best.min(cache[start - 1] + distance(start, end));
                }

                cache[end - 1] = best;
            }
        }

        *cache.last().unwrap() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_distance(houses: Vec<i32>, k: i32) -> i32 {
        Self::min_distance(houses, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

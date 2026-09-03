pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn contains(seen: &[usize], num: i32) -> bool {
        let key = (num + 100_000).cast_unsigned();

        seen.get((key / usize::BITS) as usize)
            .is_some_and(|&bucket| bucket & (1 << (key % usize::BITS)) != 0)
    }

    fn insert(seen: &mut [usize], seen_buckets: &mut Vec<u16>, num: i32) {
        let key = (num + 100_000).cast_unsigned();
        let bucket_index = key / usize::BITS;
        let bit = 1 << (key % usize::BITS);
        let bucket = &mut seen[bucket_index as usize];

        if *bucket & bit == 0 {
            if *bucket == 0 {
                seen_buckets.push(bucket_index as _);
            }

            *bucket |= bit;
        }
    }

    pub fn centered_subarrays(nums: Vec<i32>) -> i32 {
        let mut seen = [0_usize; usize::div_ceil(200_001, usize::BITS as _)];
        let mut seen_buckets = Vec::new();
        let mut iter = nums.iter().copied();
        let mut result = nums.len() as _;

        while let Some(mut sum) = iter.next() {
            Self::insert(&mut seen, &mut seen_buckets, sum);

            for num in iter.clone() {
                sum += num;
                Self::insert(&mut seen, &mut seen_buckets, num);
                result += i32::from(Self::contains(&seen, sum));
            }

            while let Some(bucket) = seen_buckets.pop() {
                seen[usize::from(bucket)] = 0;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn centered_subarrays(nums: Vec<i32>) -> i32 {
        Self::centered_subarrays(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

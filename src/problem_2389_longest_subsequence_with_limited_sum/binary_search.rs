pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut queries = queries;

        nums.sort_unstable();

        let mut sum = 0;

        for num in &mut nums {
            sum += *num;
            *num = sum;
        }

        for query in &mut queries {
            let target = *query;

            *query = nums.partition_point(|&x| x <= target) as _;
        }

        queries
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        Self::answer_queries(nums, queries)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

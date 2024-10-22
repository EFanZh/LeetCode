pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn reverse(mut x: u32) -> u32 {
        let mut result = 0;

        loop {
            result = result * 10 + x % 10;
            x /= 10;

            if x == 0 {
                return result;
            }
        }
    }

    pub fn kth_palindrome(queries: Vec<i32>, int_length: i32) -> Vec<i64> {
        let int_length = int_length as u32;
        let half_int_length = int_length / 2;
        let other_half_int_length = int_length - half_int_length;
        let first = u32::pow(10, other_half_int_length - 1);
        let count = first * 9;
        let offset = first - 1;
        let shift = u64::from(u32::pow(10, half_int_length));
        let iter = queries.into_iter();

        if int_length & 1 == 0 {
            iter.map(|query| {
                let query_u32 = query as u32;

                if query_u32 <= count {
                    let left = offset + query_u32;

                    (u64::from(left) * shift + u64::from(Self::reverse(left))) as _
                } else {
                    -1
                }
            })
            .collect()
        } else {
            iter.map(|query| {
                let query_u32 = query as u32;

                if query_u32 <= count {
                    let left = offset + query_u32;

                    (u64::from(left) * shift + u64::from(Self::reverse(left / 10))) as _
                } else {
                    -1
                }
            })
            .collect()
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn kth_palindrome(queries: Vec<i32>, int_length: i32) -> Vec<i64> {
        Self::kth_palindrome(queries, int_length)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

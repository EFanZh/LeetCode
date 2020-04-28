pub struct Solution {}

impl Solution {
    fn acc_max<'a, I: 'a + IntoIterator<Item = &'a i32>>(iter: I) -> impl 'a + Iterator<Item = i32> {
        let mut state = 0;

        iter.into_iter().map(move |x| {
            state = state.max(*x);

            state
        })
    }

    pub fn trap(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }

        let mut maximums = vec![0; height.len() * 2];
        let (left_maxs, right_maxs) = maximums.split_at_mut(height.len());

        for (target, num) in left_maxs[1..].iter_mut().zip(Self::acc_max(&height)) {
            *target = num;
        }

        for (target, num) in right_maxs[1..].iter_mut().zip(Self::acc_max(height.iter().rev())) {
            *target = num;
        }

        let mut result = 0;

        for (num, (left_max, right_max)) in height.into_iter().zip(left_maxs.iter().zip(right_maxs.iter().rev())) {
            let &min = left_max.min(right_max);

            if min > num {
                result += min - num;
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn trap(height: Vec<i32>) -> i32 {
        Self::trap(height)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

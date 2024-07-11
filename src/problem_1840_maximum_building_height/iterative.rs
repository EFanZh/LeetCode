pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_building(n: i32, restrictions: Vec<Vec<i32>>) -> i32 {
        let n = n as u32;

        let mut restrictions = restrictions
            .into_iter()
            .map(|restriction| {
                let [id, max_height]: [_; 2] = restriction.try_into().ok().unwrap();

                (id as u32, max_height as u32)
            })
            .collect::<Box<_>>();

        restrictions.sort_unstable_by_key(|&(id, _)| id);

        let mut prev = (1, 0);

        for restriction in &mut *restrictions {
            restriction.1 = restriction.1.min(prev.1 + (restriction.0 - prev.0));
            prev = *restriction;
        }

        let mut result = prev.1 + (n - prev.0);

        for restriction in restrictions.iter_mut().rev() {
            restriction.1 = restriction.1.min(prev.1 + (prev.0 - restriction.0));
            prev = *restriction;
        }

        prev = (1, 0);

        for &restriction in &*restrictions {
            result = result.max((restriction.0 - prev.0 + prev.1 + restriction.1) / 2);
            prev = restriction;
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_building(n: i32, restrictions: Vec<Vec<i32>>) -> i32 {
        Self::max_building(n, restrictions)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

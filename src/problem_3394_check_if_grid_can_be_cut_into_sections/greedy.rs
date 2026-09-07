pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn check(rectangles: &[Vec<i32>], buffer: &mut Vec<u32>, mut key: impl FnMut(&[i32]) -> (i32, i32)) -> bool {
        buffer.extend(rectangles.iter().flat_map(|rectangle| {
            let (start, end) = key(rectangle);

            [start.cast_unsigned() * 2 + 1, end.cast_unsigned() * 2]
        }));

        buffer.sort_unstable();

        let mut count = 0;

        let mut iter_0 = buffer.iter();

        let mut iter_1 = iter_0.by_ref().filter(|&&state| {
            if state.is_multiple_of(2) {
                count -= 1;
            } else {
                count += 1;
            }

            count == 0
        });

        iter_1.next().is_some() && iter_1.next().is_some() && iter_0.next().is_some()
    }

    pub fn check_valid_cuts(n: i32, rectangles: Vec<Vec<i32>>) -> bool {
        _ = n;

        let mut buffer = Vec::new();

        Self::check(&rectangles, &mut buffer, |rectangle| {
            let [start_x, _, end_x, _] = rectangle.try_into().ok().unwrap();

            (start_x, end_x)
        }) || {
            buffer.clear();

            Self::check(&rectangles, &mut buffer, |rectangle| {
                let [_, start_y, _, end_y] = rectangle.try_into().ok().unwrap();

                (start_y, end_y)
            })
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn check_valid_cuts(n: i32, rectangles: Vec<Vec<i32>>) -> bool {
        Self::check_valid_cuts(n, rectangles)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

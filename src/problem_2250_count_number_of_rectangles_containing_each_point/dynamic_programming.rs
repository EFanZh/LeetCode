pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_rectangles(rectangles: Vec<Vec<i32>>, points: Vec<Vec<i32>>) -> Vec<i32> {
        let parse_point = |point| <[_; 2]>::try_from(point).ok().unwrap().map(|x| x as u32);
        let mut rectangles = rectangles.into_iter().map(parse_point).collect::<Vec<_>>();

        rectangles.sort_unstable_by_key(|&[w, _]| w);

        let mut cache = Vec::with_capacity(rectangles.len());
        let mut state = (rectangles[0][0], [0_u16; 100]);

        for [w, h] in rectangles {
            if w != state.0 {
                cache.push(state);
                state = (w, [0; 100]);
            }

            state.1[h as usize - 1] += 1;
        }

        cache.push(state);

        let mut prev = &[0; 100];

        for (_, state) in cache.iter_mut().rev() {
            let mut top = 0;

            for (target, &right) in state.iter_mut().zip(prev).rev() {
                top += *target;
                *target = top + right;
            }

            prev = state;
        }

        points
            .into_iter()
            .map(|point| {
                let [x, y] = parse_point(point);
                let i = cache.partition_point(|&(w, _)| w < x);

                i32::from(cache.get(i).map_or(0, |(_, counts)| counts[y as usize - 1]))
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_rectangles(rectangles: Vec<Vec<i32>>, points: Vec<Vec<i32>>) -> Vec<i32> {
        Self::count_rectangles(rectangles, points)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

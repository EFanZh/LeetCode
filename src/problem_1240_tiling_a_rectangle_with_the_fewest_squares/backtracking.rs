pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

struct Context<'a> {
    rows: usize,
    column_heights: &'a mut [usize],
    remaining: usize,
    used_squares: usize,
    result: usize,
}

impl Solution {
    fn dfs(context: &mut Context) {
        if context.used_squares + 1 < context.result {
            let (start, start_height) = context
                .column_heights
                .iter()
                .copied()
                .enumerate()
                .min_by_key(|&(_, height)| height)
                .unwrap();

            let max_length = (context.rows - start_height).min(
                context.column_heights[start + 1..]
                    .iter()
                    .copied()
                    .take_while(|&height| height == start_height)
                    .count()
                    + 1,
            );

            if max_length * max_length == context.remaining {
                context.result = context.used_squares + 1;
            } else {
                let saved_remaining = context.remaining;

                context.used_squares += 1;

                for length in (1..=max_length).rev() {
                    context.column_heights[start..start + length].fill(start_height + length);
                    context.remaining = saved_remaining - length * length;
                    Self::dfs(context);
                    context.column_heights[start + length - 1] = start_height;
                }

                context.remaining = saved_remaining;
                context.used_squares -= 1;
            }
        }
    }

    pub fn tiling_rectangle(n: i32, m: i32) -> i32 {
        let n = n as usize;
        let m = m as usize;
        let (rows, columns) = if m < n { (n, m) } else { (m, n) };

        let mut buffer = [0; 13];

        let mut context = Context {
            rows,
            column_heights: &mut buffer[..columns],
            remaining: (columns * rows),
            used_squares: 0,
            result: usize::MAX,
        };

        Self::dfs(&mut context);

        context.result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn tiling_rectangle(n: i32, m: i32) -> i32 {
        Self::tiling_rectangle(n, m)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

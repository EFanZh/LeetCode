pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    fn bfs(
        source: (usize, usize),
        target: (usize, usize),
        forest: &[Vec<i32>],
        columns: usize,
        visited: &mut [bool],
        queue: &mut VecDeque<(usize, usize)>,
    ) -> i32 {
        let mut steps = 1;

        visited[columns * source.0 + source.1] = true;
        queue.push_back((source.0, source.1));

        loop {
            for _ in 0..queue.len() {
                let (row, column) = queue.pop_front().unwrap();

                for (next_row, next_column) in [
                    (row.wrapping_sub(1), column),
                    (row, column.wrapping_sub(1)),
                    (row, column + 1),
                    (row + 1, column),
                ] {
                    if (next_row, next_column) == target {
                        queue.clear();
                        visited.iter_mut().for_each(|v| *v = false);

                        return steps;
                    }

                    if let Some(&height) = forest.get(next_row).and_then(|r| r.get(next_column)) {
                        if let visited_value @ false = &mut visited[columns * next_row + next_column] {
                            *visited_value = true;

                            if height != 0 {
                                queue.push_back((next_row, next_column));
                            }
                        }
                    }
                }
            }

            if queue.is_empty() {
                break;
            }

            steps += 1;
        }

        -1
    }

    pub fn cut_off_tree(forest: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;

        let mut trees = forest
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(j, &x)| if x < 2 { None } else { Some(((i, j), x)) })
            })
            .collect::<Vec<_>>();

        trees.sort_unstable_by_key(|&(_, x)| x);

        let mut trees_iter = trees.iter().copied();

        if trees[0].0 == (0, 0) {
            trees_iter.next().unwrap();
        };

        let columns = forest[0].len();
        let mut visited = vec![false; columns * forest.len()];
        let mut queue = VecDeque::new();
        let mut source = (0, 0);

        for (target, _) in trees_iter {
            let steps = Self::bfs(source, target, &forest, columns, &mut visited, &mut queue);

            if steps == -1 {
                return -1;
            }

            result += steps;
            source = target;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn cut_off_tree(forest: Vec<Vec<i32>>) -> i32 {
        Self::cut_off_tree(forest)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

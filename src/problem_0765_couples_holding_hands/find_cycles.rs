pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_swaps_couples(row: Vec<i32>) -> i32 {
        let m = row.len();
        let mut neighbors = vec![0; m];

        for (&left, &right) in row.iter().zip(&row[1..]).step_by(2) {
            neighbors[left as usize] = right as usize;
            neighbors[right as usize] = left as usize;
        }

        let mut result = 0;
        let mut visited = vec![false; m];

        for (i, &neighbor) in neighbors.iter().enumerate() {
            if let visited_value @ false = &mut visited[i] {
                *visited_value = true;
                visited[neighbor] = true;

                let target = i + 1;
                let mut current = neighbor;

                while current != target {
                    let current_couple = current ^ 1;

                    current = neighbors[current_couple];
                    visited[current_couple] = true;
                    visited[current] = true;

                    result += 1;
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_swaps_couples(row: Vec<i32>) -> i32 {
        Self::min_swaps_couples(row)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

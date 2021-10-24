pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::convert::TryInto;
use std::mem;

struct Node {
    parent: u32,
    rank: u32,
    size: u32,
}

struct UnionFind {
    data: Vec<Node>,
}

impl UnionFind {
    fn new(length: u32) -> Self {
        Self {
            data: (0..length)
                .map(|_| Node {
                    parent: u32::MAX,
                    rank: 0,
                    size: 1,
                })
                .collect(),
        }
    }

    fn find_root(&mut self, x: u32) -> u32 {
        let parent = self.data[x as usize].parent;

        if parent == u32::MAX {
            x
        } else {
            let result = self.find_root(parent);

            self.data[x as usize].parent = result;

            result
        }
    }

    fn get_two_mut<T>(values: &mut [T], x: usize, y: usize) -> Option<(&mut T, &mut T)> {
        match x.cmp(&y) {
            Ordering::Less => {
                let (right, rest) = values[..=y].split_last_mut().unwrap();

                Some((&mut rest[x], right))
            }
            Ordering::Equal => None,
            Ordering::Greater => {
                let (right, rest) = values[..=x].split_last_mut().unwrap();

                Some((right, &mut rest[y]))
            }
        }
    }

    fn union(&mut self, left: u32, right: u32) {
        let left_root = self.find_root(left);
        let right_root = self.find_root(right);

        if let Some((left_root_node, right_root_node)) =
            Self::get_two_mut(&mut self.data, left_root as usize, right_root as usize)
        {
            if left_root_node.rank < right_root_node.rank {
                left_root_node.parent = right_root;
                right_root_node.size += left_root_node.size;
            } else {
                right_root_node.parent = left_root;
                left_root_node.size += right_root_node.size;

                if left_root_node.rank == right_root_node.rank {
                    left_root_node.rank += 1;
                }
            }
        }
    }

    fn get_root_size(&mut self) -> u32 {
        let root = self.find_root(0);

        self.data[root as usize].size
    }
}

impl Solution {
    pub fn hit_bricks(grid: Vec<Vec<i32>>, hits: Vec<Vec<i32>>) -> Vec<i32> {
        let mut grid = grid;
        let mut hits = hits;
        let rows = grid.len() as u32;
        let columns = grid.first().map_or(0, Vec::len) as u32;

        // Remove all bricks.

        for hit in &mut hits {
            let [y, x]: [i32; 2] = hit.as_slice().try_into().unwrap();

            if mem::replace(&mut grid[y as usize][x as usize], 0) == 0 {
                hit.clear();
            }
        }

        // Count groups.

        let mut union_find = UnionFind::new(1 + (columns * rows));
        let mut node = 1;

        // Union first row onto top.

        let (first_row, rest_rows) = grid.split_first().unwrap();

        for &value in first_row {
            if value != 0 {
                union_find.union(node, 0);
            }

            node += 1;
        }

        // Union rest rows

        for (prev_row, row) in grid.iter().zip(rest_rows) {
            let mut iter = prev_row.iter().copied().zip(row.iter().copied());

            let (top, value) = iter.next().unwrap();

            if value != 0 && top != 0 {
                union_find.union(node, node - columns);
            }

            node += 1;

            let mut prev = value;

            for (top, value) in iter {
                if value != 0 {
                    if prev != 0 {
                        union_find.union(node, node - 1);
                    }

                    if top != 0 {
                        union_find.union(node, node - columns);
                    }
                }

                prev = value;
                node += 1;
            }
        }

        // Union group back to original grid in reversed order.

        let mut result = vec![0; hits.len()];

        for (target, hit) in result.iter_mut().zip(&hits).rev() {
            let hit: Result<[_; 2], _> = hit.as_slice().try_into();

            if let Ok([y, x]) = hit {
                let (y, x) = (y as usize, x as usize);
                let node = columns * y as u32 + x as u32 + 1;
                let saved_stable = union_find.get_root_size();

                grid[y][x] = 1;

                if y == 0 {
                    union_find.union(node, 0);
                } else if grid[y - 1][x] != 0 {
                    union_find.union(node, node - columns);
                }

                if grid[y].get(x.wrapping_sub(1)).copied() == Some(1) {
                    union_find.union(node, node - 1);
                }

                if grid[y].get(x + 1).copied() == Some(1) {
                    union_find.union(node, node + 1);
                }

                if grid.get(y + 1).map(|row| row[x]) == Some(1) {
                    union_find.union(node, node + columns);
                }

                if let Some(value) = (union_find.get_root_size() - saved_stable).checked_sub(1) {
                    *target = value as _;
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn hit_bricks(grid: Vec<Vec<i32>>, hits: Vec<Vec<i32>>) -> Vec<i32> {
        Self::hit_bricks(grid, hits)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

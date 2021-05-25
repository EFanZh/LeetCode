use std::collections::HashMap;

pub struct SummaryRanges {
    nodes: HashMap<i32, (i32, u32)>,
    roots: HashMap<i32, (i32, i32)>,
}

impl SummaryRanges {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            roots: HashMap::new(),
        }
    }

    fn get_root(nodes: &mut HashMap<i32, (i32, u32)>, node: i32) -> Option<(i32, u32)> {
        nodes.get(&node).copied().map(|(parent, parent_rank)| {
            if node == parent {
                (parent, parent_rank)
            } else {
                let (root, root_rank) = Self::get_root(nodes, parent).unwrap();

                nodes.get_mut(&node).unwrap().0 = root;

                (root, root_rank)
            }
        })
    }

    fn add_num(&mut self, val: i32) {
        if !self.nodes.contains_key(&val) {
            match (
                Self::get_root(&mut self.nodes, val - 1),
                Self::get_root(&mut self.nodes, val + 1),
            ) {
                (None, None) => {
                    self.nodes.insert(val, (val, 0));
                    self.roots.insert(val, (val, val));
                }
                (None, Some((right_root, _))) => {
                    self.nodes.insert(val, (right_root, 0));
                    self.roots.get_mut(&right_root).unwrap().0 = val;
                }
                (Some((left_root, _)), None) => {
                    self.nodes.insert(val, (left_root, 0));
                    self.roots.get_mut(&left_root).unwrap().1 = val;
                }
                (Some((left_root, left_rank)), Some((right_root, right_rank))) => {
                    if left_rank < right_rank {
                        self.nodes.insert(val, (right_root, 0));
                        self.nodes.get_mut(&left_root).unwrap().0 = right_root;
                        self.roots.get_mut(&right_root).unwrap().0 = self.roots.remove(&left_root).unwrap().0;
                    } else {
                        self.nodes.insert(val, (left_root, 0));
                        self.nodes.get_mut(&right_root).unwrap().0 = left_root;
                        self.roots.get_mut(&left_root).unwrap().1 = self.roots.remove(&right_root).unwrap().1;

                        if left_rank == right_rank {
                            self.nodes.get_mut(&left_root).unwrap().1 += 1;
                        }
                    }
                }
            }
        }
    }

    fn get_intervals(&self) -> Vec<Vec<i32>> {
        let mut result = self
            .roots
            .values()
            .map(|&(from, to)| vec![from, to])
            .collect::<Vec<_>>();

        result.sort_unstable_by_key(|item| item[0]);

        result
    }
}

impl super::SummaryRanges for SummaryRanges {
    fn new() -> Self {
        Self::new()
    }

    fn add_num(&mut self, val: i32) {
        self.add_num(val);
    }

    fn get_intervals(&self) -> Vec<Vec<i32>> {
        self.get_intervals()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::SummaryRanges>();
    }
}

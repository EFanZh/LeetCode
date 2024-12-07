pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn insert_neighbor((scores, neighbors): &mut ([u32; 3], [u16; 3]), score: u32, neighbor: u16) {
        if score > scores[2] {
            let mut i = 2;

            while i != 0 {
                let next_i = i - 1;

                if score > scores[next_i] {
                    scores[i] = scores[next_i];
                    neighbors[i] = neighbors[next_i];
                    i = next_i;
                } else {
                    break;
                }
            }

            scores[i] = score;
            neighbors[i] = neighbor;
        }
    }

    pub fn maximum_score(scores: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let n = scores.len();
        let mut nodes = vec![([0_u32; 3], [0_u16; 3]); n].into_boxed_slice();

        let edges = edges
            .into_iter()
            .map(|edge| {
                let [from, to] = <[_; 2]>::map(edge.as_slice().try_into().ok().unwrap(), |x| x as u32 as usize);

                Self::insert_neighbor(&mut nodes[from], scores[to] as _, to as _);
                Self::insert_neighbor(&mut nodes[to], scores[from] as _, from as _);

                (from as u16, to as u16)
            })
            .collect::<Box<_>>();

        let mut result = -1;

        for &(from, to) in &*edges {
            let (from_usize, to_usize) = (usize::from(from), usize::from(to));
            let middle_score = scores[from_usize] + scores[to_usize];
            let (from_node, to_node) = (&nodes[from_usize], &nodes[to_usize]);

            for i in 0..3 {
                let from_neighbor_score = from_node.0[i] as i32;

                if from_neighbor_score == 0 {
                    break;
                }

                let from_neighbor = from_node.1[i];

                if from_neighbor != from && from_neighbor != to {
                    for j in 0..3 {
                        let to_neighbor_score = to_node.0[j] as i32;

                        if to_neighbor_score == 0 {
                            break;
                        }

                        let to_neighbor = to_node.1[j];

                        if to_neighbor != to && to_neighbor != from && to_neighbor != from_neighbor {
                            result = result.max(middle_score + from_neighbor_score + to_neighbor_score);
                        }
                    }
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_score(scores: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        Self::maximum_score(scores, edges)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

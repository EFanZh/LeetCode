pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZeroU8;

impl Solution {
    pub fn cat_mouse_game(graph: Vec<Vec<i32>>) -> i32 {
        const DRAW: u8 = 0;
        const MOUSE_WINS: u8 = 1;
        const CAT_WINS: u8 = 2;

        let n = graph.len();
        let mut states = vec![(0_u8, DRAW); (n - 1) * n * 2];

        let index =
            |cat, mouse, cat_first| 2 * n * (usize::from(cat) - 1) + 2 * usize::from(mouse) + usize::from(cat_first);

        // Fill node degrees.

        for (cat, cat_neighbors) in (1..).zip(&graph[1..]) {
            let cat_moves = cat_neighbors.iter().filter(|&&neighbor| neighbor != 0).count() as _;

            for (mouse, neighbors) in (0..).zip(&graph) {
                states[index(cat, mouse, false)].0 = neighbors.len() as _;
                states[index(cat, mouse, true)].0 = cat_moves;
            }
        }

        // Build queue.

        let mut queue = Vec::with_capacity(4 * (n - 1));

        for cat in 1..n as _ {
            // Mouse wins.

            states[index(cat, 0, false)].1 = MOUSE_WINS;
            states[index(cat, 0, true)].1 = MOUSE_WINS;

            // Cat wins.

            states[index(cat, cat, false)].1 = CAT_WINS;
            states[index(cat, cat, true)].1 = CAT_WINS;

            // Fill queue.

            queue.extend(&[(cat, 0, false), (cat, 0, true), (cat, cat, false), (cat, cat, true)]);
        }

        // Minimax.

        while let Some((cat, mouse, cat_move)) = queue.pop() {
            let outcome = states[index(cat, mouse, cat_move)].1;

            if cat_move {
                let prev_mouses = graph[usize::from(mouse)].iter().map(|&mouse| mouse as _);

                if outcome == MOUSE_WINS {
                    for prev_mouse in prev_mouses {
                        if let prev_outcome @ &mut DRAW = &mut states[index(cat, prev_mouse, false)].1 {
                            *prev_outcome = MOUSE_WINS;
                            queue.push((cat, prev_mouse, false));
                        }
                    }
                } else {
                    for prev_mouse in prev_mouses {
                        if let (prev_degree, prev_outcome @ DRAW) = &mut states[index(cat, prev_mouse, false)] {
                            *prev_degree -= 1;

                            if *prev_degree == 0 {
                                *prev_outcome = CAT_WINS;
                                queue.push((cat, prev_mouse, false));
                            }
                        }
                    }
                }
            } else {
                let prev_cats = graph[usize::from(cat)]
                    .iter()
                    .filter_map(|&cat| NonZeroU8::new(cat as _).map(NonZeroU8::get));

                if outcome == CAT_WINS {
                    for prev_cat in prev_cats {
                        if let prev_outcome @ &mut DRAW = &mut states[index(prev_cat, mouse, true)].1 {
                            *prev_outcome = CAT_WINS;
                            queue.push((prev_cat, mouse, true));
                        }
                    }
                } else {
                    for prev_cat in prev_cats {
                        if let (prev_degree, prev_outcome @ DRAW) = &mut states[index(prev_cat, mouse, true)] {
                            *prev_degree -= 1;

                            if *prev_degree == 0 {
                                *prev_outcome = MOUSE_WINS;
                                queue.push((prev_cat, mouse, true));
                            }
                        }
                    }
                }
            }
        }

        i32::from(states[index(2, 1, false)].1)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn cat_mouse_game(graph: Vec<Vec<i32>>) -> i32 {
        Self::cat_mouse_game(graph)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

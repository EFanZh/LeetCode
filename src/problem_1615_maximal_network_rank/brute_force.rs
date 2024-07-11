pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

struct BitSet(Box<[u8]>);

impl BitSet {
    fn new(n: usize) -> Self {
        Self(vec![0; (n + 7) / 8].into_boxed_slice())
    }

    fn get(&self, index: usize) -> bool {
        self.0[index / 8] & (1 << (index % 8)) != 0
    }

    fn set(&mut self, index: usize) {
        self.0[index / 8] |= 1 << (index % 8);
    }
}

impl Solution {
    pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as u32 as usize;
        let mut degrees = vec![0_u8; n].into_boxed_slice();
        let mut connected = BitSet::new(n * n);

        for road in roads {
            let [from, to]: [_; 2] = road.try_into().ok().unwrap();
            let from = from as u32 as usize;
            let to = to as u32 as usize;

            degrees[from] += 1;
            degrees[to] += 1;

            let (from, to) = if to < from { (to, from) } else { (from, to) };

            connected.set(n * from + to);
        }

        let mut result = 0;

        for (from, &from_degree) in degrees.iter().enumerate() {
            let base_index = n * from;
            let mut to = from;

            loop {
                to += 1;

                if let Some(&to_degree) = degrees.get(to) {
                    result = result.max(from_degree + to_degree - u8::from(connected.get(base_index + to)));
                } else {
                    break;
                }
            }
        }

        i32::from(result)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        Self::maximal_network_rank(n, roads)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

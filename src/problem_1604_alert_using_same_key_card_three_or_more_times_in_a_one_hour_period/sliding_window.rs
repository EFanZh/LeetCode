pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    pub fn alert_names(key_name: Vec<String>, key_time: Vec<String>) -> Vec<String> {
        let mut workers = HashMap::<_, Vec<_>>::new();

        for (name, time) in key_name.into_iter().zip(key_time) {
            let [h0, h1, _, m0, m1]: [_; 5] = time.as_bytes().try_into().ok().unwrap();
            let time = u16::from(h0) * 600 + u16::from(h1) * 60 + u16::from(m0) * 10 + u16::from(m1);

            workers
                .entry(name)
                .and_modify(|times| times.push(time))
                .or_insert_with(|| vec![time]);
        }

        let mut result = workers
            .into_iter()
            .filter_map(|(worker, mut times)| {
                times.sort_unstable();

                let mut start = 0;
                let mut count = 0;
                let mut i = 0;

                while let Some(&time) = times.get(i) {
                    count += 1;

                    let min_left_time = time - 60;

                    while times[start] < min_left_time {
                        start += 1;
                        count -= 1;
                    }

                    if count > 2 {
                        return Some(worker);
                    }

                    i += 1;
                }

                None
            })
            .collect::<Vec<_>>();

        result.sort_unstable();

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn alert_names(key_name: Vec<String>, key_time: Vec<String>) -> Vec<String> {
        Self::alert_names(key_name, key_time)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

pub struct TopVotedCandidate {
    leads: Vec<(u32, i32)>,
}

impl TopVotedCandidate {
    fn new(persons: Vec<i32>, times: Vec<i32>) -> Self {
        let n = persons.len();
        let mut iter = times.into_iter().map(|time| time as u32).zip(persons);
        let mut leads = Vec::with_capacity(n);
        let mut counts = vec![0_u32; n];
        let (first_time, first_person) = iter.next().unwrap();
        let mut max_count = 1;
        let mut max_person = first_person;

        counts[first_person as u32 as usize] = 1;
        leads.push((first_time, first_person));

        for (time, person) in iter {
            let count = &mut counts[person as u32 as usize];

            *count += 1;

            if *count >= max_count {
                max_count = *count;

                if person != max_person {
                    max_person = person;
                    leads.push((time, person));
                }
            }
        }

        Self { leads }
    }

    fn q(&self, t: i32) -> i32 {
        let i = self
            .leads
            .binary_search_by_key(&(t as _), |&(time, _)| time)
            .unwrap_or_else(|i| i - 1);

        self.leads[i].1
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::TopVotedCandidate for TopVotedCandidate {
    fn new(persons: Vec<i32>, times: Vec<i32>) -> Self {
        Self::new(persons, times)
    }

    fn q(&self, t: i32) -> i32 {
        self.q(t)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::TopVotedCandidate>();
    }
}

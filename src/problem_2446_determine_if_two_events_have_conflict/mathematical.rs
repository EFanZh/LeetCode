pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn parse_time(time: &str) -> u16 {
        let [h_0, h_1, _, m_0, m_1] = time.as_bytes().try_into().ok().unwrap();

        600 * u16::from(h_0) + 60 * u16::from(h_1) + 10 * u16::from(m_0) + u16::from(m_1)
    }

    fn parse_event(event: &[String]) -> (u16, u16) {
        let [start_time, end_time]: &[_; 2] = event.try_into().ok().unwrap();

        (Self::parse_time(start_time), Self::parse_time(end_time))
    }

    pub fn have_conflict(event1: Vec<String>, event2: Vec<String>) -> bool {
        let event1 = Self::parse_event(&event1);
        let event2 = Self::parse_event(&event2);

        event2.0 <= event1.1 && event1.0 <= event2.1
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn have_conflict(event1: Vec<String>, event2: Vec<String>) -> bool {
        Self::have_conflict(event1, event2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

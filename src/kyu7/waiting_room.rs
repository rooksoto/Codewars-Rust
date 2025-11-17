use crate::kyu::Kyu;
use crate::problem::Problem;

pub struct WaitingRoom;

impl WaitingRoom {
    fn last_chair(n: u32) -> u32 {
        n - 1
    }
}

impl Problem for WaitingRoom {
    fn problem_name(&self) -> &'static str {
        "Waiting room"
    }

    fn problem_description(&self) -> &'static str {
        r#"
        There's a waiting room with N chairs set in single row. Chairs are consecutively numbered from 1 to N. First is closest to the entrance (which is exit as well).

        For some reason people choose a chair in the following way

        Find a place as far from other people as possible
        Find a place as close to exit as possible
        All chairs must be occupied before the first person will be served

        So it looks like this for 10 chairs and 10 patients

        - Chairs	    1	2	3	4	5	6	7	8	9	10
        - Patients  	1	7	5	8	3	9	4	6	10	2
        Your task is to find last patient's chair's number.

        - Input: number of chairs N, an integer greater than 2.
        - Output: a positive integer, the last patient's chair number.
        "#
    }

    fn kyu(&self) -> Kyu {
        Kyu::Kyu7
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn last_chair_10() {
        assert_eq!(WaitingRoom::last_chair(10), 9);
    }
}

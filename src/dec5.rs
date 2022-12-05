pub mod supply_stacks;

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use crate::dec5;
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn supply_stacks_toppers_test() {
        assert_eq!(supply_stacks::supply_stacks(true), vec!['C', 'M', 'Z']);
        assert_eq!(supply_stacks::supply_stacks(false), vec!['B', 'S', 'D', 'M', 'Q', 'F', 'L', 'S', 'P']);
    }

    #[test]
    fn supply_stacks_toppers_part_2_test() {
        assert_eq!(supply_stacks::supply_stacks_part_two(true), vec!['M', 'C', 'D']);
        assert_eq!(supply_stacks::supply_stacks_part_two(false), vec!['P', 'G', 'S', 'Q', 'B', 'F', 'L', 'D', 'P']);
    }
}
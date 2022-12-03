pub mod rucksack_reorganization;

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use crate::dec3;
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_rucksack_part_one() {
        assert_eq!(rucksack_reorganization::rucksacks_part_one(true),157);
        assert_eq!(rucksack_reorganization::rucksacks_part_one(false),7763);
    }

    #[test]
    fn test_rucksack_part_two() {
        assert_eq!(rucksack_reorganization::rucksacks_part_two(true), 70);
        assert_eq!(rucksack_reorganization::rucksacks_part_two(false), 2569);
    }
}
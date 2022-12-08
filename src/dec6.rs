pub mod tuning_trouble;

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use crate::dec6;
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn tuning_trouble_part_one() {
        assert_eq!(tuning_trouble::tuning_trouble(true, 4), Some(11));
        assert_eq!(tuning_trouble::tuning_trouble(false, 4), Some(1140));
    }

    #[test]
    fn tuning_trouble_part_two() {
        assert_eq!(tuning_trouble::tuning_trouble(false, 14), Some(3495));
    }
}
pub mod sum_calories;

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use crate::dec1;
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn greatest_calorie_count_test() {
        assert_eq!(sum_calories::greatest_calorie_count(true), 24000);
        assert_eq!(sum_calories::greatest_calorie_count(false), 71924);
    }

    #[test]
    fn top_three_greatest_calorie_count_test() {
        assert_eq!(sum_calories::top_three(true), 45000);
        assert_eq!(sum_calories::top_three(false), 210406);
    }
}
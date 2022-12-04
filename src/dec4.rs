pub mod camp_cleanup;

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use crate::dec4;
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn number_of_included_sections_test() {
        assert_eq!(camp_cleanup::included_sections(true), 2);
        assert_eq!(camp_cleanup::included_sections(false), 507);
    }

    #[test]
    fn number_of_overlapping_sections_test() {
        assert_eq!(camp_cleanup::overlapping_sections(true), 4);
        assert_eq!(camp_cleanup::overlapping_sections(false), 897);
    }
}
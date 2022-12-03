pub mod rock_paper_scissors;

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use crate::dec2;
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_first_rock_paper_scissor() {
        assert_eq!(rock_paper_scissors::get_points_first_version(true), 15);
        assert_eq!(rock_paper_scissors::get_points_first_version(false), 11666);
        }

    #[test]
    fn test_second_rock_paper_scissor() {
        assert_eq!(rock_paper_scissors::get_points_second_version(true), 12);
        assert_eq!(rock_paper_scissors::get_points_second_version(false), 12767);
    }
}
mod treetop_tree_house;

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use crate::dec8;
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn treetop_tree_house_part_one() {
        assert_eq!(treetop_tree_house::visible_trees(true), 21);
        assert_eq!(treetop_tree_house::visible_trees(false), 1816);

    }

    #[test]
    fn treetop_tree_house_part_two() {
        assert_eq!(treetop_tree_house::best_scenic_score(true), 8);
        assert_eq!(treetop_tree_house::best_scenic_score(false), 383520);
    }
}
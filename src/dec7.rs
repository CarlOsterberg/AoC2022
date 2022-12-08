pub mod no_space_left_on_device;

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use crate::dec7;
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn tuning_trouble_part_one() {
        assert_eq!(no_space_left_on_device::no_space_left_on_device(true), 584+94853+24933642+48381165);
        //assert_eq!(no_space_left_on_device::no_space_left_on_device(false), 95437);
    }
}
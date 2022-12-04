use crate::get_data_as_string;

struct PairOfSections {
    first_section: (u32, u32),
    second_section: (u32, u32),
}

impl PairOfSections {
    fn new(unstuctured: &str) -> PairOfSections {
        let mut sections_as_str = unstuctured.split(",");

        let first_section_as_str = sections_as_str.next().unwrap();
        let mut sections_as_iter = first_section_as_str.split("-");
        let left: u32 = sections_as_iter.next().unwrap().parse::<u32>().unwrap();
        let right: u32 = sections_as_iter.next().unwrap().parse::<u32>().unwrap();
        let first_section = (left,right);

        let second_section_as_str = sections_as_str.next().unwrap();
        let mut sections_as_iter = second_section_as_str.split("-");
        let left: u32 = sections_as_iter.next().unwrap().parse::<u32>().unwrap();
        let right: u32 = sections_as_iter.next().unwrap().parse::<u32>().unwrap();
        let second_section = (left,right);

        PairOfSections {
            first_section,
            second_section,
        }
    }

    fn is_section_included(&self) -> bool {
        let a = self.first_section.0;
        let b = self.first_section.1;
        let c = self.second_section.0;
        let d = self.second_section.1;
        //right interval inside left interval
        if a <= c && b >= d {
            true
        }
        //left interval inside right interval
        else if c <= a && d >= b {
            true
        }
        else {
            false
        }
    }

    fn is_section_overlapping(&self) -> bool {
        if self.is_section_included() {
            return true;
        }
        let a = self.first_section.0;
        let b = self.first_section.1;
        let c = self.second_section.0;
        let d = self.second_section.1;
        if a <= c && b >= c || a <= d && b >= d {
            true
        }
        else if c <= a && c >= b || d <= a && d >= b {
            true
        }
        else {
            false
        }
    }
}

pub fn included_sections(is_example: bool) -> u32 {
    let unparsed = get_data_as_string(is_example, "dec4");
    let lines = unparsed.lines();
    let mut sections: Vec<PairOfSections> = Vec::new();
    for line in lines {
        sections.push(PairOfSections::new(line));
    }
    sections
        .iter()
        .map(|n|n.is_section_included())
        .fold(0,|acc, n| if n {acc + 1} else {acc} )
}

pub fn overlapping_sections(is_example: bool) -> u32 {
    let unparsed = get_data_as_string(is_example, "dec4");
    let lines = unparsed.lines();
    let mut sections: Vec<PairOfSections> = Vec::new();
    for line in lines {
        sections.push(PairOfSections::new(line));
    }
    sections
        .iter()
        .map(|n|n.is_section_overlapping())
        .fold(0,|acc, n| if n {acc + 1} else {acc} )
}
use crate::get_data_as_string;

struct Elf {
    calorie_list: Vec<u32>
}

struct Elves {
    all_elves: Vec<Elf>
}

impl Elf {
    fn new(calories: Vec<u32>) -> Elf {
        Elf {
            calorie_list: calories
        }
    }
    fn sum(&self) -> u32 {
        self.calorie_list.iter().sum()
    }
}

impl Elves {
    fn new(unparsed: String) -> Elves {
        let mut elves:Vec<Elf> = Vec::new();
        for elf_calories in unparsed.split("\n\n") {
            let mut calories:Vec<u32> = Vec::new();
            for line in elf_calories.lines() {
                calories.push(line.parse::<u32>().unwrap());
            }
            elves.push(Elf::new(calories));
        };
        Elves {
            all_elves: elves
        }
    }
    fn get_greatest_sum(&self) -> u32 {
        self.all_elves
            .iter()
            .map(|n|n.sum())
            .fold(0, |acc, n| if n > acc {n} else {acc})
    }

}

pub fn greatest_calorie_count(is_example: bool) -> u32 {
    let raw_data = get_data_as_string(is_example, "dec1");
    let elves = Elves::new(raw_data);
    elves.get_greatest_sum()
}

pub fn top_three(is_example: bool) -> u32 {
    let raw_data = get_data_as_string(is_example, "dec1");
    let elves = Elves::new(raw_data);
    let elves_calories = elves
        .all_elves
        .iter()
        .map(|n|n.sum());
    let mut first: u32 = 0;
    let mut second: u32 = 0;
    let mut third: u32 = 0;
    for elf in elves_calories {
        if elf > first {
            third = second;
            second = first;
            first = elf;
        }
        else if elf > second {
            third = second;
            second = elf;
        }
        else if elf > third {
            third = elf;
        }
    }
    first + second + third
}

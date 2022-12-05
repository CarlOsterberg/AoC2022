use crate::get_data_as_string;

#[derive(Debug)]
struct Crate {
    contents: char,
}

impl Crate {
    fn new(unparsed_crate: &str) -> Crate {
        let contents = unparsed_crate.chars().nth(0).unwrap();
        Crate {
            contents
        }
    }
}

#[derive(Debug)]
struct Stack {
    stack: Vec<Crate>,
}

impl Stack {
    fn new() -> Stack {
        let stack: Vec<Crate> = Vec::new();
        Stack {
            stack
        }
    }
    fn push(&mut self, a_crate: Crate) {
        self.stack.push(a_crate);
    }
    fn pop(&mut self) -> Crate {
        self.stack.pop().unwrap()
    }
    fn get_top(&self) -> char {
       self.stack.iter().last().unwrap().contents
    }
}

#[derive(Debug)]
struct Move {
    how_many: u32,
    from: u32,
    to: u32,
}

impl Move {
    fn new(unparsed_move: &str) -> Move {
        let mut parts = unparsed_move.split(" from ");
        let left = parts.next().unwrap();
        let how_many = left.split(" ").nth(1).unwrap();
        let right = parts.next().unwrap();
        let mut right_indexes = right.split(" to ");
        let from = right_indexes.next().unwrap();
        let to = right_indexes.next().unwrap();
        Move {
            how_many: how_many.parse::<u32>().unwrap(),
            from: from.parse::<u32>().unwrap() - 1,
            to: to.parse::<u32>().unwrap() - 1,
        }
    }
}

struct Stacks {
    stacks: Vec<Stack>,
    moves: Vec<Move>,
}

impl Stacks {
    fn new(unparsed: String) -> Stacks {
        let mut lines = unparsed.split("\n\n");
        let stacks = lines.next().unwrap();
        let moves = lines.next().unwrap();
        Stacks {
            stacks: Stacks::parse_stacks(stacks),
            moves: Stacks::parse_moves(moves),
        }
    }
    fn parse_stacks(unparsed_stacks: &str) -> Vec<Stack> {
        let mut lines = unparsed_stacks.lines().rev();
        let number_of_stacks = lines.next()
            .unwrap().split("   ").count();
        let mut stacks: Vec<Stack> = Vec::new();
        for _ in 0..number_of_stacks {
            stacks.push(Stack::new());
        }
        for line in lines {
            let string =
                if line.chars().next().unwrap() != ' ' {
                    (&line[1..line.len()]).to_string()
                } else {
                    line.to_string()
                };
            let unparsed_crates = string.split(" [");
            let mut index = 0;
            for unparsed_crate in unparsed_crates {
                if unparsed_crate.len() > 2 && unparsed_crate.chars().nth(0).unwrap() == ' ' {
                    let whitespace: Vec<&str> = unparsed_crate.matches(" ").collect();
                    index += whitespace.len() / 3;
                }
                else if unparsed_crate.len() > 2 {
                    stacks[index].stack.push(Crate::new(unparsed_crate));
                    let whitespace: Vec<&str> = unparsed_crate.matches(" ").collect();
                    index += whitespace.len() / 3 + 1;
                }
                else {
                    stacks[index].stack.push(Crate::new(unparsed_crate));
                    index += 1;
                }
            }
            println!();
        }
        stacks
    }

    fn parse_moves(unparsed_moves: &str) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        for unparsed_move in unparsed_moves.lines() {
            moves.push(Move::new(unparsed_move));
        }
        moves
    }

    fn perform_moves(&mut self) {
        for a_move in &self.moves {
            for _ in 0..a_move.how_many {
                let crate_to_move = self.stacks.get_mut(a_move.from as usize).unwrap().pop();
                self.stacks.get_mut(a_move.to as usize).unwrap().push(crate_to_move);
            }
        }
    }

    fn perform_moves_new(&mut self) {
        for a_move in &self.moves {
            let crate_stack = self.stacks.get_mut(a_move.from as usize).unwrap();
            let mut crates_to_move = crate_stack.stack.split_off(crate_stack.stack.len() - a_move.how_many as usize);
            let stack_to_move_to = self.stacks.get_mut(a_move.to as usize).unwrap();
            stack_to_move_to.stack.append(&mut crates_to_move);
        }
    }

    fn get_current_stack_tops(&self) -> Vec<char> {
        let mut chars: Vec<char> = Vec::new();
        for stack in &self.stacks {
            chars.push(stack.get_top())
        }
        chars
    }
}

pub fn supply_stacks(is_example: bool) -> Vec<char> {
    let unparsed_string = get_data_as_string(is_example, "dec5");
    let mut stacks = Stacks::new(unparsed_string);
    stacks.perform_moves();
    stacks.get_current_stack_tops()
}

pub fn supply_stacks_part_two(is_example: bool) -> Vec<char> {
    let unparsed_string = get_data_as_string(is_example, "dec5");
    let mut stacks = Stacks::new(unparsed_string);
    stacks.perform_moves_new();
    stacks.get_current_stack_tops()
}


use crate::get_data_as_string;

struct Datastream {
    stream: String
}

impl Datastream {
    fn new(stream: String) -> Datastream {
        Datastream {
            stream
        }
    }

    fn get_first_unique_sequence(&self, sequence_length: usize) -> Option<usize> {
        let mut current_seq: Vec<char> = Vec::new();
        for (i, char) in self.stream.chars().enumerate() {
            for (j, char_in_seq) in current_seq.clone().iter().enumerate() {
                if char == *char_in_seq {
                    let (_, new_seq) = current_seq.split_at_mut(j+1);
                    current_seq = new_seq.to_vec();
                }
            }
            current_seq.push(char);
            if current_seq.len() == sequence_length {
                return Some(i + 1)
            }
        }
        None
    }
}

pub fn tuning_trouble(is_example:bool, sequence_length: usize) -> Option<usize> {
    let stream = get_data_as_string(is_example, "dec6");
    let obj = Datastream::new(stream);
    obj.get_first_unique_sequence(sequence_length)
}
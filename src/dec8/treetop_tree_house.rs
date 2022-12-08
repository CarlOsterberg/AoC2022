use crate::get_data_as_string;


pub fn visible_trees(is_example: bool) -> usize {
    let string = get_data_as_string(is_example, "dec8");
    let mut grid:Vec<Vec<usize>> = Vec::new();
    for line in string.lines() {
        let chars = line.chars();
        let numbers: Vec<usize> = chars.into_iter()
            .map(|c|c.to_string().parse::<usize>().unwrap())
            .collect();
        grid.push(numbers);
    }
    let mut visible_trees = grid.len()*4 - 4;
    for i in 1..(grid.len()-1) {
        for j in 1..(grid.len()-1) {
            let size = *grid.get(i).unwrap().get(j).unwrap();
            //print!("coord to check ({}:{}){}\n", i,j,size);
            if  is_visible_in_direction(
                    grid.clone(), (i as i32, j as i32),
                    size, 1, 0) ||
                is_visible_in_direction(
                    grid.clone(), (i as i32, j as i32),
                    size, -1, 0) ||
                is_visible_in_direction(
                    grid.clone(), (i as i32, j as i32),
                    size, 0, -1) ||
                is_visible_in_direction(
                    grid.clone(), (i as i32, j as i32),
                    *grid.get(i).unwrap().get(j).unwrap(),
                    0, 1)
            {
                //println!("yes {} {}", i, j);
                visible_trees += 1;
            }
        }
    }
    return visible_trees;
}

fn is_visible_in_direction(grid: Vec<Vec<usize>>, coordinate: (i32,i32), size: usize, i:i32, j:i32) -> bool {
    match grid.get((coordinate.0 + i) as usize) {
        Some(vec) => {
            match vec.get((coordinate.1 + j) as usize) {
                None => {true}
                Some(other_size) => {
                    //print!(" checking vs ({}:{}){}", coordinate.0,coordinate.1,other_size);
                    if other_size >= &size {
                        //println!(" failed!");
                        false
                    } else {
                        is_visible_in_direction(grid,(coordinate.0 + i, coordinate.1 + j),size, i, j)
                    }
                }
            }
        }
        None => {
            true
        }
    }
}

pub fn best_scenic_score(is_example: bool) -> usize {
    let string = get_data_as_string(is_example, "dec8");
    let mut grid:Vec<Vec<usize>> = Vec::new();
    for line in string.lines() {
        let chars = line.chars();
        let numbers: Vec<usize> = chars.into_iter()
            .map(|c|c.to_string().parse::<usize>().unwrap())
            .collect();
        grid.push(numbers);
    }
    let mut highest_score = 0;
    for i in 1..(grid.len()-1) {
        for j in 1..(grid.len()-1) {
            let size = *grid.get(i).unwrap().get(j).unwrap();
            let contender =
            get_scenic_score(grid.clone(),(i as i32,j as i32),size, 0, 1) *
            get_scenic_score(grid.clone(),(i as i32,j as i32),size, 0, -1) *
            get_scenic_score(grid.clone(),(i as i32,j as i32),size, 1, 0) *
            get_scenic_score(grid.clone(),(i as i32,j as i32),size, -1, 0);
            if contender > highest_score {
                highest_score = contender;
            }
        }
    }
    return highest_score;
}

fn get_scenic_score(grid: Vec<Vec<usize>>, coordinate: (i32,i32), size: usize, i:i32, j:i32) -> usize {
    match grid.get((coordinate.0 + i) as usize) {
        Some(vec) => {
            match vec.get((coordinate.1 + j) as usize) {
                None => {0}
                Some(other_size) => {
                    if other_size >= &size {
                        1
                    } else {
                        1 + get_scenic_score(grid,(coordinate.0 + i, coordinate.1 + j),size, i, j)
                    }
                }
            }
        }
        None => {0}
    }
}
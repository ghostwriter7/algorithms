

fn main() {
    let floors = vec![0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1];
    let first_floor_it_breaks = get_first_floor_crystal_ball_breaks(floors);
    println!("The first floor is {first_floor_it_breaks}");
}

fn get_first_floor_crystal_ball_breaks(floors: Vec<i32>) -> i32 {
    let length = (floors.len() - 1) as i32;
    let jump = (length as f64).sqrt().floor() as i32;
    let mut index = jump;

    while index < length {
        if floors[index as usize] == 1 {
            break;
        }

        index = if index + jump < length { index + jump } else { length };
    }

    let prev_index = index - jump;

    for value in prev_index..=index {
        if floors.get(value as usize).unwrap() == &1 {
            return value;
        }
    }

    return -1;
}
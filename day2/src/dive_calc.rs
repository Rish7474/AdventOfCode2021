use std::fs;

pub fn calculate_dive_position(file_path: &str) -> i32 {
    let file_content = fs::read_to_string(file_path).unwrap();
    let dive_cmds: Vec<&str> = file_content.lines().collect::<Vec<&str>>();
    let mut x_pos: i32 = 0;
    let mut y_pos: i32 = 0;

    for dive_cmd in &dive_cmds[0..] {
        let (direction, delta) = dive_cmd.split_once(" ").unwrap();
        let delta_num = delta.parse::<i32>().unwrap();
        match direction {
            "forward" => x_pos += delta_num,
            "down" => y_pos += delta_num,
            "up" => y_pos -= delta_num,
            _ => {}
        }
    }

    return x_pos * y_pos;
}

pub fn calculate_dive_position_with_aim(file_path: &str) -> i32 {
    let file_content = fs::read_to_string(file_path).unwrap();
    let dive_cmds: Vec<&str> = file_content.lines().collect::<Vec<&str>>();
    let mut x_pos: i32 = 0;
    let mut y_pos: i32 = 0;
    let mut aim: i32 = 0;

    for dive_cmd in &dive_cmds[0..] {
        let (direction, delta) = dive_cmd.split_once(" ").unwrap();
        let delta_num = delta.parse::<i32>().unwrap();
        match direction {
            "forward" => {
                x_pos += delta_num;
                y_pos += delta_num * aim
            }
            "down" => aim += delta_num,
            "up" => aim -= delta_num,
            _ => {}
        }
    }

    return x_pos * y_pos;
}

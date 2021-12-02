use std::env;
use std::fs;

pub fn calculate_sweep(input_file: &str) {
    let data = fs::read_to_string(input_file).expect("File could not be read");
    let data_itr = data.lines();
    let data_vec = data_itr.collect::<Vec<&str>>();

    let mut prev_num: i32 = data_vec[0].parse::<i32>().unwrap();
    let mut sweep_counter: i32 = 0;

    for i in 1..data_vec.len() {
        let cur_num = data_vec[i].parse::<i32>().unwrap();
        if cur_num > prev_num {
            sweep_counter += 1;
        }
        prev_num = cur_num;
    }

    println!("Total sweep counts: {}", sweep_counter);
}

pub fn calculate_sweep_three(input_file: &str) {
    let data = fs::read_to_string(input_file).expect("File could not be read");
    let data_itr = data.lines();
    let data_vec = data_itr.collect::<Vec<&str>>();

    let mut prev_sum: i32 = data_vec[0].parse::<i32>().unwrap()
        + data_vec[1].parse::<i32>().unwrap()
        + data_vec[2].parse::<i32>().unwrap();
    let mut sweep_counter: i32 = 0;

    for i in 3..data_vec.len() {
        let cur_sum = prev_sum + data_vec[i].parse::<i32>().unwrap()
            - data_vec[i - 3].parse::<i32>().unwrap();
        if cur_sum > prev_sum {
            sweep_counter += 1;
        }
        prev_sum = cur_sum;
    }

    println!("Total sweep counts: {}", sweep_counter);
}

mod dive_calc;

fn main() {
    let mut dive_pos = dive_calc::calculate_dive_position("../input_data.txt");
    println!("Dive Position: {}", dive_pos);
    dive_pos = dive_calc::calculate_dive_position_with_aim("../input_data.txt");
    println!("Dive Posistion with Aim: {}", dive_pos);
}

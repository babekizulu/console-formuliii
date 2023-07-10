use std::io::stdin;
fn main() {
    const FORMULAE: [&str; 2] = ["acceleration", "velocity"];
    let mut user_selection = String::new();
    let calc_prompt = "Enter the values for the following variables";
    let err_msg: &str = "Error: failed to read line";
    let err_msg_2: &str = "Error: failed to parse string into i8, please enter a valid number";
    println!("Welcome to FORMULiii");
    println!(
    "
    Select a calculator (select by number):
    1. {}
    2. {}
    ", FORMULAE[0], FORMULAE[1]);
    stdin().read_line(&mut user_selection).expect(err_msg);
    if user_selection.trim() == "1" {
        let mut variable_1 = String::new();
        let mut variable_2 = String::new();
        println!("Acceleration Calculator");
        println!("{calc_prompt}");
        println!("Displacement:");
        stdin().read_line(&mut variable_1).expect(err_msg);
        println!("Time:");
        stdin().read_line(&mut variable_2).expect(err_msg);
        let d: f64 = variable_1.trim().parse().expect(err_msg_2);
        let t: f64 = variable_2.trim().parse().expect(err_msg_2);
        acceleration(d, t);
    }
    if user_selection.trim() == "2" {
        let mut variable_1 = String::new();
        let mut variable_2 = String::new();
        println!("Velocity Calculator");
        println!("{calc_prompt}");
        println!("Displacement:");
        stdin().read_line(&mut variable_1).expect(err_msg);
        println!("Time:");
        stdin().read_line(&mut variable_2).expect(err_msg);
        let d: f64 = variable_1.trim().parse().expect(err_msg_2);
        let t: f64 = variable_2.trim().parse().expect(err_msg_2);
        velocity(d, t);
    }
}

fn acceleration(displacement: f64, time: f64) {
    let solution: f64 = displacement/time;
    let solution = solution.powi(2);
    println!("Acceleration: {solution}");
}

fn velocity(displacement: f64, time: f64) {
    let solution: f64 = displacement/time;
    println!("Velocity: {solution}");
}

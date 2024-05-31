use std::fs;

//First puzzel
pub fn nr_of_increasing_measurements() -> i32{
    let depth_readings: String = fs::read_to_string("day1_depth_readings")
        .expect("Something went wrong reading the file");
    let readings = depth_readings.lines();

    let mut prev_reading: i32 = 0;
    let mut increasing_counter: i32 = -1;

    for reading in readings {
        let integer_reading = reading.parse::<i32>().unwrap();
        if integer_reading > prev_reading {
            increasing_counter += 1;
        }
        prev_reading = integer_reading;
    }
    
    println!("{}", increasing_counter);
    increasing_counter
}

pub fn nr_of_increasing_measurements_2nd() -> i32{
    let depth_readings: String = fs::read_to_string("day1_depth_readings")
        .expect("Something went wrong reading the file");
    let readings:Vec<&str> = depth_readings.lines().collect();

    let mut prev_coupled_sum: i32 = 0;
    let mut increasing_counter: i32 = -1;
    let mut coupled_sum: i32;
    
    for i in 0..readings.len() {
        if i >= readings.len()-2 { break; }
        coupled_sum = readings[i].parse::<i32>().unwrap();
        for j in (i+1)..(i+3) {
            coupled_sum += readings[j].parse::<i32>().unwrap();
        }
        if coupled_sum > prev_coupled_sum {
            increasing_counter += 1;
        }
        prev_coupled_sum = coupled_sum;
    }
    
    println!("{}", increasing_counter);
    increasing_counter
}


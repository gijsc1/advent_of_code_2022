use std::fs;
use std::io::{BufRead, BufReader};

///A terrible but quick way to compare the current value with the three known biggest values.
/// If the current value is bigger, it replaces the corresponding value and pushes any other values further down the list.
/// Also updates the current and max elf values to keep track of which elf has the most food,
/// even though that is apparantly not required for the solution.
/// Also also sets the current value back to 0, and increments the current elf by 1 regardless of comparison outcome.
fn compare_and_update(current_val:&mut i32,max_val:&mut i32, second_max: &mut i32, third_max: &mut i32, current_elf:&mut i32, max_elf:&mut i32){
    if current_val>=max_val {
        *third_max = *second_max;
        *second_max = *max_val;
        *max_elf = *current_elf;
        *max_val = *current_val;
    }
    else if  current_val >= second_max {
        *third_max = *second_max;
        *second_max = *current_val;
    } else if current_val > third_max {
        *third_max = *current_val;
    }
    *current_val = 0;
    *current_elf+=1;
}

/// Finds the three biggest food supplies in a list of food items carried by elfs.
/// Also doubles as My First Rust Program, so prepare for suboptimal solutions.
fn main() {
    let file = fs::File::open("input.txt").expect("Error while reading file");
    let reader = BufReader::new(file);
    let lines = reader.lines();
    // This is stupid, but I am hungry and it will work.
    let mut current = 0;
    let mut elf = 1;
    let mut max_val = 0;
    let mut second_max = 0;
    let mut third_max = 0;
    let mut max_elf = 0;
    for line in lines {
        let line = line.expect("Error while reading line");
        if line == "" {
            compare_and_update(&mut current,&mut max_val, &mut second_max, &mut third_max, &mut elf,&mut max_elf);
            continue
        }
        current += line.parse::<i32>().expect("Failure parsing line to int");
        // println!("output: {:#?}", line);
    }
    compare_and_update(&mut current,&mut max_val, &mut second_max, &mut third_max, &mut elf,&mut max_elf);
    println!("biggest elf: {} with value {}. runner up: {}, third: {}, total: {}",max_elf,max_val,second_max,third_max,max_val+second_max+third_max);


}

use std::str::FromStr;
use crate::diag_iterator::DiagIterator;
use crate::file_io::get_lines;
use crate::sensor::{manhatten_dist, Sensor};

#[path="../../shared_code/file_io.rs"]
mod file_io;
#[path="../../shared_code/error.rs"]
mod error;
mod map;
mod sensor;
mod layout;
mod diag_iterator;

fn main() {
    let sensor_iter = get_lines("input.txt")
        .map(|l|Sensor::from_str(l.as_str()).unwrap());
    let sensors:Vec<Sensor> = sensor_iter.collect();


    // answer1(&sensors);
    let max_x = 4000000;

     'outerloop: for sensor in sensors.iter(){
         'coordloop: for bordercoord in DiagIterator::new(sensor.get_location(),sensor.get_distance()+1){
             if bordercoord.0>max_x || bordercoord.1 > max_x || bordercoord.0<0 || bordercoord.1<0{
                 continue
             }
             // if bordercoord.0==14 {
             //     println!("debug: now looking at {},{}",bordercoord.0,bordercoord.1);
             //
             // }

             for othersensor in sensors.iter(){
                if !othersensor.can_contain_beacon(bordercoord){
                    // if bordercoord.0==14 && bordercoord.1==11 {
                    //     println!("debug:{},{} is covered by {}", bordercoord.0, bordercoord.1, othersensor);
                    //     println!("debug:sensor distance:{}, sensor-point distance:{}",othersensor.get_distance(),manhatten_dist(othersensor.get_location(),bordercoord));
                    //     println!("doublecheck contains:{}",othersensor.debug_can_contain_beacon(bordercoord));
                    // }
                    continue 'coordloop;
                }
            }
             let frequency:u128 = (bordercoord.0 as u128)*4000000 +bordercoord.1 as u128;
             println!("answer2: coord: {},{}. freq:{}",bordercoord.0,bordercoord.1,frequency);
             break 'outerloop;
        }
    }
    println!("Hello, world!");
}

fn answer1(sensors:&Vec<Sensor>){
    let max_x = sensors.iter().map(|s|s.get_location().0+s.get_distance()as i32).max().unwrap();
    let min_x = sensors.iter().map(|s|s.get_location().0-s.get_distance() as i32).min().unwrap();
    println!("width: {}. from {} to {}",max_x-min_x,min_x,max_x);

    let mut counter  = 0;
    let y  = 2000000;
    for x in min_x..=max_x{
        // println!("debug: now checking {},{}",x,y);
        for sensor in sensors.iter() {
            if sensor.cannot_contain_beacon((x, y)){
                // println!("debug: found a match with {} with distance:{}",sensor,sensor.get_distance());
                counter+=1;
                break;
            }
        }
    }
    println!("answer1:{counter}");
}

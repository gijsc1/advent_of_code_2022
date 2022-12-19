use std::str::FromStr;
use std::time::SystemTime;
use crate::factory::Factory;
use crate::file_io::get_lines;
use crate::types::{Geode, PassState, State, Time};

#[path="../../shared_code/file_io.rs"]
mod file_io;
#[path="../../shared_code/error.rs"]
mod error;
mod parsing;
mod factory;
mod types;

const DOP2:bool = true;
const STARTINGTIME:Time = if DOP2 {32} else {24};
const DODEBUG:bool = false;
const TIMELOOKBACK:usize = 10;

//times+-100ms
//current p1 time: 77ms



fn main() {
    let factories = get_lines("input.txt").map(|l|Factory::from_str(&l).unwrap());
    let mut sum = if DOP2 {1} else {0};
    let start = SystemTime::now();
    for factory in factories{
        if DOP2 && factory.name>3{
            continue
        }
        let result = get_max_geodes(&factory,&mut get_starting_state(), (false,false,false),STARTINGTIME);
        if DOP2{
            println!("factory {} produced {} geodes",factory.name,result,);
            sum*=result;
        } else {
            let grade = result*factory.name;
            println!("factory {} produced {} geodes for grade {}",factory.name,result,grade);
            sum+=grade;
        }

    }
    println!("answer:{}",sum);
    let end = SystemTime::now();
    let since_the_epoch = end
        .duration_since(start)
        .expect("Time went backwards");
    println!("runtime: {}ms", since_the_epoch.as_millis());
    println!("Hello, world!");
}

fn get_max_geodes(factory: &Factory, state:&mut State,(passore,passclay,passobs):PassState, time:Time) ->Geode{
    let (_,_,_,_,_,_,geoderobot,geode) = state;
    if time==1{
        return *geode+*geoderobot;
    }
    let mut max = *geode;
    let can_ore = factory.can_build_orerobot(state);
    let can_clay = factory.can_build_clayrobot(state);
    let can_obs = factory.can_build_obsrobot(state);
    let can_geo = factory.can_build_geoderobot(state);

    // If we can build a geode robot, this is always the best choice.
    if can_geo{
        if DODEBUG&& time>=STARTINGTIME-TIMELOOKBACK{
            println!("now working on T={} in branch geoderobot",time)
        }
        mine_resources(state);
        factory.build_geoderobot(state);
        let result = max.max(get_max_geodes(factory,state,(false,false,false),time-1));
        factory.destruct_geoderobot(state);
        unmine_resources(state);
        return result;
    }

    if can_obs && !passobs && !factory.enough_obsrobot(state)
        // && factory.obs_still_usefull(state,time)
    {
        if DODEBUG && time>=STARTINGTIME-TIMELOOKBACK{
            println!("now working on T={} in branch obsrobot",time)
        }
        mine_resources(state);
        factory.build_obsrobot(state);
        max = max.max(get_max_geodes(factory,state,(false,false,false),time-1));
        factory.destruct_obsrobot(state);
        unmine_resources(state);
    }


    if can_clay && !passclay && !factory.enough_clayrobot(state){
        if DODEBUG && time>=STARTINGTIME-TIMELOOKBACK{
            println!("now working on T={} in branch clayrobot",time)
        }
        mine_resources(state);
        factory.build_clayrobot(state);
        max = max.max(get_max_geodes(factory,state,(false,false,false),time-1));
        factory.destruct_clayrobot(state);
        unmine_resources(state);
    }

    if can_ore && !passore && !factory.enough_orebot(state){
        if DODEBUG && time>=STARTINGTIME-TIMELOOKBACK{
            println!("now working on T={} in branch orerobot",time)
        }
        mine_resources(state);
        factory.build_orerobot(state);
        max = max.max(get_max_geodes(factory,state,(false,false,false),time-1));
        factory.destruct_orerobot(state);
        unmine_resources(state);
    }

    if DODEBUG && time>=STARTINGTIME-TIMELOOKBACK{
        println!("now working on T={} in branch no action",time)
    }
    mine_resources(state);
    //This implies that in the next iterations, something should only be bought if it was not possible now.
    max = max.max(get_max_geodes(factory,state,(can_ore,can_clay,can_obs),time-1));
    unmine_resources(state);

    return max;
}

fn get_starting_state()->State{
    (1,0,0,0,0,0,0,0)
}

pub fn mine_resources((orerobot,ore,clayrobot,clay,obsrobot,obs,geoderobot,geode):&mut State) {
    *ore+=*orerobot;
    *clay+=*clayrobot;
    *obs+=*obsrobot;
    *geode+=*geoderobot;
}

pub fn unmine_resources((orerobot,ore,clayrobot,clay,obsrobot,obs,geoderobot,geode):&mut State) {
    *ore-=*orerobot;
    *clay-=*clayrobot;
    *obs-=*obsrobot;
    *geode-=*geoderobot;
}


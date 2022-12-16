use std::collections::{ HashMap};
use std::hash::Hash;
use std::str::FromStr;
use crate::file_io::get_lines;
use crate::parsing::{Parsable};
use crate::pipe::Pipe;

#[path="../../shared_code/file_io.rs"]
mod file_io;
#[path="../../shared_code/error.rs"]
mod error;
mod pipe;
mod parsing;
// mod cheat_solution;

/// "improving the parsing just a bit" turned out to be so time consuming that i doubt the challenge itself will ever get finished. But who nows.
/// The parsing is nice and chainable now though.
///
type ValveState = usize;
type ValveId = usize;
type Time = usize;
type Key = ((ValveId,ValveId),ValveState,(Time,Time));
type Presure = usize;

const TEST:bool = false;

const VALVECOUNT:usize = if TEST{10}else{60};
const MAX_DIST:usize = usize::MAX/2;

fn main() {
    // let s = "Valve BT has flow rate=0; tunnels lead to valves EZ, TO";
    // testfn(s).expect("testfun failed");
    let pipes:Vec<Pipe> = get_lines(if TEST {"testinput.txt"}else{"input.txt"}).map(|l|pipe::Pipe::from_str(l.as_str()).unwrap()).collect();
    let mut name_map :HashMap<pipe::Pipeid,ValveId> = HashMap::with_capacity(VALVECOUNT);
    let mut namegen:ValveId = 0;
    for pipe  in pipes.iter() {
        name_map.insert(pipe.id.clone(), namegen);
        namegen+=1;
    }
    let mut direct_connections = HashMap::with_capacity(VALVECOUNT);
    let mut flow_map = [0;VALVECOUNT];
    for pipe in pipes.iter(){
        let name = name_map[&pipe.id];
        flow_map[name] = pipe.flowrate;
        let mut convec = Vec::with_capacity(pipe.connections.len());
        for connection in pipe.connections.iter(){
            convec.push(name_map[connection]);
        }
        direct_connections.insert(name,convec);
    }
    let distance_map = floyd_warshall(&direct_connections);
    let mut resultcache = HashMap::<Key,Presure>::new();
    let starting_loc = *name_map.get("AA").unwrap();
    let answer1 = calc_pressure(&distance_map,&mut resultcache,&flow_map,0,(starting_loc,starting_loc),(26,26));

    println!("answer1:{answer1}");
}


fn floyd_warshall(edges:&HashMap<ValveId,Vec<ValveId>>)->[[usize;VALVECOUNT];VALVECOUNT]{
    let mut result = [[MAX_DIST;VALVECOUNT];VALVECOUNT];
    for (source,targets) in edges.iter(){
        for target in targets.iter(){
            result[*source][*target] = 1;
            result[*target][*source] = 1;
        }
    }
    for node in 0..VALVECOUNT{
        result[node][node] = 0;
    }
    for k in 0..VALVECOUNT{
        for i in 0..VALVECOUNT{
            for j in 0..VALVECOUNT{
                if result[i][j] > result[i][k] + result[k][j]{
                    result[i][j] = result[i][k] + result[k][j];
                }
            }
        }
    }
    result

}

fn calc_state(state:ValveState,loc:ValveId)->ValveState{
    // assert!(valve_is_off(state,loc));
    let res = state | (1<<loc);
    // assert!(!valve_is_off(res,loc));
    res

}

fn valve_is_off(state:ValveState,loc:ValveId)->bool{
    state & (1<<loc) == 0
}

fn calc_pressure(distances:&[[usize;VALVECOUNT];VALVECOUNT],
                     resultcache: & mut HashMap<Key,Presure>,
                     flowrates:& [usize;VALVECOUNT],
                     mut state: ValveState,
                     location:(ValveId,ValveId),
                     mut time:(Time,Time)) -> Presure{
    if time.0 ==0 && time.1==0{
        return 0;
    }
    let key = (location,state,time);
    if let Some(val) = resultcache.get(&key){
        return *val;
    }
    let mut self_flow = 0;
    if flowrates[location.0]>0 && valve_is_off(state,location.0) {
        time.0-=1;
        self_flow = time.0*flowrates[location.0];
        state = calc_state(state,location.0);
    }
    if flowrates[location.1]>0 && valve_is_off(state,location.1) {
        time.1-=1;
        self_flow = time.1*flowrates[location.1];
        state = calc_state(state,location.1);
    }

    let mut max = 0;
    // let mut best_target = 0;
    for (next_loc,next_dist) in distances[location.0].iter().enumerate().filter(|(loc,_)|flowrates[*loc]>0&&valve_is_off(state,*loc)){
        // println!("debug: now looking at {next_loc}");
        if *next_dist >= time.0{
            continue;
        }
        let sub_res = calc_pressure(distances,resultcache,flowrates,state,(next_loc,location.1),(time.0-next_dist,time.1));
        max = max.max(sub_res);
        // best_target=next_loc;
    }
    for (next_loc,next_dist) in distances[location.1].iter().enumerate().filter(|(loc,_)|flowrates[*loc]>0&&valve_is_off(state,*loc)){
        // println!("debug: now looking at {next_loc}");
        if *next_dist >= time.1{
            continue;
        }
        let sub_res = calc_pressure(distances,resultcache,flowrates,state,(location.0,next_loc),(time.0,time.1-next_dist));
        max = max.max(sub_res);
        // best_target=next_loc;
    }
    let result = max+self_flow;
    resultcache.insert(key,result);

    // println!("pipe {location} at time {time} has value {} by going to {best_target}",max+self_flow);
    return result;
}



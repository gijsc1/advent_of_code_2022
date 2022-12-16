use std::collections::{BTreeSet, HashMap, HashSet};
use std::hash::Hash;
use std::panic::Location;
use std::str::FromStr;
use crate::error::Error;
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

///Disclaimer, I had no idea on how to solve this problem, as it seems trying all possible paths is not possible.
///
/// "improving the parsing just a bit" turned out to be so time consuming that i doubt the challenge itself will ever get finished. But who nows.
/// The parsing is nice and chainable now though.
///
/// Due to time constraints this solution is now heavily based on an existing one found at:
/// https://topaz.github.io/paste/#XQAAAQCIFQAAAAAAAAA6nMjJFHMADebh9lMSAXn5c0lZw0XzLjIVxATQJaSMlgO28y8f4pRoeBLZvpxZUzHdAR5MG3hfw+IX69Uj7C/GdPw57WBodpSh9IU7fS8Msh+mD9ItMwylVdBKc3+hiPo4d2mfwqLwJ3ap1h9MUPedtax/Z/8pKR9W8cruX6TIztk2MUuA+3bBL10z0Tmufdd1SYYN/UtlOp5kURUa5xTUJwMnv+HkQKRwbKa++1MI0z4kN9NhqtCnq9Qbco53UWlgQqoEGhy8EJKvX2LnOGT03YJ1dHLM3zbQpvjYG5LqvBl1ofOEMro24Ms40a/ObvH3chZDbivq+L2IFjOC49DMU+2nJH+8gAY56c6gFajh/ZhL2pyC3MUv2aEODHzgQHSl2xa+UtZoTS3x0V1s4JntUzuGpv3PLdOJQGzFscdTxbFSRdlfiMoOULSl0/LUc+QZ1ID9g+hI2c7Hhd9feBrXGlt+0TkvGTcAPz0yTnrN2xnWv6h0fxwlGXidqvj3WAYf5Ed7Sy65XSiixjC7Q3bedc6HXPguwSX3jtHJlV2LoLaTk6hhWVmOwQJZ/VVsuwhqn2VjRG0MTOtsLZBLCsEC2u+8UEdYMFJ47/McaBXdqlldGBOzdz6HnpxLZlEzzMFjHdQJykXTzI7dlM3Q9EGSsAyRpj5+SRk0zd4ad0bqtwJmjWSC/rYfA8r/Ox7g6jaGBc3+wYhtPWgQKDzt3uj2unGbUkPbLtXeBwd+oqstcIjLgOG5WXRcvnOLr60jrTsLn09aLP9lez+R/inRj1/eLXmyQOj8lPNzcLCEBjrJvB2W1dh+IeZFWqf+DVs272e57Xy9VeX1hoNPe2I6lwCKoYxMp5qV69HiqAregl2obrKD2dYrFWhe0lPBm6rhqSgw5zI4nMyk7ZdhcliM1qKKPlJ29YCH2HJoKt3qTtTGUmoh3dVPOIx0w6yB03Xo2xz7ogYHOUcWKbsqd947m8B07fbr8ZUxyalA7Q2sPWwQNviYSYd03xDIhvfWa5vBuQz4Sbgk/2plhM0/AhXN44GUM2haupPhnrBxtVWyrlttOTrSjMm8WadO/iafe9wr2zYQtQ5qxRaJR/tb4eGE0uSTEJ0AUl6O4nl71xNv5PgYM/jSuz3ZRcOxsbtlaykyceCy4YLkEvaOtiJV1SPfIgXDTjAzvnB1aov3TReHVKwz0wTJaiuBISyXQBAY8lw97/B9PHWjBakVZmO/Z9r3J+XxfIS6aJKm7VworIcFm4k7a5ninLIZN14uKx9rwf/ZzXt4HPQBtaAPkJryuRlVSTL6HUjEgIXXD9W6zcfIlNOqJ0nAqS9wbbdNYPqT4BgIWMFgEvvzSnYow5eIA1ZwRC+dxDnJ6Zbls0jGFnlno8AeD4Fpux3xE5KSYiSsRWqB8ZyZ5ohmI5bbJ6zmc2/OZErjPxQUxi1dRzm9KKHaK+6tOz25U3pac/dr86MZTXuA3Cdgs6yPKf7MAFWOy5+dixnU26cC2mEAowSIk0W8Qsan83e5uY3C5S93+Pzk3doROjZau83A4n8rDsyzXp2cGTtuE6VURrwQAleQwISy2nv3vUMR1WFV6A3kOM0/1jYDET3hBc5D70BvuM4w8IpcgHlb7tU2GxjUgDpeaQgxlqw+8zGBtIwW3ag1Pes3rXn3waNe/gJBfCHCOMmtWnvsn0IUQ7gu3VHwhrc87xzeEQjhE9tcJBSgsNxijEWedx0rk3FwRP42sfuB8KsChbmUtdCBvg6cJ8AKQ7M6+3MTIiraYyvk2FZOk85CjsrGZsFt+xwjKsfE05s6zMyAUb/1Prsw1MsyLYoU7wYqMD2W1qobHQes+OrXyARtdUmvMlHU9wky1wVBVUlyfSUVv9Ouy8sZ27ce05LjiLc+MeetctEPxjnevOD3/WzQQBg47XAXUSS8Av3//CkLRw==
type ValveState = usize;
type ValveId = usize;
type Time = usize;
type Key = (ValveId,ValveState,Time);
type Presure = usize;

const TEST:bool = true;
const P2:bool = true;

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
    // for (node,targets) in distance_map.iter().enumerate(){
    //     for (target,distance) in targets.iter().enumerate(){
    //         println!("distance from {node} to {target} is {distance}");
    //     }
    // }
    // println!("distances:\n {:?}",distance_map);
    let mut resultcache = HashMap::<Key,Presure>::new();
    let answer1 = calc_pressure(&distance_map,&mut resultcache,&flow_map,0,*name_map.get("AA").unwrap(),if P2 {26}else{30},P2);

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
                     location:ValveId,
                     mut time:Time,
                     elephant_exists:bool) -> Presure{
    if time ==0{
        if elephant_exists{
            let mut newchache = HashMap::new();
            return calc_pressure(distances,&mut newchache,flowrates,state,0,26,false);
        }else {
            return 0;
        }
    }
    let key = (location,state,time);
    if let Some(val) = resultcache.get(&key){
        return *val;
    }
    let mut self_flow = 0;
    if flowrates[location]>0 {
        time-=1;
        self_flow = time*flowrates[location];
        state = calc_state(state,location);
    }
    let mut max = 0;
    let mut best_target = 0;
    for (next_loc,next_dist) in distances[location].iter().enumerate().filter(|(loc,_)|flowrates[*loc]>0&&valve_is_off(state,*loc)){
        // println!("debug: now looking at {next_loc}");
        if *next_dist > time{
            continue;
        }
        let sub_res = calc_pressure(distances,resultcache,flowrates,state,next_loc,time-next_dist,elephant_exists);
        max = max.max(sub_res);
        best_target=next_loc;
    }
    let result = max+self_flow;
    resultcache.insert(key,result);

    // println!("pipe {location} at time {time} has value {} by going to {best_target}",max+self_flow);
    if max==0 && elephant_exists{
        let mut newchache = HashMap::new();
        return calc_pressure(distances,&mut newchache,flowrates,state,0,26,false)+result;
    }else {
        return result;
    }
}



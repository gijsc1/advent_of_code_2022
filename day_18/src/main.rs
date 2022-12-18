use std::collections::{HashMap, HashSet};
use std::thread;
use crate::DropStatus::{ENCLOSED, LAVA, UNENCLOSED};
use crate::file_io::get_lines;

#[path="../../shared_code/file_io.rs"]
mod file_io;
#[path="../../shared_code/error.rs"]
mod error;

type Coord = isize;
type XCoord = Coord;
type YCoord = Coord;
type ZCoord = Coord;
type Drop = (XCoord,YCoord,ZCoord);
// const MAX_ENCLOSED_SIZE:usize = 5000;
// 3954 too high

#[derive(PartialEq,Eq,Hash,Debug)]
enum DropStatus{
    LAVA,
    ENCLOSED,
    UNENCLOSED(Drop),
}

impl DropStatus{
    pub fn is_unenclosed(&self)->bool{
        match self {
            UNENCLOSED(_)=>true,
            _ => false
        }
    }
}

fn main() {

    //To get around stack size issues.
    let child = thread::Builder::new().stack_size(32 * 1024 * 1024).spawn(move || {
        actual_main();
    }).unwrap();
    let res = child.join().unwrap();


}

fn actual_main(){
    let mut input:Vec<Drop> = get_lines("input.txt")
        .map(|l|{
            let mut vals = l.split(',');
            let x:XCoord = vals.next().expect("Expected ','").parse().expect("Error parsing x coord");
            let y:YCoord = vals.next().expect("Expected ','").parse().expect("Error parsing y coord");
            let z:ZCoord = vals.next().expect("Expected ','").parse().expect("Error parsing z coord");
            (x,y,z)
        }).collect();

    let boundries = input.iter()
        .fold(((XCoord::MAX,XCoord::MIN),(YCoord::MAX,YCoord::MIN),(ZCoord::MAX,ZCoord::MIN)),
              |((xmin,xmax),(ymin,ymax),(zmin,zmax)),(x,y,z)|
                  ((xmin.min(*x),xmax.max(*x)),(ymin.min(*y),ymax.max(*y)),(zmin.min(*z),zmax.max(*z))));
    print_the_whole_thing_i_guess(&input);
    let mut is_enclosed_cache:HashMap<Drop,DropStatus> = HashMap::new();
    let mut tempcache: HashSet<Drop> = HashSet::new();
    input.iter().for_each(|d|{is_enclosed_cache.insert(*d,LAVA);});
    assert_eq!(is_enclosed_cache.get(&(8,12,7)),Some(&LAVA));
    let mut enclosed_sides = 0;
    for drop in input.iter(){
        for neighbour in get_neighbours(*drop) {
            if !is_enclosed_cache.contains_key(&neighbour){
                assert!(tempcache.is_empty());
                let result  = check_if_enclosed(&mut is_enclosed_cache,&mut tempcache,neighbour,&boundries);
                if result>0{
                    if result>20 {
                        println!("found a set of {} cubes with surface {}", tempcache.len(), result);
                    }
                    enclosed_sides+=result;
                    assert_eq!((result == 0), tempcache.is_empty());
                    for item in tempcache.iter(){
                        assert_eq!(is_enclosed_cache.get(item),None);
                        is_enclosed_cache.insert(*item,ENCLOSED);
                    }
                } else {
                    assert!(is_enclosed_cache.get(&neighbour).unwrap().is_unenclosed());
                    for item in tempcache.iter(){
                        assert!(is_enclosed_cache.get(item).unwrap().is_unenclosed(),"The value of {:?} was {:?} instead of Some(enclosed). current neighbour:{:?}",item,is_enclosed_cache.get(item),neighbour)
                    }
                }
                tempcache.clear();
            }
        }
    }
    for (space,status) in is_enclosed_cache.iter(){
        if *status == ENCLOSED{
            input.push(*space);
        }
    }
    let mut surface = input.len()*6;
    for i in 0..input.len(){
        for j in i+1..input.len(){
            if are_touching(input[i],input[j]){
                surface-=2;
            }
        }

    }
    println!("answer1:{surface}");
    println!("enclosed sides:{}",enclosed_sides);
    // println!("answer2:{}",surface-enclosed_sides);
}
//For debugging
const poivec: &'static [Drop] = &[(8,13,7),(9,13,7),(10,13,7),(11,13,7),(12,13,7),(13,13,7),(14,13,7),(15,13,7),(16,13,7),(17,13,7),(18,13,7),(19,13,7)];
fn check_if_enclosed(cache:&mut HashMap<Drop,DropStatus>,tempcache:&mut HashSet<Drop>,drop:Drop,boundries:&((XCoord,XCoord),(YCoord,YCoord),(ZCoord,ZCoord)))->usize{
    let (x,y,z) = &drop;
    let ((xmin,xmax),(ymin,ymax),(zmin,zmax)) = boundries;
    if poivec.contains(&drop){
        println!("now looking at point of interest {},{},{}",drop.0,drop.1,drop.2);
    }
    if *x<=*xmin || *x>= *xmax || *y<=*ymin || *y>= *ymax || *z<=*zmin || *z>= *zmax {
        cache.insert(drop,UNENCLOSED(drop));
        if poivec.contains(&drop){
            println!("out of bounds???");
        }
        return 0;
    }



    tempcache.insert(drop);
    let mut side_count = 0;
    for neighbour in get_neighbours(drop){
        if poivec.contains(&drop){
            println!("now looking at neighbour: {neighbour:#?}");
        }
        match  cache.get(&neighbour){
            None => {
                if !tempcache.contains(&neighbour){
                    if poivec.contains(&drop){
                        println!("doing recursive call");
                    }
                    let rec_res = check_if_enclosed(cache,tempcache,neighbour,boundries);
                    if poivec.contains(&drop){
                        println!("result = {rec_res} for inquiry into {},{},{}",neighbour.0,neighbour.1,neighbour.2);
                    }
                    if rec_res == 0{
                        if let Some(UNENCLOSED(c)) = cache.get(&neighbour){
                            let cause  = *c;
                            cache.insert(drop,UNENCLOSED(cause));
                            for drop in tempcache.iter() {
                                if poivec.contains(&drop){
                                    println!("weird unenclose found");
                                }
                                cache.insert(*drop,UNENCLOSED(cause));
                            }
                            tempcache.clear();
                            return 0;
                        }
                        // else {
                        //     println!("Should be unreachable, value was {:?}",cache.get(&neighbour));
                        //     unreachable!()
                        // }

                    } else {
                        side_count +=rec_res
                    }
                } else {
                    if poivec.contains(&drop){
                        println!("now already in path");
                    }
                }

            },
            Some(UNENCLOSED(c)) => {
                let cause = *c;
                if poivec.contains(&drop){
                    println!("neighbour is unenclosed because {},{},{} is reachable",cause.0,cause.1,cause.2);
                }
                cache.insert(drop,UNENCLOSED(cause));
                for drop in tempcache.iter() {
                    cache.insert(*drop,UNENCLOSED(cause));
                }
                tempcache.clear();
                return 0;},
            Some(LAVA) => { side_count +=1;
                if poivec.contains(&drop){
                    println!("neighbour is lava");
                }},
            Some(ENCLOSED) => {
                // assert_eq!(tempcache.len(), 1)
                unreachable!("This should not be possible")
            }
        }
    }
    // println!("debug: found enclosed space at {},{},{}",drop.0,drop.1,drop.2);
    if drop == (8,13,7){
        println!("returning correctly with val {side_count}");
    }
    // assert_ne!(side_count, 0);
    return side_count;

}

fn difference(coord:Coord,other:Coord)->usize{
    (coord.max(other) - coord.min(other)) as usize
}

fn are_touching((x,y,z):Drop,(x2,y2,z2):Drop)->bool{
    (x==x2 && y==y2 && difference(z,z2)==1) ||
        (x==x2 && z ==z2 && difference(y,y2)==1) ||
        (y==y2 && z==z2 && difference(x,x2)==1)
}

fn get_neighbours((x,y,z):Drop)-> [Drop;6]{
    [(x+1,y,z),(x-1,y,z),(x,y+1,z),(x,y-1,z),(x,y,z+1),(x,y,z-1)]
}

fn print_the_whole_thing_i_guess(input: &Vec<Drop>){
    //This is modified for debuging and no longer actually prints the whole thing
    for z in 7..8{
        for y in 0..21{
            for x in 0..21{
                if input.contains(&(x,y,z)){
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
}
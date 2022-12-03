use std::collections::HashSet;
use std::fs;
use std::io::{BufRead, BufReader};

/// Today was a massive battle against "this value cannot be moved".
/// In the end I gave up and just added some copy statements, so this is probably far from
/// an optimal solution. The main idea was to solve this using hashsets, but it turns out that
/// the rust Hashset interface is either complicated, stupid or both.
/// After every set operation, the entire result needs to be copied before you can continue
/// (there is probably a way to not have to do this, but I havent found it yet).
/// Operations can also not be chained, which makes even calculating the intersection of three sets
/// a long and dificult task for some reason.
fn main() {
    let file = fs::File::open("input.txt").expect("Error while reading file");
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.expect("failure to parse rucksack"));

    /// Question 1.
    let mut sum = 0;
    let mut bags: Vec<HashSet<char>> = Vec::new();
    for line in lines{
        let (bag1,bag2) =  line.split_at(line.len()/2);
        let bagset1: HashSet<char> = bag1.chars().collect();
        let bagset2: HashSet<char> = bag2.chars().collect();
        let problem_element = bagset1.intersection(&bagset2).next().expect("there was no shared element on this line.");
        // println!("line: {:?}, {}, intersect: {}",bagset1,bag2,problem_element);
        sum+=get_prio(problem_element);
        // I have no idea how to solve this better without the clone, but at this point I just want it to work.
        bags.push( bagset1.union(&bagset2).map(|v| v.clone()).collect())
    }
    println!("sum: {}",sum);

    //question 2
    let mut badgesum = 0;
    ///Again, after 30 minutes of trying there does not seem to be a way to loop over a list in sets of three,
    /// without spreading out the unpacking over several lines like this.
    for chunk in bags.chunks(3)
    {
        let bag1 = chunk.get(0).expect("bags were not given in multiple of 3");
        let bag2 = chunk.get(1).expect("bags were not given in multiple of 3");
        let bag3 = chunk.get(2).expect("bags were not given in multiple of 3");
        let doubles: HashSet<char> = bag1.intersection(bag2).copied().collect();
        let mut badge = bag3.intersection(&doubles);
        badgesum+=get_prio(badge.next().expect("there was no badge found"))

    }
    println!("badge sum: {}",badgesum)

}

/// Get the priority value for a given item tag.
fn get_prio(c: &char) -> u32{
    return if c > &'Z' {
        *c as u32 - 'a' as u32 + 1
    } else {
        *c as u32 - 'A' as u32 + 27
    }
}

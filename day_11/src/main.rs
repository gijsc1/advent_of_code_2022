extern crate core;

use std::borrow::Borrow;
use std::collections::HashMap;
use queues::{IsQueue};
use crate::file_io::get_string;
use crate::monkeys::{Monkey, parse_monkey};

#[path="../../shared_code/file_io.rs"]
mod file_io;
mod monkeys;

fn main() {
    let filecontent = get_string("input.txt");
    let mut monkeys:Vec<Monkey> = Vec::new();
    let mut items = HashMap::new();
    for monekytext in filecontent.split("\n\n"){
        parse_monkey(monekytext, &mut monkeys, &mut items).expect("Error parsing monkey");
    }
    println!("monkeycount:{}",monkeys.len());
    for _ in 0..20 {
        for monkey in monkeys.iter_mut() {
            let item_amount = items.get(monkey.get_name().borrow()).unwrap().size();
            for _ in 0..item_amount {
                let item = items.get_mut(monkey.get_name().borrow()).unwrap().remove().expect("failure to extract item");
                let (new_item, target_monkey) = monkey.handle_trow(item);
                items.get_mut(target_monkey.borrow()).unwrap().add(new_item).unwrap();
            }
        }
    }
    let mut throwcounts: Vec<usize> = monkeys.iter().map(|m|m.get_throw_count()).collect();
    throwcounts.sort_by(|a,b|b.cmp(a));
    println!("answer1: {}",throwcounts[0]*throwcounts[1]);
    println!("Hello, world!");
}

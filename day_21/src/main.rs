use std::collections::HashMap;
use crate::file_io::get_lines;
use crate::operation::{Operation, parse_op_line};
use crate::operation::Operation::Val;
use crate::types::{MonkeyID, MonkeyMap, ValType};

#[path="../../shared_code/file_io.rs"]
mod file_io;
#[path="../../shared_code/error.rs"]
mod error;
#[path="../../shared_code/parsing.rs"]
mod parsing;
mod types;
mod operation;


fn main() {
    let mut monkeymap:MonkeyMap = get_lines("input.txt")
        .map(|l|parse_op_line(&l).unwrap())
        .map(|m|m.into())
        .collect();
    let rootname = "root".to_string();
    println!("answer1:{}",get_val_from_monkeymap(&monkeymap,&rootname,None).unwrap());
    let human_name = "humn".to_string();
    monkeymap.insert(human_name.clone(),Operation::VAR);
    println!("answer2:{}",question_2(&mut monkeymap,&rootname));
    // monkeymap.insert(human_name,Val(answer2));
    // println!("answer2 double check: {} == {}",get_val_from_monkeymap(&mut monkeymap,&"vpmn".to_string(),Some(answer2)).unwrap(),get_val_from_monkeymap(&mut monkeymap,&"pqtt".to_string(),Some(answer2)).unwrap());
}
///Varval is only there for debugging question 2
fn get_val_from_monkeymap(map: &MonkeyMap,name:&MonkeyID,varval:Option<ValType>)->Option<ValType>{
    match map.get(name)? {
        Val(answer) => {Some(*answer)}
        Operation::App(op, left_monkey, right_monkey) => {
            Some(op.eval(get_val_from_monkeymap(map,left_monkey,varval)?,get_val_from_monkeymap(map,right_monkey,varval)?))
        }
        Operation::VAR=> {varval}
    }
}

fn question_2(map: &mut MonkeyMap,rootname:&MonkeyID)->ValType{
    if let Some(Operation::App(_,left_monkey,right_monkey)) = map.get(rootname){
        println!("full equation:");
        print_equation(map,left_monkey);
        print!("\n=\n");
        print_equation(map,right_monkey);
        println!();
        // println!("handling {}:: {}={}",rootname,left_monkey,right_monkey);
        if let Some(left_answer) = get_val_from_monkeymap(map,left_monkey,None){

            // println!("now solving {}={}",right_monkey,left_answer);
            return  make_equal(map,right_monkey,left_answer);
        }
        if let Some(right_answer) = get_val_from_monkeymap(map,right_monkey,None){
            // println!("now solving {}={}",left_monkey,right_answer);
            return  make_equal(map,left_monkey,right_answer);
        }
    }
    unreachable!()
}

///A terrible way to solve the problem, but easy to implement.
/// Assumes that there is only one variable in the monkeymap, it is only used once, and each monkey that uses it is only used once.
fn make_equal(map: &MonkeyMap,name: &MonkeyID, value:ValType)->ValType{
    match map.get(name).unwrap() {
        Operation::Val(answer) => {unreachable!()
            //if *answer==value{value} else { unreachable!() }
             }
        Operation::App(op, left_monkey, right_monkey) => {
            // println!("handling {}:: {}{}{}",name,left_monkey,op,right_monkey);
            if let Some(left_answer) = get_val_from_monkeymap(map,left_monkey,None){
                // println!("now solving {}{}{}={}",left_answer,op,right_monkey,value);
                let ans = make_equal(map,right_monkey,op.left_inverse(value,left_answer));
                let right_answer = get_val_from_monkeymap(map,right_monkey,Some(ans)).unwrap();
                // println!("answer check: {}{}{}({})={}",left_answer,op,right_monkey,right_answer,value);
                // assert_eq!(op.eval(left_answer,right_answer),value,
                //            "failed for {}{}{}({})={}",left_answer,op,right_monkey,right_answer,value);
                return ans;
            }
            if let Some(right_answer) = get_val_from_monkeymap(map,right_monkey,None){
                // println!("now solving {}{}{}={}",left_monkey,op,right_answer,value);
                let ans = make_equal(map,left_monkey,op.right_inverse(value,right_answer));
                let left_answer = get_val_from_monkeymap(map,left_monkey,Some(ans)).unwrap();
                // println!("answer check: {}({}){}{}={}",left_monkey,left_answer,op,right_answer,value);
                // assert_eq!(op.eval(get_val_from_monkeymap(map,left_monkey,Some(ans)).unwrap(),right_answer),value,
                //            "failed for {}{}{}={}",left_monkey,op,right_answer,value);
                return ans;
            }
            unreachable!()
        }
        Operation::VAR=>{
            // println!("found humn={}",value);
            value }
        _ => {unreachable!("var not supported for get_val")}
    }
}

fn print_equation(map: &MonkeyMap, monkey:&MonkeyID){
    match map.get(monkey).unwrap() {
        Operation::Val(val) => {print!("{}",val)}
        Operation::App(op, left_monkey, right_monkey) => {
            print!("(");
            print_equation(map,left_monkey);
            print!("{}",op);
            print_equation(map,right_monkey);
            print!(")");
        }
        Operation::VAR => {print!("X")}
    }
}

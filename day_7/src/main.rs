mod command;
mod filesystem;

use std::fs;
use std::io::{BufRead, BufReader};
use trees::{Node, tr, Tree, TreeWalk, TupleTree};
use crate::command::parse_commands;
use crate::filesystem::FileNode::{Dir, File};
use crate::filesystem::{FileNode, update_fs};

//Today's goa: Trees. Apparently easier said than done in rust as they are not part of the stdlib.

fn main() {
    let file = fs::File::open("input.txt").expect("Error while reading file");
    let reader = BufReader::new(file).lines()
        .map(|s|s.expect("Failure to read file"));
    let commands = parse_commands(reader).expect("Error while parsing commands");
    // println!("commands:\n{:?}",commands);
    let mut tree = tr(Dir("/".to_string()));
    let mut iter = commands.into_iter().peekable();
    while iter.peek().is_some(){
        update_fs(tree.root_mut(),&mut iter);
    }
    // tree.push_back(tr(File("testfile".to_string(),42)));
    // print_tree(tree.root(),0);
    // println!("filecount: {}",count_predicate(tree.root(),is_file))
    let (totalsize,smallsize) = question_1(tree.root(),100000);
    println!("total: {totalsize}, small:{smallsize}");
    let needed_space = 30000000-(70000000-totalsize);
    let best_size = question_2(tree.root(),needed_space,std::usize::MAX);
    println!("best delete target: {best_size}");

}

fn print_tree(tree: &Node<FileNode>,depth:usize){
    println!("{:indent$}node: {:?}","",tree.data(),indent=depth);
    for item in tree.iter(){
        print_tree(item,depth+4);
    }
}

///At this point I already spend way to much time, so efficiency is out the window
///return value (total size, total size less than 100000
fn question_1(node:&Node<FileNode>,maxsize:usize)->(usize,usize){
    let (total_size,small_size) = node.iter().map(|n| question_1(n,maxsize))
        .fold((0,0),|(acc1,acc2),(total_size,small_size)|
            (acc1+total_size,
             acc2+small_size));
    let self_size = node.data().get_size()+ total_size;
    if self_size<maxsize && node.data().is_dir(){
        // println!("counting {} as small with size {}. smallsize is {}",node.data().get_name(),self_size,small_size);
        return (self_size,small_size+self_size)
    } else {
        return (self_size,small_size)
    }
}
///This is inneficiency squared, but eh at this point.
fn question_2(node:&Node<FileNode>,needed_size:usize,best_size:usize)->usize{
    // println!("debug: node size{}",node.iter().count());
    let new_best_size = node.iter().map(|n| question_2(n,needed_size,best_size))
        .min().unwrap_or(std::usize::MAX).min(best_size);
    let (total_size,small_size) = node.iter().map(|n| question_1(n,0))
        .fold((0,0),|(acc1,acc2),(total_size,small_size)|
            (acc1+total_size,
             acc2+small_size));
    return if total_size >= needed_size && total_size < new_best_size { total_size } else { new_best_size }

}
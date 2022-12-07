use std::borrow::BorrowMut;
use std::pin::Pin;
use trees::{Node, tr, Tree, TreeWalk, TupleTree};
use crate::command::{Command, ListLine};
use crate::command::Command::{CD, HOME, LIST};
use crate::filesystem::FileNode::{Dir, File};

#[derive(Debug)]
pub enum FileNode{
    Dir(String),
    File(String,usize)
}

impl FileNode where{
    pub fn get_name(&self) ->&str{
        match *self {
            Dir(ref name) => name,
            File(ref name,_) => name
        }
    }

    pub fn get_size(&self) ->usize{
        match *self {
            Dir(_)=>0,
            File(_,size)=>size
        }
    }

    pub fn is_dir(&self)->bool{
        match *self {Dir(_) =>true,_=>false }
    }

}
///This whole signature is way to complicated, but I dont know enough rust to do this better.
/// The return value is represents, do a cd .., because I cannot seem to get a mutable reference to a parant node,
/// Only mutable references to children
pub fn update_fs<I:Iterator<Item=Command>>(mut currentnode:Pin<& mut Node<FileNode>>,commands:&mut I)-> bool{
    while let Some(command) = commands.next(){
        // println!("debug: now handling command :{:?}",command);
        match command {
            CD (ref dir) =>{
                if dir.as_str() == ".."{
                    return true;
                } else{
                    if update_fs( get_child(&mut currentnode,dir.as_str()),commands){
                        continue
                    } else { return false; }
                }
            },
            HOME=>{return false;},
            LIST(results) =>{
                // let current = walk.get().expect("");
                // ?.push_back(tr(Dir("t".to_string())));
                for item in results{
                    match item {
                        ListLine::File{size,name} =>{
                            currentnode.push_back(tr(FileNode::File(name,size)))
                        },
                        ListLine::Dir{name} =>{
                            currentnode.push_back(tr(FileNode::Dir(name)))
                        }
                    }
                }
            }
        }
    }
    return false;
}


pub fn get_child<'a,'b>(node : &'a mut Pin<&'b mut Node<FileNode>>,child_name:&str) -> Pin<&'a mut Node<FileNode>>{
    for child in node.iter_mut(){
        let name = child.data().get_name();
        if name == child_name{
            return child;
        }
    }

    panic!("no child found with name: {}",child_name)
}
///Depreciated
pub fn set_to_child(tree: &mut TreeWalk<FileNode>,child_name:&str){
    let mut i = 0;
    while let Some(child) = tree.to_child(i){
        let name = child.node().data().get_name();
        if name ==child_name{
            return
        }
        tree.to_parent();
        i+=1;
    }
    panic!("no child found with name: {}",child_name)
}

///Depreciated
pub fn set_to_root<T>(tree: &mut TreeWalk<T>){
    while tree.get_parent().is_some() {
        tree.to_parent();
    }
}
use std::fmt::{Display, Formatter};
use crate::move_operation::Move;
use crate::move_operation::Move::{Down, Left, Right, Up};

pub struct Board{
    board: Vec<Vec<u8>>,
    rope:Vec<(usize,usize)>,
    width:usize,
    heigth:usize
}
const EMPTY:u8 = 0;
const VISITED:u8 = 1;

pub fn create_board(size:usize,length:usize)->Board{
    Board{
        board: vec![vec![EMPTY;size];size],
        rope:vec![(0,0);length],
        width: size,
        heigth: size,
    }
}

impl Board {
    fn get_tail(&self)->(usize,usize)
    {
        return self.rope[self.rope.len()-1];
    }

    ///get the textual representation of a space of the board.
    pub fn get_val(&self,x:usize,y:usize)->char{
        return if self.rope[0] == (x, y) {
            'H'
        } else if self.get_tail() == (x, y) {
            'T'
        } else if self.rope[1..self.rope.len()-1].contains(&(x, y))
        {
            '*'
        } else {
            match self.board[x][y] {
                0 => '0',
                1 => '1',
                _ => unreachable!()
            }

        }
    }

    pub fn do_move(&mut self, move_op:&Move){

        for _ in 0..move_op.get_amount(){
            self.do_move_once_main_head(move_op);
            for i in 0..self.rope.len()-1{
                let newtail = do_catchup(self.rope.get(i+1).unwrap(),self.rope.get(i).unwrap());
                self.rope[i+1] = newtail;
            }
            self.mark_tail();
        }
    }

    ///set the y coordinate at a given index into the rope.
    fn set_y_at_index(&mut self, index:usize,val:usize)
    {
        let (_,oldy) = self.rope.get_mut(index).unwrap();
        *oldy = val;
    }

    ///set the x coordinate at a given index into the rope.
    fn set_x_at_index(&mut self, index:usize,val:usize)
    {
        let (oldx,_) = self.rope.get_mut(index).unwrap();
        *oldx = val;
    }

    ///Move the head of the rope acording to the direction, but not the amount, of a move operation.
    fn do_move_once_main_head(&mut self, move_op:&Move){
            let (xheadref, yheadref) = self.rope.get(0).unwrap();
            let xhead = *xheadref;
            let yhead = *yheadref;
            match move_op {
                Up(_)=>{
                    if yhead == self.heigth-1{
                        self.add_space_above(1);
                    }
                    self.set_y_at_index(0,yhead+1);
                },
                Down(_)=>{
                    if yhead == 0{
                        self.add_space_below(1);
                    }
                    self.set_y_at_index(0,self.rope.get(0).unwrap().1-1)
                },
                Right(_)=>{
                    if xhead == self.width-1{
                        self.add_space_right(1);
                    }
                    self.set_x_at_index(0,xhead+1)
                },
                Left(_)=>{
                    if xhead == 0{
                        self.add_space_left(1);
                    }
                    self.set_x_at_index(0,self.rope.get(0).unwrap().0-1)
                }
            }
    }

    ///Mark the space of the board where the tail is currently at as visited.
    fn mark_tail(&mut self){
        // println!("debug:\n{}",self);
        let (x,y) = self.get_tail();
        self.board[x][y]=VISITED;
    }

    fn add_space_above(&mut self,amount:usize){
        self.heigth+=amount;
        for col in self.board.iter_mut(){
            col.reserve(amount);
            for _ in 0..amount {
                col.push(EMPTY);
            }
        }
    }

    fn add_space_below(&mut self,amount:usize){
        self.add_space_above(amount);
        for col in self.board.iter_mut(){
            col.rotate_right(amount);
        }
        for (_,y) in self.rope.iter_mut(){
            *y+=amount;
        }

    }

    fn add_space_right(&mut self,amount:usize){
        self.width+=amount;
        self.board.reserve(amount);
        for _ in 0..amount{
            self.board.push(vec![EMPTY;self.heigth]);
        }

    }

    fn add_space_left(&mut self,amount:usize){
        self.add_space_right(amount);
        self.board.rotate_right(amount);
        for (x,_) in self.rope.iter_mut(){
            *x+=amount;
        }
    }

    pub fn iter(&self) -> BoardIter {
        return BoardIter{ board: self, position: (0, 0) }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in (0..self.heigth).rev(){
            for x in 0..self.width-1 {
                write!(f, "{}, ", self.get_val(x,y))?;
            }
            write!(f,"{}\n",self.get_val(self.width-1,y))?
        }
        Ok(())
    }
}

///For iteration over a board.
pub struct BoardIter<'l>{
    board: &'l Board,
    position: (usize,usize)

}

impl Iterator for BoardIter<'_>{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let (x,y) = self.position;
        let xmax = x == self.board.width-1;
        let ymax = y == self.board.heigth-1;
        if  xmax && ymax{
            return None;
        }
        let item = self.board.board.get(x)?.get(y)?;
        let mut newx = x;
        let mut newy = y;
        if xmax{
            newx=0;
            newy = y+1;
        } else {
            newx+=1;
        }
        self.position = (newx,newy);
        return Some(*item);
    }
}

///The checks for if after moving the tail in one direction, it should also move in the other direction, and how far.
fn do_catchup_secondary(head:&usize, tail:&usize) -> usize
{
    if *head > *tail{
        return *tail+1
    } else if *head < *tail{
        return *tail-1
    } else {
        return *tail
    }
}

///Calculates the new position of a tail element, given the position of the element it is chasing.
fn do_catchup(tail: &(usize,usize),head:&(usize,usize))->(usize,usize){
    let (xhead,yhead) = head;
    let (xtail,ytail) = tail;
    if *xhead> *xtail+1{
        return (*xtail+1,
        do_catchup_secondary(yhead,ytail))
    } else if *xhead+1< *xtail{
        return (*xtail-1,
        do_catchup_secondary(yhead,ytail))
    } else if *yhead> *ytail+1{
        return (
        do_catchup_secondary(xhead,xtail),
        *ytail+1)
    } else if *yhead+1< *ytail{
        return (
        do_catchup_secondary(xhead,xtail),
        *ytail-1)
    }
    return (*xtail,*ytail)
}

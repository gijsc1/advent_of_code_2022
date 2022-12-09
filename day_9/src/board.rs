use std::borrow::BorrowMut;
use std::fmt::{Display, Formatter};
use crate::move_operation::Move;
use crate::move_operation::Move::{Down, Left, Right, Up};

pub struct Board{
    board: Vec<Vec<u8>>,
    head:(usize,usize),
    tail:(usize,usize),
    width:usize,
    heigth:usize
}
const EMPTY:u8 = 0;
const VISITED:u8 = 1;

pub fn create_board(size:usize)->Board{
    Board{
        board: vec![vec![EMPTY;size];size],
        head: (size-1, size-1),
        tail: (size-1, size-1),
        width: size,
        heigth: size,
    }
}

impl Board {
    pub fn get_val(&self,x:usize,y:usize)->char{
        return if self.head == (x, y) {
            'H'
        } else if self.tail == (x, y) {
            'T'
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
            self.do_move_once(move_op);
            self.mark_tail();
        }
    }

    fn do_move_once(&mut self, move_op:&Move){
        match move_op {
            Up(_)=>{
                if self.head.1 == self.heigth-1{
                    self.add_space_above(1);
                }
                self.head.1+=1;
                if self.tail.1 < self.head.1-1{
                    self.tail.1+=1;
                    self.tail.0=self.head.0;
                }
            },
            Down(_)=>{
                if self.head.1 == 0{
                    self.add_space_below(1);
                }
                self.head.1-=1;
                if self.tail.1 > self.head.1+1{
                    self.tail.1-=1;
                    self.tail.0=self.head.0;
                }
            },
            Right(_)=>{
                if self.head.0 == self.width-1{
                    self.add_space_right(1);
                }
                self.head.0+=1;
                if self.tail.0 < self.head.0-1{
                    self.tail.0+=1;
                    self.tail.1=self.head.1;
                }
            },
            Left(_)=>{
                if self.head.0 == 0{
                    self.add_space_left(1);
                }
                self.head.0-=1;
                if self.tail.0 > self.head.0+1{
                    self.tail.0-=1;
                    self.tail.1=self.head.1;
                }
            }
        }
    }

    fn mark_tail(&mut self){
        self.board[self.tail.0][self.tail.1]=VISITED;
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
        let (_,yhead) = self.head.borrow_mut();
        *yhead+=amount;
        let (_,ytail) = self.tail.borrow_mut();
        *ytail+=amount;
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
        let (xhead,_) = self.head.borrow_mut();
        *xhead+=amount;
        let (xtail,_) = self.tail.borrow_mut();
        *xtail+=amount;
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

pub fn test_board(board: &mut Board)
{
    board.add_space_above(2);
    println!("{board}");
    board.add_space_right(2);
    println!("{board}");
    board.add_space_left(2);
    println!("{board}");
    board.add_space_below(2);
    println!("{board}");
}

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


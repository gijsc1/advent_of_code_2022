use std::fmt::{ Display, Formatter};

#[derive(Debug,Clone)]
pub struct Board{
    board :Vec<Vec<char>>,
}

impl Board{
    pub fn new(num_cols: usize,num_rows:usize)-> Board{
        let mut board = Vec::with_capacity(num_cols);
        for _ in 0..num_cols {
            board.push(Vec::with_capacity(num_rows));
        }
        Board{board }
    }

    pub fn push_row(&mut self, row: Vec<char>){
        for (c,column) in row.into_iter().zip(self.board.iter_mut()){
            column.push(c);
        }
    }

    fn num_cols(&self)->usize{
        self.board.len()
    }

    pub fn move_amount(&mut self,amount: usize, from:usize,to:usize){
        for _ in 0..amount{
            self.do_move(from,to,0)
        }
    }

    fn do_move(&mut self, from:usize,to:usize,offset:usize){
        // println!("doing move from {} to {} with offset {}",from,to,offset);
        if let Some(from_index) = self.get_top_empty_index(from,offset){
            if from_index == 0{
                // println!("attempting to do move from empty stack. aborting");
                return
            }
            if let Some(to_index) = self.get_top_empty_index(to,0){
                // println!("actually doing move from {},{} to {},{}",from,from_index-1,to,to_index);
                self.board[to][to_index] = self.board[from][from_index-1];
                self.board[from][from_index-1] = ' ';
            } else {
                // println!("to column overloaded! Adding more space");
                self.push_row(vec![' ';self.num_cols()]);
                self.do_move(from,to,offset);
            }
        } else {
            // println!("from column overloaded! Adding more space");
            self.push_row(vec![' ';self.num_cols()]);
            self.do_move(from,to,offset);}
    }

    fn get_top_empty_index(&self,col:usize,offset:usize)-> Option<usize>{
        //println!("attempting to find empty index in col {}\ncolumn: {:?}",col,self.board[col]);
        let index = self.board[col].iter().rev().position(|c| *c !=' ')
            .map(|i|self.board[col].len()-i)
            .unwrap_or(0);
            if index == self.board[col].len() || index < offset{
                None
            } else { Some(index-offset) }
    }


    pub fn get_top(&self)->String{
        let mut s = String::with_capacity(self.num_cols());
        for column in 0..self.board.len() {
            let top_index = self.get_top_empty_index(column,0).unwrap_or(self.board[column].len());
            s.push(self.board[column][top_index-1]);
        }
        return s;
    }

    pub fn move_multi(&mut self,amount: usize, from:usize,to:usize){
        for i in (0..amount).rev(){
            self.do_move(from,to,i)
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row_i in (0..self.board[0].len()).rev(){
            let mut rowstring = String::with_capacity(self.num_cols()*2);
            for col_i in 0..self.num_cols(){
                rowstring.push(self.board[col_i][row_i]);
                rowstring.push(' ');
            }
            rowstring.pop();
            write!(f,"{}\n",rowstring)?;
        }

        let numline_base = String::with_capacity(self.num_cols()*2);
        let mut numline  = (1..=self.num_cols()).into_iter().fold(numline_base,|mut acc,val|
            {
                acc.push_str(&val.to_string());
                acc.push(' ');
                acc
            });
        numline.pop();
        write!(f,"{}",numline)
    }
}

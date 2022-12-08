use std::cmp::max;
use std::fmt::{Debug, Display};
use std::fs;
use std::io::{BufReader};
use itertools::{Either, Itertools};
use rulinalg::matrix::{BaseMatrix, BaseMatrixMut, Matrix};
use rulinalg::{vector};
use rulinalg::vector::Vector;
use utf8_chars::BufReadCharsExt;

// Todays goal: Use a matrix library

fn main() {
    let file = fs::File::open("input.txt").expect("Error while reading file");
    let mut reader = BufReader::new(file);
    let chars = reader.chars()
        .map(|l| l.expect("Failure parsing char"))
        .map(|c| c.to_digit(10))
        .filter(|maybe| maybe.is_some())
        .map(|c| c.unwrap() as i32);
    let chars = chars.collect::<Vec<i32>>();
    let len = (chars.len() as f64).sqrt() as usize;
    let matrix: Matrix<i32> = Matrix::new(len,len,chars);
    let mut most_blocking = Matrix::new(len,len,vector![0;len*len]);

    const LOWEST: i32 = -1;
    let visible_left= calculate_visibility_matrix(&matrix, &mut most_blocking, len, LOWEST, false);
    let visible_right = calculate_visibility_matrix(&matrix, &mut most_blocking, len, LOWEST,true);
    let transposed_matrix = matrix.transpose();
    let visible_top = calculate_visibility_matrix(&transposed_matrix, &mut most_blocking, len, LOWEST, false).transpose();
    let visible_bottom = calculate_visibility_matrix(&transposed_matrix, &mut most_blocking, len, LOWEST, true).transpose();

    let visible_matrix:Matrix<bool> = Matrix::new(len,len,
                                                  visible_right.iter()
        .zip_eq(visible_left.iter())
        .zip_eq(visible_bottom.iter())
        .zip_eq(visible_top.iter())
        .map(|(((r,l),b),t)| *r || *l || *b || *t)
        .collect::<Vector<bool>>());
    let answer1:usize = visible_matrix.iter().map(|b|*b as usize).sum();
    println!("answer1: {answer1}");

    let indices = (0..len).cartesian_product(0..len);
    let answer2 = indices.map(|coords|calculate_scenic_value(&matrix,coords))
        .max()
        .expect("Matrix was empty???");
    println!("answer2: {answer2}");

    // println!("scene value: {}",calculate_scenic_value(&matrix,(3,2)));

}

fn calculate_visibility_matrix<T:Ord + Copy>(matrix:&Matrix<T>,blocking_matrix:&mut Matrix<T>,size:usize,lowest:T,do_reverse:bool)->Matrix<bool>{
    let mut highest =lowest;
    //look from left to right
    for (row,mut blocking_row) in matrix.row_iter().map(|r|r.raw_slice())
        .zip(blocking_matrix.row_iter_mut()) {
        let iter_start = row.iter().zip(blocking_row.raw_slice_mut());
        let iter = match do_reverse {
            false => Either::Left(iter_start),
            true => Either::Right(iter_start.rev())
        };
        for (val,blocking_val) in iter{
            *blocking_val = highest;
            highest = max(highest,*val);
        }
        highest=lowest;
    }
    return Matrix::new(size,size,matrix.iter().zip(blocking_matrix.iter()).map(
        |(tree,blocking)| *tree>*blocking
    ).collect::<Vector<bool>>());
}

fn calculate_scenic_value<T:Ord+Display>(matrix:&Matrix<T>,coords:(usize,usize))->usize{
    let (y,x) = coords;
    let len = matrix.rows();
    // println!("debug: len:{len}");
    if y == 0 || x ==0 || y ==len-1 || x== len-1{return 0}

    let hight = &matrix[[y,x]];
    // println!("debug: height:{hight}");
    let mut score = 0;
    let mut scores = Vec::new();

    //look up
    for high_y in (0..y).rev(){
        score+=1;
        if matrix[[high_y,x]] >= *hight{break;}
    }
    scores.push(score);
    score=0;

    //look down
    for low_y in y+1..len{
        score+=1;
        if matrix[[low_y,x]] >= *hight{break;}
    }
    scores.push(score);
    score=0;

    //look left
    for low_x in (0..x).rev(){
        score+=1;
        if matrix[[y,low_x]] >= *hight{break;}
    }
    scores.push(score);
    score=0;

    //look right
    for high_x in x+1..len{
        score+=1;
        if matrix[[y,high_x]] >= *hight{break;}
    }
    scores.push(score);
    return scores.iter().fold(1,|acc,val|acc*val);

    0
}
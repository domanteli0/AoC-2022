use std::fmt::Debug;

use matrix::{prelude::Compressed, Matrix, Size, Number};

fn main() {
    const STR: &str = include_str!("../input.txt");

    let x_len = STR.lines().next().unwrap().len();
    let y_len = STR.lines().count();

    let mut mtrx: Compressed<u32> = Compressed::zero((x_len, y_len));

    
    for (ix, line) in STR.lines().enumerate()
    {
        for (jy, ch) in line.chars().enumerate()
        {
            mtrx.set((ix, jy), (ch as u32) - 48);
        }
    }
    
    for row in mtrx.iter()
    {
        println!("row: {row:?}");
    }

    dbg!(is_visible(&mtrx, 1,2));

    let mut scores = Vec::new();
    
    let (rows, cols) = mtrx.dimensions();

    for x in 0..rows
    {
        for y in 0..cols
        {
            scores.push(is_visible(&mtrx, x, y));
        }
    }

    println!("max: {:?}", scores.into_iter().max());

    // println!("count: {}", visibles.into_iter().filter(|b| *b).count());

    // println!("x_len: {x_len}, y_len: {y_len}, mtrx: {mtrx:?}");
}

// trait IsVisible {
//     fn is_visible(&self, x: usize, y: usize) -> u32;   
// }

fn is_visible(mtrx: &Compressed<u32>, x: usize, y: usize) -> u32
{

    let (x_max, y_max) = mtrx.dimensions();
    // if x == 0 || y == 0 || (x + 1) == x_max || (y + 1) == y_max
    // {
    //     return true;
    // } 

    let hight = dbg!(mtrx.get((x, y)));

    let mut left: Vec<_> = Vec::new();
    let mut right: Vec<_> =  Vec::new();
    let mut bottom: Vec<_> =  Vec::new();
    let mut top: Vec<_> =  Vec::new();

    for ix in (0..x).rev()
    {
        top.push(mtrx.get((ix, y)));
    }

    for ix in (x + 1)..x_max
    {
        bottom.push(mtrx.get((ix, y)));
    }

    for jx in (0..y).rev()
    {
        left.push(dbg!(mtrx.get((x, jx))));
    }

    for jx in (y + 1)..y_max
    {
        right.push(mtrx.get((x, jx)));
    }

    println!("left: {left:?}");
    println!("right: {right:?}");
    println!("top: {top:?}");
    println!("bottom: {bottom:?}");

    let top_score = 
        (top).clone()
        .into_iter()
        .take_while(|el| (*el) < hight)
        .chain(
            top.clone()
            .into_iter()
            .skip_while(|el| (*el) < hight)
            .take(1)
        )
        .count();
    
    let bottom_score = 
        (bottom).clone()
        .into_iter()
        .take_while(|el| (*el) < hight)
        .chain(
            bottom.clone()
            .into_iter()
            .skip_while(|el| (*el) < hight)
            .take(1)
        )
        .count();

    let right_score = 
        (right).clone()
        .into_iter()
        .take_while(|el| (*el) < hight)
        .chain(
            right.clone()
            .into_iter()
            .skip_while(|el| (*el) < hight)
            .take(1)
        )
        .count();

    let left_score = 
        (left).clone()
        .into_iter()
        .take_while(|el| (*el) < hight)
        .chain(
            left.clone()
            .into_iter()
            .skip_while(|el| (*el) < hight)
            .take(1)
        )
        .count();
    
    println!("top_score: {top_score:?}");
    println!("bottom_score: {bottom_score:?}");
    println!("left_score: {left_score:?}");
    println!("right_score: {right_score:?}");
    
    (top_score * bottom_score * left_score * right_score) as u32
} 

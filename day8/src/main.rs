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

    let mut visibles = Vec::new();
    
    let (rows, cols) = mtrx.dimensions();

    for x in 0..rows
    {
        for y in 0..cols
        {
            visibles.push(mtrx.is_visible(x, y));
        }
    }

    println!("count: {}", visibles.into_iter().filter(|b| *b).count());

    // println!("x_len: {x_len}, y_len: {y_len}, mtrx: {mtrx:?}");
}

trait IsVisible {
    fn is_visible(&self, x: usize, y: usize) -> bool;   
}

impl<T> IsVisible for Compressed<T> 
where T: matrix::Element + Ord + Copy + Debug
{
    fn is_visible(&self, x: usize, y: usize) -> bool {
        let (x_max, y_max) = self.dimensions();
        if x == 0 || y == 0 || (x + 1) == x_max || (y + 1) == y_max
        {
            return true;
        } 

        let hight = dbg!(self.get((x, y)));

        let mut left: Vec<_> = Vec::new();
        let mut right: Vec<_> =  Vec::new();
        let mut bottom: Vec<_> =  Vec::new();
        let mut top: Vec<_> =  Vec::new();

        for ix in 0..x
        {
            top.push(self.get((ix, y)));
        }

        for ix in (x + 1)..x_max
        {
            bottom.push(self.get((ix, y)));
        }

        for jx in 0..y
        {
            left.push(dbg!(self.get((x, jx))));
        }

        for jx in (y + 1)..y_max
        {
            right.push(self.get((x, jx)));
        }

        println!("left: {left:?}");
        println!("right: {right:?}");
        println!("top: {top:?}");
        println!("bottom: {bottom:?}");

        // let mut is_visible = true;
        // loop {
            if top.into_iter().any(|el| el >= hight)
            {
                println!("top invisible");
                // is_visible = false;
            } else { return true; }
            
            if bottom.into_iter().any(|el| el >= hight)
            {
                println!("bottom invisible");
                // return true;
            } else { return true; }
            
            if left.into_iter().any(|el| el >= hight)
            {
                println!("left invisible");
                // return true;
            } else { return true; }
            
            if right.into_iter().any(|el| el >= hight)
            {
                println!("right invisible");
                // return true;
            } else { return true; }

            // break;
        // }


        // for ix in 0..x_max
        // {
        //     for jy in 0..y_max
        //     {
        //         if (self.get(ix, jy) )
        //     }
        // }

        // todo!()
        false
    }
}
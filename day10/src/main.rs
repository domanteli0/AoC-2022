use std::str::{FromStr, Lines};

use nom::{sequence::preceded, combinator::map, bytes::complete::{tag, take_while}, IResult, branch::alt, Finish};

fn main() {
    const STR: &str = include_str!("../input.txt");

    let instrs =
        STR
        .lines()
        .map(Ins::from_str)
        .map(|e| e.unwrap());
        // .collect::<Vec<_>>();

    // dbg!(instrs);

    let mut cycles: Vec<i32> = vec![1];
    // let mut regx: isize = 1;

    for inst in instrs
    {
        // dbg!((inst, cycles.last()));
        match inst {
            Ins::Iaddx(x) => {
                dbg!(x);
                let temp_add: i32 = dbg!(*cycles.last().unwrap_or(&1_i32));
                cycles.push(TryInto::try_into(temp_add).unwrap());
                cycles.push(dbg!( temp_add + x))
            },
            Ins::Inoop => cycles.push(*cycles.last().unwrap_or(&1_i32)),
        }
    }

    let cycles_ =cycles.clone();
    let mut screen = vec![];
    let lines = cycles_.chunks(40);
    for line in lines
    {
        if line.len() != 40 { break; }
        // println!("line: {line:?}");
        let mut line_str = String::with_capacity(40);
        for (pos, ch) in line.iter().enumerate()
        {
            match (pos as i32).is_near(ch) {
                true => line_str += "#",
                false => line_str += ".",
            }
            // println!("pos: {pos}, ch: {ch}, is_near: {}", (pos as i32).is_near(ch));
        }
        println!("{line_str}");
        screen.push(line_str);
    }
    
    // const CMP: &str = include_str!("../cmp.txt");
    // let mut ITER: Lines<'_> = CMP.lines();
    // let mut iter = screen.iter();
    // while let Some(line) = ITER.next() {
    //     if let Some(line_mine) = iter.next() {
    //         assert_eq!(line, line_mine);
    //     }
    // }


    // dbg!(cycles.clone().into_iter().enumerate().collect::<Vec<_>>());

    // let mut cycle = 20;
    // let mut scores = Vec::new();
    // let iter = &cycles[..];

    // while let Some(x) = iter.get(cycle) {
    //     scores.push((cycle, *x));
    //     cycle += 40;
    // }

    //     match iter.get(cycle) {
    //         Some(x) => {
                
    //         },
    //         None => break,
    //     }
    // }

    // let scores = scores.clone();

    // dbg!(scores.clone());
    // println!("Total: {:?}",
    //     scores
    //         .into_iter()
    //         .map(|(x1, x2)| (x1 as isize) * x2)
    //         .sum::<isize>()
    // );
}

trait IsNear {
    fn is_near(&self, other: &Self) -> bool;
}

impl IsNear for i32
{
    fn is_near(&self, other: &Self) -> bool {
        (*self <= (*other) + 1) && (*self >= *other - 1)
    }
}

#[derive(Debug, Clone, Copy)]
enum Ins
{
    Iaddx(i32),
    Inoop,
}

impl FromStr for Ins
{
    type Err = nom::error::Error<&'static str>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(alt((parse_noop, parse_addx))(s).finish().unwrap().1)
    }
}

fn parse_addx(str: &str) -> IResult<&str, Ins>
{
    map(
        preceded(tag("addx "), take_while(|_| true)),
        |s: &str| Ins::Iaddx(s.parse::<i32>().unwrap())
    )(str)
}

fn parse_noop(str: &str) -> IResult<&str, Ins>
{
    map(
        tag("noop"),
        |_| -> Ins {Ins::Inoop}
    )(str)
}
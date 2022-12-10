use std::str::FromStr;

use nom::{sequence::preceded, combinator::map, bytes::complete::{tag, take_while}, IResult, branch::alt, Finish};

fn main() {
    const STR: &str = include_str!("../input.txt");

    let mut instrs =
        STR
        .lines()
        .map(Ins::from_str)
        .map(|e| e.unwrap());
        // .collect::<Vec<_>>();

    // dbg!(instrs);

    let mut cycles: Vec<isize> = vec![1, 1];
    // let mut regx: isize = 1;

    for inst in instrs
    {
        // dbg!((inst, cycles.last()));
        match inst {
            Ins::Iaddx(x) => {
                let temp = *cycles.last().unwrap_or(&1_isize);
                cycles.push(temp);
                cycles.push(temp + x)
            },
            Ins::Inoop => cycles.push(*cycles.last().unwrap_or(&1_isize)),
        }
        // inst.unwrap()
        // match inst {
        //     Ok(_) => todo!(),
        //     Err(_) => todo!(),
        // }
    }

    // dbg!(cycles.clone());

    let mut cycle = 20;
    let mut scores = Vec::new();
    let iter = &cycles[..];

    while let Some(x) = iter.get(cycle) {
        scores.push((cycle, *x));
        cycle += 40;
    }

    //     match iter.get(cycle) {
    //         Some(x) => {
                
    //         },
    //         None => break,
    //     }
    // }

    // let scores = scores.clone();

    dbg!(cycles.clone().into_iter().enumerate().collect::<Vec<_>>());
    dbg!(scores.clone());
    println!("Total: {:?}",
        scores
            .into_iter()
            .map(|(x1, x2)| (x1 as isize) * x2)
            .sum::<isize>()
    );
}

#[derive(Debug, Clone, Copy)]
enum Ins
{
    Iaddx(isize),
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
        |s: &str| Ins::Iaddx(s.parse::<isize>().unwrap())
    )(str)
}

fn parse_noop(str: &str) -> IResult<&str, Ins>
{
    map(
        tag("noop"),
        |_| -> Ins {Ins::Inoop}
    )(str)
}
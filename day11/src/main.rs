use std::{str::FromStr};

use nom::{sequence::{preceded, delimited}, bytes::complete::{tag, take_until, take_till, take_while}, IResult, combinator::map, Finish};

fn main() {
    const STR: &str = include_str!("../input.txt");
    let mut monkeys = STR
        .split("\n\n")
        .map(|monk| -> Result<Monkey, _> {
                println!("{}", monk);
                Monkey::from_str(monk)
        })
        .filter_map(|e| e.ok())
        .collect::<Vec<_>>();

    let mut inspections = vec![0; monkeys.len()];

    // let new_monkeys = Vec::new();
    for _ in 0..20 {
        for m_ix in 0..monkeys.len() {
            // let mut m = &mut monkeys[m_ix];
    
            while !monkeys[m_ix].items.is_empty() {
                inspections[m_ix] += 1;

                let i = dbg!(monkeys[m_ix].items.remove(0));
    
    
                let new_size = match monkeys[m_ix].op_ch {
                    '*' => const_or_id(monkeys[m_ix].op_arg1.clone())(i) * const_or_id(monkeys[m_ix].op_arg2.clone())(i),
                    '+' => const_or_id(monkeys[m_ix].op_arg1.clone())(i) + const_or_id(monkeys[m_ix].op_arg2.clone())(i),
                    _ => unreachable!()
                } / 3;
    
                match new_size % (monkeys[m_ix].div_by as isize) == 0 {
                    true => {
                        let index = monkeys[m_ix].if_true;
                        monkeys[index].items.push(new_size)
                    },
                    false => {
                        let index = monkeys[m_ix].if_false;
                        monkeys[index].items.push(new_size)
                    },
                }
            }
        }
        monkeys.iter().for_each(|m| println!("{:?}", m.items));
    }
    inspections.sort_by(|a, b| b.cmp(a));
    dbg!(&inspections);
    println!("Score: {}", inspections[0] * inspections[1]);

    // for mut m in monkeys {
    //     let op_add = |num| (const_or_id(m.op_arg1))(num) + const_or_id(m.op_arg2.clone())(num);
    //     // let op_mul = |num| (const_or_id(m.op_arg1.clone())(num) * const_or_id(m.op_arg2.clone())(num));

    //     match m.op_ch {
    //         '+' => m.op = Some(Box::new(op_add)),
    //         // '*' => m.op = Some(Box::new(op_mul)),
    //         _ => unreachable!()
    //     };
    // }


    
}

#[derive(Clone)]
struct Monkey
{
    id: usize,
    items: Vec<isize>,
    op_arg1: String,
    op_arg2: String,
    op_ch: char,
    div_by: usize,
    if_true: usize,
    if_false: usize,
}

impl std::fmt::Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f
            .debug_struct(&&format!("Monkey {}", self.id)[..])
            .field("starting_items", &self.items)
            .field("arg1", &self.op_arg1)
            .field("arg2", &self.op_arg2)
            .field("op", &self.op_ch)
            .field("div_by", &self.div_by)
            .field("if_true", &self.if_true)
            .field("if_false", &self.if_false)
            .finish()
    }
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.lines();
        let id = parse_monkey_id(iter.next().unwrap()).finish().unwrap().1;
        let starting_items = parse_starting_items(iter.next().unwrap()).finish().unwrap().1;
        let (op_arg1, op_ch, op_arg2): (String, char, String) = parse_op(iter.next().unwrap()).finish().unwrap().1;
        let div_by = parse_div_by(iter.next().unwrap()).finish().unwrap().1;

        Ok(Monkey { 
            id, 
            items: starting_items,
            div_by,
            op_arg1,
            op_arg2,
            op_ch,
            if_true: parse_if_true(iter.next().unwrap()).unwrap().1, 
            if_false: parse_if_false(iter.next().unwrap()).unwrap().1, 
        })
    }
}

// enum OldOrNum {Old, Num(isize)}

// impl FromStr for OldOrNum {
//     type Err = ();

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s {
//             "old" => Ok(OldOrNum::Old),
//             s => match s.parse::<isize>() {
//                 Ok(i) => Ok(OldOrNum::Num(i)),
//                 _ => unreachable!()
//             } 
//         }
//     }
// }

fn const_or_id(s: String) -> impl Fn(isize) -> isize {
    move |i| {
        match &s[..] {
            "old" => i,
            s => s.clone().trim().parse::<isize>().unwrap()
        }
    }
}

// struct FuncWrapper { func: dyn Fn(isize) -> isize }

// fn get_op_add(str1: &str, str2: &str) -> impl Fn(isize) -> isize {

//     move |num| (const_or_id(String::from(str1))(num) * const_or_id(String::from(str2))(num))
// }

fn parse_op(s: &str) -> IResult<&str, (String, char, String)> {

    map(
        preceded(tag("  Operation: new = "), take_while(|_| true)),
        |s: &str| {
            let mut iter = s.split(' ');
            (
                iter.next().unwrap().to_owned(), // num1
                iter.next().unwrap().chars().next().unwrap(), // op
                iter.next().unwrap().to_owned(), // num2
            )

            // let op_mul = |num1: isize, num2: isize| num1 * num2;
            // let op_mul = FuncWrapper { func: Box::new( |num| (const_or_id(num1_str).as_ref()(num) * const_or_id(num2_str).as_ref()(num))) };
            // let op_add = FuncWrapper { func: Box::new( |num| (const_or_id(num1_str).as_ref()(num) + const_or_id(num2_str).as_ref()(num))) };
            // let op_add = Box::new( |num| (const_or_id(num1_str).as_ref()(num) + const_or_id(num2_str).as_ref()(num)));
            // let op_add = |num1: isize, num2: isize| num1 + num2;
            // let op_fac = |op: bool| if op { op_mul } else { op_add }; 

            // let op_mul = |num| (const_or_id(num1_str).as_ref()(num) * const_or_id(num2_str).as_ref()(num));
            
            // let num1 = String::from(num1_str);
            // let num2 = String::from(num2_str);


            // match op_str {
            //     "*" => 
            //         move |num| (const_or_id(num1)(num) * const_or_id(num2)(num)), 
            //     // "*" => get_op_add(num1_str, num2_str),
            //     // "*" => Box::new(FuncWrapper { func: |num| (const_or_id(num1_str).as_ref()(num) * const_or_id(num2_str).as_ref()(num)) }),
            //     // "+" => FuncWrapper { func: Box::new( |num| (const_or_id(num1_str).as_ref()(num) + const_or_id(num2_str).as_ref()(num))) },
            //     // "+" => Box::new(|num| const_or_id(num1_str).as_ref()(num) + const_or_id(num2_str).as_ref()(num)),
            //     _ => unreachable!()
            // }
        }
    )(s)
}

fn parse_fac<'a>(s: &'a str) -> impl FnMut(&'a str) -> IResult<&str, usize> {
    map(
        preceded(tag(s), take_while(|_| true)),
        |s: &str| s.trim().parse::<usize>().unwrap()
    )
}

fn parse_div_by(s: &str) -> IResult<&str, usize> {
    parse_fac("  Test: divisible by ")(s)
}

fn parse_if_true(s: &str) -> IResult<&str, usize> {
 
    parse_fac("    If true: throw to monkey ")(s)
}
fn parse_if_false(s: &str) -> IResult<&str, usize> {
    parse_fac("    If false: throw to monkey ")(s)
}

fn parse_starting_items(s: &str) -> IResult<&str, Vec<isize>> {

    map(
        preceded(tag("  Starting items: "), take_till(|ch| ch == '\n')),
        |vec: &str| -> Vec<isize> { vec.split(", ").map(|e| e.trim().parse::<isize>().unwrap()).collect::<Vec<_>>() }
    )(s)
}

fn parse_monkey_id(s: &str) -> IResult<&str, usize> {
    map(
        delimited(tag("Monkey "), take_until(":"), tag(":")),
        |s: &str| s.parse::<usize>().unwrap()
    )(s)
}

use std::collections::VecDeque;

fn main() {
    let str = include_str!("../input.txt");

    // let mut stacks: Vec< VecDeque<char>> > = Vec::new();
    // let mut stacks: Vec<VecDeque<char>> = Vec::from([VecDeque::new()]);
    let mut stacks: Vec<VecDeque<char>> = Vec::new();

    let stack_list = 
        str.lines()
            .find(|s| s.starts_with(" 1 ") )
            .unwrap();

    stack_list.split("  ").for_each(|el| {
        let _conv = el.trim().parse::<i32>().unwrap();
        println!("'{el}', '{_conv}'");
        stacks.push(VecDeque::new());
    } );

    let str_ = str.split("\n 1").into_iter().next().unwrap();
    let str__ = str_.lines().rev();

    for s in str__
    {
        println!("s: '{s}'");
        let mut str = String::from(s);
        str.push(' ');

        let mut ix = 0;
        // let size = stacks.len() - 1;
        // println!("size: {size}");

        while ix < (stacks.len())
        {
            if let Some(val) = str.next_crate() {
                stacks[ix].push_front(val)
            }

            ix += 1;
        }   
    }

    // println!("S: {stacks:#?}");
    
    
    // stacks.move_crate(2, 1, 1);
    // println!("S: {stacks:#?}");
    // stacks.move_crate(1, 3, 3);
    // stacks.move_crate(2, 1, 2);
    // stacks.move_crate(1, 2, 1);

    // println!("S: {stacks:#?}");
    

    for s in str.lines()
    {
        if s.starts_with("move ")
        {
            let s = s.strip_prefix("move ").unwrap();
            let (times_str, rest_str) = 
                s.split_once(' ')
                    .unwrap();
            
            let rest_str_ = rest_str.strip_prefix("from ").unwrap();
            let (src_str, rest_str__) = rest_str_.split_once(' ').unwrap();
            let dst_str = rest_str__.strip_prefix("to ").unwrap().trim();
            // parse::<i32>().unwrap();
            println!("{times_str}, {src_str}, {dst_str}");
            stacks.move_crate(
                src_str.parse().unwrap(),
                dst_str.parse().unwrap(),
                times_str.parse().unwrap()
            );
        }
    }

    let mut top = Vec::new();
    for mut stack in stacks
    {
        top.push(stack.pop_front());
    }

    println!("{top:?}");
    // for line in str.lines()
    // {
        
    // }

    // println!("Hello, world!");
}

trait NextCrate
{
    fn next_crate(&mut self) -> Option<char>;
}

trait Move
{
    fn move_crate(&mut self, src: usize, dst: usize, times: usize);
}

impl Move for Vec<VecDeque<char>> {
    fn move_crate(&mut self, src: usize, dst: usize, times: usize)
    {
        for _ in 0..times
        {
            let item = self[src - 1].pop_front().unwrap();
            self[dst - 1].push_front(item);
        }
    }
}

impl NextCrate for String
{
    fn next_crate(self: &mut String ) -> Option<char> {
        
        let mut slice = String::new();
        slice.clone_from(&String::from(&self[0..=3]));

        self.clone_from(&String::from(&self[4..]));

        println!("self: '{self}', slice: '{slice}'");
        
        if let Some(val) = slice.trim().strip_prefix('[') {
            return val.strip_suffix(']').unwrap().chars().next();
        }

        None
    }
}
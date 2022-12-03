use std::{fs, collections::HashMap};

fn main() {
    let strs = fs::read_to_string("input.txt")
        .expect("Failed to read input.txt");

    let mut common: Vec<char> = Vec::new();

    let mut str = strs.split('\n');

    let count = str.clone().count() / 3;
    let mut ix = 0;

    let mut groups: Vec<(&str, &str, &str)> = Vec::new();

    while ix < count
    {
        groups.insert(0, (
            str.next().expect("failed to group").clone(),
            str.next().expect("failed to group").clone(),
            str.next().expect("failed to group").clone(),
        ));
        
        ix += 1;
    }

    for (o, s, p) in groups
    {
        let mut group_common = Vec::new();
        println!("group: {o}, {s}, {p} ");
        'lol: for ox in o.chars()
        {
            for sx in s.chars()
            {
                for px in p.chars()
                {
                    if ox == sx && px == sx && ox == px
                    {
                        if !group_common.contains(&ox)
                        {
                            group_common.insert(0, ox);
                            continue 'lol;
                        }
                    }
                }
            }
        }

        group_common.into_iter().for_each(|gc| -> () { common.insert(0, gc); } )
    }

    common.clone().into_iter().for_each(|c| -> () {println!("common: {c}");});

    // let sum = 'Z'.priotity();
    let sum = common.into_iter().fold(0, |acc, el| -> u32 {acc + el.priotity()});
    println!("sum: {sum}");
    
}

impl Priority for char {
    fn priotity(self: char) -> u32
    {
        if self.is_ascii_lowercase()
        {
            let i: u32 = self.into();
            i - 96
        } else {
            let i: u32 = self.into();
            i - 38
            // self.to_digit(0).expect("failed priority")
        }
    }
}

trait Common {
    fn find_common(&self, other: &Self) -> Vec<char>;
}

impl Common for str {
    fn find_common(&self, other: &Self) -> Vec<char>
    {
        let mut vec: Vec<char> = Vec::new();

        for o in self.chars()
        {
            for s in other.chars()
            {
                if o == s
                {
                    if !vec.contains(&o)
                    {
                        vec.insert(0, o);
                    }
                }
            }
        }

        vec
    }
}

trait Priority {
    fn priotity(self) -> u32;
}
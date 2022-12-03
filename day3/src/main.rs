use std::{fs, collections::HashMap};

fn main() {
    let strs = fs::read_to_string("input.txt")
        .expect("Failed to read input.txt");

    let mut common: Vec<char> = Vec::new();

    for str in strs.split('\n').into_iter()
    {
        let (fst, snd) = str.split_at(str.len() / 2);
        println!("first: {fst}, second: {snd}");

        let com = fst.find_common(snd);

        for el in com.into_iter()
        {
            common.insert(0, el);
        }
    }

    // let sum = 'Z'.priotity();
    let sum = common.into_iter().fold(0, |acc: u32, x: char| -> u32 {acc + x.priotity()});
    println!("prio: {sum}");
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
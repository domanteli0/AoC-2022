fn main() {
    let str = include_str!("../input.txt");

    for line in str.lines()
    {
        let mut pos = 0;
        'foo: for (ix, _) in line.chars().enumerate()
        {
            if (dbg!(line[ix..=(ix+13)].made_up_of_unique()))
            {
                pos = dbg!(ix + 14);
                break 'foo;
            }
        }
    }


    // let foo = "fdafs".made_up_of_unique();

    // println!("foo: {foo}");
}

trait FindMarker {
    fn find_marker(&self) -> usize;
}

trait MadeUpOfUnique {
    fn made_up_of_unique(&self) -> bool;
}

impl MadeUpOfUnique for str
{
    fn made_up_of_unique(&self) -> bool
    {
        let mut r#match = true;
        'foo: for (ix, s) in self.chars().enumerate()
        {
            for (jx, ss) in self.chars().enumerate()
            {
                if ix != jx && s == ss {
                    r#match = false;
                    break 'foo;
                }
            }
        }
        
        r#match
    }
    
}

impl FindMarker for str
{
    fn find_marker(&self) -> usize
    {


        todo!()
    }

    // fn next_crate(self: &mut String ) -> Option<char> {
        
    //     let mut slice = String::new();
    //     slice.clone_from(&String::from(&self[0..=3]));

    //     self.clone_from(&String::from(&self[4..]));

    //     println!("self: '{self}', slice: '{slice}'");
        
    //     if let Some(val) = slice.trim().strip_prefix('[') {
    //         return val.strip_suffix(']').unwrap().chars().next();
    //     }

    //     None
    // }
}
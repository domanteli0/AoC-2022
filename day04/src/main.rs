use std::{fs, str::FromStr, fmt::Debug};

#[derive(Debug)]
struct Range<T>(T, T)
    where  T: Ord;

#[derive(Debug)]
pub enum Error {
    // InvalidRangeSyntax(String)
}

impl<T> FromStr for Range<T>
where 
    T: FromStr + Debug + Ord,
    <T as FromStr>::Err: Debug
{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split('-');

        Ok(Range(
            iter.next().expect("failed to iter in from_str")
                .parse().expect("failed to convert to i32"), 
            iter.next().expect("failed to iter in from_str")
                .parse::<T>().expect("failed to convert to i32"), 
        ))
    }
}

fn main() {
    let strs = fs::read_to_string("input.txt")
        .expect("Failed to read input.txt");
    
    let mut pairs: Vec<(Range<i32>, Range<i32>)> = Vec::new(); 

    for line in strs.split("\n")
    {
        if line.len() > 0
        {
            let mut iter = line.split(',');
            let fst = iter.next().expect("failed to next on ',' split");
            let snd = iter.next().expect("failed to next on ',' split");
    
            // println!("fst: {fst}, snd: {snd}");
            
            pairs.push((
                fst.parse().expect("failed to parse"),
                snd.parse().expect("failed to parse"),
            ));
        }    
    }
    (&pairs).into_iter().for_each(
        |el| -> () {println!("pair: {el:?}")}
    );

    let filtered = (&pairs)
        .into_iter()
        .filter(|el| el.contains_range())
        .collect::<Vec<_>>();
    
    (&filtered)
        .into_iter()
        .for_each(
            |el| -> () {println!("filtered: {el:?}")}
        );

    let sum = filtered.into_iter().count();

    println!("Count: {sum}");
}

trait ContainsRange
{
    fn contains_range(&self) -> bool;
}

impl<T> ContainsRange for (Range<T>, Range<T>)
where T: Ord
{
    fn contains_range(&self) -> bool
    {
        let fst = &self.0;
        let snd = &self.1;

        if  (fst.1 >= snd.0 && fst.0 <= snd.1)
        // || 
        // (fst.1 >= snd.0 && fst.0 <= snd.1)
        {
            true
        } else if (snd.1 >= fst.0 && snd.0 <= fst.1)
        {
            true
        } else 
        {
            false
        }
    }

}
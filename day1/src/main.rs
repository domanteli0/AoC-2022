use std::fs;

// fn main() -> Result<(), Error> 
fn main()
{
    let str = fs::read_to_string("input.txt")
        .expect("Failed to read input.txt");
    
    let mut calories: Vec<u32> = Vec::new();

    for cals in str.split("\n\n").into_iter()
    {
        // println!("{cals:?}");
        let cals = cals.split("\n")
            .into_iter()
            .fold(0, { |acc, number| -> u32 {
                match number.parse::<u32>() {
                    Ok(num) => acc + num,
                    _ => acc
                }
        }});

        calories.push(cals);

    }

    println!("{calories:?}");

    let max = (&calories).into_iter().max().expect("failed to find max");
    let elf = 
        (&calories).into_iter()
        .position(|el| -> bool {el == max}).expect("failed to position max");
    println!("max cal elf: {elf}, max: {max}");

}


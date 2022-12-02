use std::fs;

fn main()
{
    let str = fs::read_to_string("input.txt")
        .expect("Failed to read input.txt");
    
    let mut calories: Vec<u32> = Vec::new();

    for cals in str.split("\n\n").into_iter()
    {
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
    
    let max1 = *(&calories).into_iter().max().expect("failed to find max");
    let elf1 = 
    (&calories).into_iter()
    .position(|el| -> bool {*el == max1}).expect("failed to position max");
    calories.remove(elf1);
    
    println!("{calories:?}");
    
    let max2 = *(&calories).into_iter().max().expect("failed to find max");
    let elf2 = 
    (&calories).into_iter()
    .position(|el| -> bool {*el == max2}).expect("failed to position max");
    calories.remove(elf2);

    println!("{calories:?}");
    
    let max3 = *(&calories).into_iter().max().expect("failed to find max");
    let elf3 = 
        (&calories).into_iter()
        .position(|el| -> bool {*el == max3}).expect("failed to position max");
    calories.remove(elf3);
    

    
    // let max2 = (&calories).into_iter().max().expect("failed to find max");
    // let elf2 = 
    //     (&calories).into_iter()
    //     .position(|el| -> bool {el == max1}).expect("failed to position max");

    // calories

    let sum = max1 + max2 + max3;
    
    println!("sum: {sum}");
    println!("max cal elf: {elf1}, max: {max1}");
    println!("max cal elf: {elf2}, max: {max2}");
    println!("max cal elf: {elf3}, max: {max3}");

}


use std::{str::FromStr, collections::HashSet};


#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Point
{
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Right(usize),
    Left(usize),
    Up(usize),
    Down(usize),
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut str = s.split(' ');
        let s1 = str.next();
        let s2: usize = str.next().unwrap().parse().unwrap();
        match s1 {
            Some("R") => Ok(Move::Right(s2)),
            Some("L") => Ok(Move::Left(s2)),
            Some("U") => Ok(Move::Up(s2)),
            Some("D") => Ok(Move::Down(s2)),
            _ => Err(())

        }
    }
}


#[derive(Debug, Default)]
struct Board
{
    head: Point,
    tails: Vec<Point>,
    history: HashSet<Point>
}

fn main() {
    const STR: &str = include_str!("../input.txt");

    let mut board = dbg!(Board::default());
    board.history.insert(Point::default());
    for _ in 0..9
    {
        board.tails.push(Point::default());
    }

    // let moves: _ = 
    STR
        .lines()
        .map(|e| e.parse().unwrap())
        .for_each(|mov| board.move_head(mov));

    dbg!(&mut board);
    
    println!("Count: {}", board.history.len());

}

impl Board {
    fn move_head(&mut self, mov: Move)
    {
        match mov {
            Move::Right(times) => {
                for _ in 0..times
                {
                    self.head.x += 1;
                    self.move_tail();
                }
            },
            Move::Left(times) => {
                for _ in 0..times
                {
                    self.head.x -= 1;
                    self.move_tail();
                }
            },
            Move::Up(times) => {
                for _ in 0..times
                {
                    self.head.y += 1;
                    self.move_tail();
                }
            },
            Move::Down(times) => {
                for _ in 0..times
                {
                    self.head.y -= 1;
                    self.move_tail();
                }
            },
        }
    }

    fn move_tail(&mut self)
    {
        let mut prev = self.head;
        let iter = &mut self.tails[..];
        for tail in iter
        {
            if !Point::is_near(&prev, tail)
            {
                if prev.x == tail.x
                {
                    println!("x == branch");
                    if tail.y < prev.y
                    {
                        tail.y += 1;
                    }
                    else {
                        tail.y -= 1;
                    }
                } else if prev.y == tail.y {
                    println!("y == branch");
                    if tail.x < prev.x
                    {
                        tail.x += 1;
                    }
                    else {
                        tail.x -= 1;
                    }
                } else if tail.x < prev.x && tail.y < prev.y {
                    println!("x <, y < branch");
                    tail.x += 1;
                    tail.y += 1;
                    // self.tail.y 
                } else if tail.x < prev.x && tail.y > prev.y {
                    println!("x <, y > branch");
                    tail.x += 1;
                    tail.y -= 1;
                    // self.tail.y 
                } else if tail.x > prev.x && tail.y < prev.y {
                    println!("x >, y < branch");
                    tail.x -= 1;
                    tail.y += 1;
                    // self.tail.y 
                } else if tail.x > prev.x && tail.y > prev.y {
                    println!("x >, y > branch");
                    tail.x -= 1;
                    tail.y -= 1;
                    // self.tail.y 
                }
    
            }

            prev = *tail;
        }

        self.history.insert(self.tails[8]);

        
        dbg!(&self.head);
        dbg!(&self.tails);
        println!();
    }
}

impl Point {
    fn is_near(&self, other: &Self) -> bool
    {
        let ep_x = self.x - other.x;
        let ep_y = self.y - other.y;
        (-1..=1).contains(&ep_x) && (-1..=1).contains(&ep_y)
    }
    
}
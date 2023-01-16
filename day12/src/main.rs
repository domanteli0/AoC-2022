use std::{str::FromStr, cell::RefCell};

fn main() {
    const STR: &str = include_str!("../input.txt");

    let grid: Grid = STR.parse::<Grid>().unwrap();

    println!("{:?}", grid);

    // println!("{:?}", grid.get_all_near_movable(grid.current));

    let binding = RefCell::new(Vec::new());
    let (_, _, sol) = solution(&grid, 0, &binding);
    let sol = sol.borrow().clone();
    println!("sol: {:?} {:?}", sol.iter().min(), sol);

}

fn solution<'a>(grid: &Grid, accum_cur: u32, accum: &'a RefCell<Vec<u32>>) -> (bool, u32, &'a RefCell<Vec<u32>>) {
    print!("a");
    let mut grid = grid.clone();

    if let Some(min) = accum.borrow().iter().min(){
        if *min <= accum_cur {
            return (false, accum_cur, accum);
        }
    }
    // dbg!(&grid);
    
    for possibility in grid.get_all_near_movable(grid.current).iter().rev() {
        // assert!(!(grid.dest.0 == possibility.pos.0 && grid.dest.1 == possibility.pos.1));
        if grid.dest.0 == possibility.pos.0 && grid.dest.1 == possibility.pos.1 {
            accum.borrow_mut().push(accum_cur + 1);
            return (true, accum_cur + 1, accum);
        } 

        grid.matrix[grid.current.0][grid.current.1] = u8::MAX;
        grid.current = possibility.pos;
        
        solution(&grid, accum_cur + 1, accum);
        // accum_cur = accum_cur_;
        // if let (true, result, vec) = solution(&grid, accum_cur + 1, accum) {
        //     return (true, result, vec);
        // }   
    }    

    (false, accum_cur + 1, accum)
}

#[derive(Debug, Clone)]
struct Grid {
    matrix: Vec<Vec<u8>>,
    current: (usize, usize),
    dest: (usize, usize),
}

#[derive(Debug, Clone, Copy)]
struct HeightedPosition {
    pos: (usize, usize),
    height: u8
}

impl From<HeightedPosition> for (usize, usize) {
    fn from(hp: HeightedPosition) -> Self {
        hp.pos
    }
}

impl HeightedPosition {
    fn can_move_to(&self, other: &Self) -> bool {
        if self.pos.0 == other.pos.0 && self.pos.1 == other.pos.1 { panic!("can_move_to with identical pos") }

        let dif = self.height as isize - other.height as isize;

        (-1..=1).contains(&dif)
    }

    fn from(pos: (usize, usize), h: u8) -> HeightedPosition{ 
        HeightedPosition {pos, height: h} 
    }
}

impl Grid {
    fn near_dest(&self) -> bool {
        self.get_current().can_move_to(&self.get_dest())
    }

    fn get_current(&self) -> HeightedPosition { 
        self.get(self.current)
    }

    fn get_dest(&self) -> HeightedPosition { 
        self.get(self.dest)
    }

    fn get(&self, (x, y): (usize, usize)) -> HeightedPosition { 
        HeightedPosition { pos: (x, y), height: self.matrix[x][y] } 
    }

    fn get_heighted(&self, (x, y) : (isize, isize)) -> Option<HeightedPosition> {
        if x < 0 || y < 0 { return None; }
        let row = self.matrix.get(x as usize)?;
        let height = row.get(y as usize)?;
        Some(HeightedPosition { pos: (x as _, y as _), height: *height })
    }

    fn get_all_near_movable(&self, (x, y): (usize, usize)) -> Vec<HeightedPosition> {
        const BEEN_THERE: u8 = u8::MAX;

        let mut ret = vec![];
        let current = self.get((x, y));

        let x: isize = x as _;
        let y: isize = y as _;


        ret.push(self.get_heighted((x + 1, y)));
        ret.push(self.get_heighted((x - 1, y)));
        ret.push(self.get_heighted((x, y - 1)));
        ret.push(self.get_heighted((x, y + 1)));

        let ret = ret
            .iter()
            .filter_map(|i| *i)
            .filter(|e| current.can_move_to(e))
            .filter(|e| (e.height) != BEEN_THERE)
            .collect();
        
        // dbg!(&ret);

        ret
        // ret.iter().filter_map(|i| *i).collect()
        
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        
    const RANG_BEGIN: u8 = b'a';
    const RANG_END: u8 = b'z';
    const S: u8 = b'S';
    const E: u8 = b'E';

    let temp = s
        .lines()
        .enumerate()
        .flat_map(|(ix, line)| 
            line
                .chars()
                .enumerate()
                .map(move |(jx, ch)| match ch {
                    'S' => Some((false, ix, jx)),
                    'E' => Some((true, ix, jx)),
                    _ => None,
                }))
        .flatten()
        .collect::<Vec<_>>();
    
    let start = temp.iter().filter_map(|(i, x, y)| if !*i { Some((*x,*y))} else {None}).collect::<Vec<_>>()[0];
    let dest= temp.iter().filter_map(|(i, x, y)| if *i { Some((*x,*y))} else {None}).collect::<Vec<_>>()[0];
    

    // dbg!(start);
    // dbg!(dest);

    // let matrix = ;
    let matrix = s
        .lines()
        .map(|line| -> Vec<_> { 
            dbg!(line);
            line.chars().map(|ch| -> u8 {
                match ch as u8 {
                RANG_BEGIN..=RANG_END => dbg!(ch as u8 - b'a'),
                S => dbg!(0),
                E => dbg!(b'z' - b'a'),
                _ => unreachable!()
            }}).collect::<Vec<u8>>()
        })
        .collect::<Vec<_>>();

    Ok(Grid { matrix, current: start, dest })
    }
}

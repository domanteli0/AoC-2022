use std::{cell::RefCell, rc::Rc, os::macos::raw::stat, collections::HashMap, fmt::Display};

use guid_create::GUID;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, is_not},
    combinator::{map, opt},
    sequence::{delimited, preceded, pair},
    Finish, IResult, error::ErrorKind,
};

#[derive(Debug)]
enum Item
{
    Dir(&'static str),
    File(&'static str, usize)
}

type Output = Vec<Item>;

fn get_commands(str: &'static str) -> Result<Vec<Command>, nom::Err<nom::error::Error<&str>>>
{

    let mut v = Vec::new();
    let mut str_ = str;
    // let mut next_s = "";

    loop {
        // let (next_i, maybe_c) = opt(preceded(tag("\n"), parse_ls_line))(i)?;
        let (next_i, maybe_c) = dbg!(opt(parse_command)(str_)?);
        match maybe_c {
            Some(c) => v.push(dbg!(c)),
            None => break,
        }
        str_ = next_i;
        // next_s = next_i;
        // i = dbg!(next_i);
    }

    dbg!( Ok(v) )
}


fn identity<T>(el: T) -> T { el }

fn parse_command(i: &'static str) -> IResult<&str, Command>
{
    if i.is_empty() {
        return 
        map(
            tag(",|<>!@#$%^&*("), 
            |_| -> _ {Command::Cd("SHOULD NOT HAPPEN")}
        )(i);
    }
    
    // dbg!(alt((map(parse_cd, identity), map(parse_ls, identity)))(i))
    dbg!(alt((map(parse_ls, identity), map(parse_cd, identity)))(i))
}

fn parse_ls(i: &'static str) -> IResult<&str, Command> 
{
    let (ret, str): (&str, &str) =
        dbg!(preceded(
            // pair(opt(tag("\n$ ls\n")), tag("$ ls\n")), 
            alt((tag("$ ls\n"), tag("\n$ ls\n"))), 
            take_while(|x| x != '$'),
        )(i)?);
    

    // let (mut i, c) = dbg!(parse_ls_line(str)?);
    let mut v = Vec::new();
    let mut str_ = str;
    // let mut next_s = "";

    loop {
        // let (next_i, maybe_c) = opt(preceded(tag("\n"), parse_ls_line))(i)?;
        let (next_i, maybe_c) = dbg!(opt(parse_ls_line)(str_)?);
        match maybe_c {
            Some(c) => v.push(dbg!(c)),
            None => break,
        }
        str_ = next_i;
        // next_s = next_i;
        // i = dbg!(next_i);
    }

    dbg!( Ok((ret, Command::Ls(dbg!(v)))) )
}

fn parse_cd(i: &'static str) -> IResult<&str, Command> {
    let cd_cmd = Command::Cd;

    map(
        preceded(alt((tag("$ cd "), tag("\n$ cd "))), take_while(|ch| ch != '\n')),
        cd_cmd   
    )(i)
}

fn parse_ls_line(i: &'static str) -> IResult<&str, Item> 
{
    alt((parse_file, parse_dir))(i)
}

fn parse_dir(i: &'static str) -> IResult<&str, Item> 
{
    // let to_dir = Item::Dir;
    let to_dir = |str| -> Item {
        Item::Dir(str)
    };

    map(
        preceded(alt((tag("dir "), tag("\ndir "))), take_while(|ch| ch != '\n')),
        // preceded(tag("dir "), take_while(|_| true)),
        to_dir
    )(i)
}

fn parse_file(i: &'static str) -> IResult<&str, Item> 
{
    if i.is_empty() || i.starts_with("dir") || i.starts_with("\ndir") || i == "\n"
    {
        return 
            map(
                tag(",|<>!@#$%^&*("), 
                |_| -> _ {Item::Dir("SHOULD NOT HAPPEN")}
            )(i);
    }

    dbg!(map(
        preceded(opt(tag("\n")),
        pair(
            take_while(|ch: char| ch.is_ascii_digit()),
            take_while(|ch: char| ch != '\n')
        )),
        |(size, str): (&'static str, &'static str)| -> _ {Item::File(dbg!(&str[1..]), dbg!(size).parse().unwrap())}
    )(i))
    
}

#[derive(Debug)]
enum Command {
    Cd(&'static str),
    Ls(Output),
}

#[derive(Debug, Clone)]
struct Node
{
    id: GUID,
    size: usize,
    name: &'static str,
    children: Vec<GUID>,
    parent: Option<GUID>
}

impl Node {
    fn is_folder(&self) -> bool
    {
        self.size == 0 && !self.children.is_empty()
    }
}

impl Node {
    fn size(&self, repo: &HashMap<GUID, Node>) -> usize
    {
        let mut accum = 0;
        for child_id in self.children.clone()
        {
            let child = repo.get(&child_id).unwrap();
            if child.is_folder()
            {
                accum += child.size(repo);
            } else {
                accum += child.size;
            }
        }

        accum
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} ({} bytes)", self.name, self.size))?;
        Ok(())
    }
}


fn main()
{
    const STR: &str = include_str!("../input.txt");
    // let fst = STR.lines().next().unwrap();
    println!("{:?}", get_commands(STR));
    // let mut root: Item;

    let mut iter = get_commands(STR).unwrap().into_iter();
    
    let mut repo: HashMap<GUID, Node> = HashMap::new(); 

    let mut root: _ =
    {
        let cmd = iter.next().unwrap();

        match cmd {
            Command::Cd(name) => {
                Node {
                    id: GUID::rand(),
                    size: 0,
                    name,
                    parent: None,
                    children: Vec::new()
                }
            },
            Command::Ls(_) => unreachable!(),
        }
    };
    
    let root_id = root.id;
    let mut state_id = root.id;
    repo.insert(root.id, root);

    while let Some(command) = iter.next()
    {
        match command {
            Command::Ls(output) => {
                for item in output
                {
                    let temp_id = GUID::rand();
                    match item {
                        Item::Dir(name) => {
                            let mut node = Node {
                                id: temp_id,
                                size: 0,
                                name,
                                children: Vec::new(),
                                parent: Some(state_id),
                            };
                            
                            // let temp = repo.cl;
                            repo.insert(temp_id, node);
                            repo
                                .get_mut(&state_id)
                                .unwrap()
                                .children
                                .push(temp_id);
                        },
                        Item::File(name, size) => {
                            let node = Node {
                                id: temp_id,
                                size,
                                name,
                                children: Vec::new(),
                                parent: Some(state_id),
                            };

                            repo.insert(temp_id, node);
                            repo
                                .get_mut(&state_id)
                                .unwrap()
                                .children
                                .push(temp_id);
                        },
                    }
                }
            },
            Command::Cd("..") => {
                let temp = repo.get(&state_id).unwrap().parent.expect("tired to cd .. on root");
                state_id = temp;
                
            },
            Command::Cd(dir) => {

                let temp = 
                    repo.get(&state_id).unwrap()
                    .children
                    .clone()
                    .into_iter()
                    .find(|e| -> bool {
                        repo.get(e).unwrap().name == dir
                    }).unwrap();
                state_id = temp;
            },
        }
    }

    // print_fs(&repo, &root_id);
    let root_size = repo.get(&root_id).unwrap().size(&repo);
    // println!("root size: {}", root_size);

    // let mut accum = { if root_size > 100_000 { 0 } else { root_size }};
    // let accum = 0;
    let space_needed = 30_000_000;
    let space_total = 70_000_000;
    let space_unused = space_total - root_size;
    let space_to_delete = space_needed - space_unused;

    let mut folders = Vec::new();
    for (_, item) in repo.clone()
    {
        if item.is_folder()
        {
            folders.push((item.name, item.size(&repo)));
        }
    }
    folders.sort_by(|(_, a), (_, b)| a.cmp(b));

    println!("Root size: {root_size}");
    println!("Space to delete: {space_to_delete}");
    println!("Total: {folders:?}");
    println!("Folder to delete: {:?}", 
        folders
            .into_iter()
            .find(|(_, e)| *e >= space_to_delete)
    );
    // wrong guess: 'lhjmzsl'
}

fn print_fs(repo: &HashMap<GUID, Node>, root_id: &GUID)
{
    let root = repo.get(root_id).unwrap();
    println!("{root}");
    for child_id in root.children.clone()
    {
        print_children(repo, &child_id, 2);

    }
    
}

fn print_children(repo: &HashMap<GUID, Node>, root_id: &GUID, ind: usize)
{
    let root = repo.get(root_id).unwrap();
    println!("{}{root}", 
        (0..ind)
        .into_iter()
        .fold(String::from(""), |acc, _| -> String {acc + " "})
    );
    for child_id in root.children.clone()
    {
        print_children(repo, &child_id, ind + 2);

    }
}
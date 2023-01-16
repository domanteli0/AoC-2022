use core::time;
use std::{str::FromStr, collections::{HashMap, BTreeMap}, thread::Thread, time::Duration};

fn main() {
    const STR: &str = include_str!("../input_simple.txt");
    let valves = STR
        .lines()
        .map(|l| l.parse::<Valve>().unwrap())
        .map(|v| (v.name.clone(), v)) 
        .collect::<BTreeMap<_, _>>();
    
        dbg!(&valves);

    println!("{:?}", 
        find_best("AA", &valves, 30)
    );
}

fn find_best(start: &str, repo: &BTreeMap<String, Valve>, time_left: u32) -> u32 {
    find_best_priv(start, repo, time_left)
}

fn find_best_priv(start: &str, repo: &BTreeMap<String, Valve>, time_left: u32) -> u32 {
    
    std::thread::sleep(Duration::new(0, 50000000));
    println!("BEFORE: ");
    dbg!(time_left);
    if time_left == 0 {
        println!("FINISHED");
        return repo.iter()
            .map(|(_, v)| v.pressure_release())
            .sum::<u32>();
    }
    println!("AFTER: ");
    dbg!(time_left);

    let mut new = repo.clone();
    
    let mut all_possibilities: Vec<u32> = Vec::new();

    for valve_into in new[start].leads_to.clone() {
        dbg!(&valve_into);
        match new[start].is_opened {
            Status::Closed => {
                all_possibilities.push(
                    find_best_priv(&valve_into, &new.clone(), time_left - 1)
                );

                if new[start].flow_rate != 0 {
                    new.get_mut(start).unwrap().is_opened = Status::OpenedForMins(time_left - 1);
                    if time_left > 1 {
                        all_possibilities.push(
                            find_best_priv(&valve_into, &new.clone(), time_left - 2)
                        );
                    }
                }
            },
            Status::OpenedForMins(_) => 
                all_possibilities.push(
                    find_best_priv(&valve_into, repo, time_left - 1)
                ),
        }
    }

    //
    *(all_possibilities.iter().max().unwrap())
    
    // {
        

    //     match current.is_opened {
    //         Status::Closed => {
    //             if current.flow_rate != 0 {
    //                 current.is_opened = Status::OpenedForMins(time_left - 1);
    //             }
    //             let mut accum = accum;
    //             accum.push(current.pressure_release());

    //             for leads in current.leads_to {

    //             }

    //         },
    //         Status::OpenedForMins(mins) => {
                
    //         },
    //     }

    //     // repo.iter()
    //     // .map(|v| v.find_best(repo, time_left - 2))
    //     // .collect::<Vec<_>>();
    // }
    
}

impl Valve {
    fn pressure_release(&self) -> u32 {
        match self.is_opened {
            Status::Closed => 0,
            Status::OpenedForMins(mins) => self.flow_rate * mins,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Status {
    Closed,
    OpenedForMins(u32)
}

#[derive(Debug, Clone)]
struct Valve {
    name: String,
    flow_rate: u32,
    leads_to: Vec<String>,
    is_opened: Status,
}

impl FromStr for Valve {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name = String::from(&s[6..=7]);

        let flow_rate = 
            s.split(" has flow rate=")
            .last().unwrap()
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>().parse::<u32>().unwrap();

        let leads_to;
        if s.contains("; tunnels lead to valves ") {
            leads_to =
                s.split("; tunnels lead to valves ")
                .last().unwrap()
                .split(", ")
                .map(String::from)
                .collect::<Vec<String>>();
            } else {
                leads_to = vec![
                    s.split("tunnel leads to valve ")
                    .last().unwrap().to_string()
                ];
            }
        

        Ok(Valve {
            name,
            flow_rate,
            leads_to,
            is_opened: Status::Closed,
        })
    }
}
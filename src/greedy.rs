// GREEDY


use std::f32::consts::E;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use itertools::{Itertools as it, enumerate};
use std::collections::HashMap;
use std::cmp::{min, max};
use std::time::{Duration, Instant};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Clone, Debug)]
struct Place {
    x: f64,
    y: f64,
    links : Vec<usize>,
}

impl Place
{
    fn new(x: f64, y: f64) -> Place
    {
        Place {x, y, links : vec![]}
    }

}
fn main() 
{
    //let end_pos : Place = Place::new(40.0, 40.0);
    // create positions from text file
    let mut places : Vec<Place> = vec![];
    if let Ok(lines) = read_lines("places.txt") 
    {
        for line in lines 
        {
            if let Ok(l) = line 
            {
                let temp = l.split(",").map(|x| x.parse::<f64>().unwrap()).collect::<Vec<f64>>();
                places.push(Place::new(temp[0], temp[1]));
            }
        }
    }

    let mut dist_vec : Vec<((usize, usize), f64)> = vec![];
    let items : Vec<usize> = (0..places.len()).collect();
    for perm in items.iter().combinations(2).unique() // time to get from 2nd place to 3rd is the same as 3rd to 2nd
    {
        let dist = ((places[*perm[0]].x - places[*perm[1]].x).powf(2.0) + (places[*perm[0]].y - places[*perm[1]].y).powf(2.0)).sqrt();
        dist_vec.push( ((*perm[0], *perm[1]), dist) );
    }
    let mut dist_hm : HashMap<(usize, usize), f64> = HashMap::new();
    for perm in items.iter().combinations(2).unique() // time to get from 2nd place to 3rd is the same as 3rd to 2nd
    {
        let dist = ((places[*perm[0]].x - places[*perm[1]].x).powf(2.0) + (places[*perm[0]].y - places[*perm[1]].y).powf(2.0)).sqrt();
        dist_hm.insert((min(*perm[0], *perm[1]), max(*perm[0], *perm[1])), dist);
    }
    
    let mut start = Instant::now();
    dist_vec.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let mut total_dist : f64 = 0.0;


    for ((from, to), dist) in dist_vec.clone()
    {
        if places[from].links.len() < 2 && places[to].links.len() < 2
        {
            let old_places = places.clone();
            places[from].links.push(to);
            places[to].links.push(from);
            if places[from].links.len() == 0 || !closed_loop(&places, from)
            {
                println!("{} -- {}", from, to);
                total_dist += dist;
            }
            else 
            {
                places = old_places;
            }
            
        }
    }

    let mut last_link : Option<usize> = None;
    for i in 0..places.len()
    {
        if places[i].links.len() == 1
        {
            if last_link.is_none()
            {
                last_link = Some(i);
            }
            else 
            {
                places[last_link.unwrap()].links.push(i);
                places[i].links.push(last_link.unwrap());
                total_dist += dist_hm.get(&(min(last_link.unwrap(), i), max(last_link.unwrap(), i))).unwrap();
            }
        }
        println!("{}: {:?}", i, places[i].links);
    }
    println!("total dist: {total_dist}");

    


}

fn closed_loop(places : &Vec<Place>, mut index : usize) -> bool
{
    let mut visited : Vec<usize> = vec![];

    loop
    {
        //println!("{}", index);
        if places[index].links.len() == 1
        {
            return false
        }

        if visited.contains(&places[index].links[0]) && visited.contains(&places[index].links[1])
        {
            return true
        }

        let temp = visited.clone();
        visited.push(index);
        if temp.contains(&places[index].links[0])
        {
            index = places[index].links[1];
        }
        else 
        {
            index = places[index].links[0];
        }
    }
}
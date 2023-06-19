// NEAREST NEIGHBOUR
//TODO TEST MORE


use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use itertools::Itertools as it;
use std::collections::HashMap;
use std::cmp::{min, max};
use rayon::prelude::*;
use std::time::{Duration, Instant};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Place {
    x: f64,
    y: f64,
}

impl Place
{
    fn new(x: f64, y: f64) -> Place
    {
        Place {x, y}
    }

}
fn main() 
{
    let mut start = Instant::now();
    let start_pos : Place = Place::new(0.0, 0.0);
    //let end_pos : Place = Place::new(40.0, 40.0);
    // create positions from text file
    let mut places : Vec<Place> = vec![];
    places.push(start_pos);
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
    //places.push(end_pos);

    let mut dist_vec : Vec<((usize, usize), f64)> = vec![];
    let mut items : Vec<usize> = (0..places.len()).collect();
    for perm in items.iter().permutations(2).unique() // time to get from 2nd place to 3rd is the same as 3rd to 2nd
    {
        let mut dist = ((places[*perm[0]].x - places[*perm[1]].x).powf(2.0) + (places[*perm[0]].y - places[*perm[1]].y).powf(2.0)).sqrt();
        dist_vec.push( ((*perm[0], *perm[1]), dist) );
    }


    let mut start = Instant::now();
    dist_vec.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let mut total_dist : f64 = 0.0;
    let mut path = vec![];
    let mut current : usize = 0;
    for i in 0..places.len()
    {
        for j in dist_vec.iter()
        {
            if j.0.0 == current && j.0.1 != 0 // make sure we dont go back to the start point prematurely
            {
                let last = current.to_owned();
                total_dist += j.1;
                path.push(current);
                current = j.0.1;
                println!("{} -> {} : {}m", last, current, j.1);
                if j.0.0 != 0
                {
                    dist_vec.retain(|x| x.0.1 != last);
                }
                break;
            }
        }
    }
    dist_vec.retain(|x| x.0.0 == current && x.0.1 == 0);
    path.push(current);

    total_dist += dist_vec[0].1; // add the distance from the last place to the start
    println!("{} -> 0 : {}m", current, dist_vec[0].1);
    path.push(0);
    println!("dist : {}m path: {:?}",total_dist, path);
    
    println!("took {} seconds to complete", start.elapsed().as_secs_f64());
}
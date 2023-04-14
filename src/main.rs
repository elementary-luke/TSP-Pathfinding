// BRUTE FORCE METHOD


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

    let mut dist_hm : HashMap<(usize, usize), f64> = HashMap::new();
    let mut items : Vec<usize> = (1..places.len()-1).collect();
    for perm in items.iter().combinations(2).unique() // time to get from 2nd place to 3rd is the same as 3rd to 2nd
    {
        let mut dist = ((places[*perm[0]].x - places[*perm[1]].x).powf(2.0) + (places[*perm[0]].y - places[*perm[1]].y).powf(2.0)).sqrt();
        dist_hm.insert((min(*perm[0], *perm[1]), max(*perm[0], *perm[1])), dist);
    }

    let mut worst_dist : f64 = 0.0;
    let mut best_dist : f64= 1000000.0;
    for(i, perm) in items.iter().permutations(items.len()).unique().enumerate()
    {
        let mut total_dist = 0.0;
        for i in 0..perm.len()-1
        {
            total_dist += dist_hm.get(&(min(*perm[i], *perm[i+1]), max(*perm[i], *perm[i+1]))).unwrap();
        }
        total_dist += ((places[*perm[0]].x - places[0].x).powf(2.0) + (places[*perm[0]].y - places[0].y).powf(2.0)).sqrt(); // start to first place
        total_dist += ((places[*perm[1]].x - places[0].x).powf(2.0) + (places[*perm[1]].y - places[0].y).powf(2.0)).sqrt(); // last place to start
        if total_dist < best_dist
        {
            best_dist = total_dist;
        }
        if total_dist > worst_dist
        {
            worst_dist = total_dist;
        }
    }
    println!("best {}m, \n worst: {}m", best_dist, worst_dist);
    println!("took {} seconds to complete", start.elapsed().as_secs_f64());
}
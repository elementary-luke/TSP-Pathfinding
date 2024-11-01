use std::collections::HashMap;
use std::time::Instant;
use rand::prelude::*;

use crate::shared::*;

pub fn random_swapping(time_limit: f64, repeatn : usize, places : Vec<Place>, dist_hm : HashMap<(usize, usize), f64>) ->f64
{
    let start = Instant::now();

    let mut path = (0..places.len()).collect::<Vec<usize>>();
    let path_len = path.len();
    path.shuffle(&mut rand::thread_rng());
    let mut eu_path = path.clone();
    eu_path.push(path[0]);//path is now a loop so cost can be calculated
    let mut current_cost = cost_calc(eu_path.clone(), dist_hm.clone());

    for _ in 0..repeatn
    {
        if start.elapsed().as_secs_f64() > time_limit
        {
            return current_cost;
        }
        
        let old = path.clone();
        path.swap(thread_rng().gen_range(0..path_len), thread_rng().gen_range(0..path_len)); // swap 2 random cities
        eu_path = path.clone();
        eu_path.push(path[0]);
        let new_cost = cost_calc(eu_path.clone(), dist_hm.clone());

        //if switch is improvement accept new tour, else discard
        if new_cost < current_cost
        {
            current_cost = new_cost;
        }
        else
        {
            path = old;
        }
        
    }
    println!("random swapping: {}", current_cost);
    return current_cost;
}
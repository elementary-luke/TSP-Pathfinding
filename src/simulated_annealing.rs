use std::collections::HashMap;
use std::time::Instant;
use rand::prelude::*;

use crate::shared::*;
pub fn annealing(time_limit: f64, repeatn : usize, places : Vec<Place>, dist_hm : HashMap<(usize, usize), f64>) -> f64
{
    let start = Instant::now();

    let mut path = (0..places.len()).collect::<Vec<usize>>();
    let path_len = path.len();
    path.shuffle(&mut rand::thread_rng());

    let mut eu_path = path.clone();
    eu_path.push(path[0]);

    let mut current_cost = cost_calc(eu_path.clone(), dist_hm.clone());

    
    for i in 0..repeatn
    {
        if start.elapsed().as_secs_f64() > time_limit
        {
            return current_cost;
        }
        let temp : f64 = 0.999994_f64.powf(i as f64);
        
        let original_path = path.clone();
        path.swap(thread_rng().gen_range(0..path_len), thread_rng().gen_range(0..path_len));
        eu_path = path.clone();
        eu_path.push(path[0]);
        let new_cost = cost_calc(eu_path.clone(), dist_hm.clone());
        if new_cost > current_cost //if switch is worse accept it with probability modelled by exponential
        {
            let prob = core::f64::consts::E.powf( (current_cost - new_cost) / temp ); 
            if rand::thread_rng().gen_range(0.0..1.0) > prob
            {
                path = original_path;
            }
            else
            {
                current_cost = new_cost;
            }
        }
        else // if improvement accept
        {
            current_cost = new_cost;
        }
        
    }
    println!("random swapping: {}", current_cost);
    return current_cost;
}
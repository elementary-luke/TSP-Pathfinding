use std::collections::HashMap;
use std::cmp::{min, max};

use crate::shared::*;

pub fn greedy(mut places : Vec<Place>, mut dist_vec : Vec<((usize, usize), f64)>, dist_hm : HashMap<(usize, usize), f64>) -> f64
{
    dist_vec.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let mut total_dist : f64 = 0.0;


    //repeatedly add smallest edge possible
    for ((from, to), dist) in dist_vec.clone()
    {
        if places[from].links.len() < 2 && places[to].links.len() < 2
        {
            let old_places = places.clone();
            places[from].links.push(to);
            places[to].links.push(from);
            if places[from].links.len() == 0 || !closed_loop(&places, from)
            {
                //println!("{} -- {}", from, to);
                total_dist += dist;
            }
            else 
            {
                places = old_places;
            }
            
        }
    }

    //add path back
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
        //println!("{}: {:?}", i, places[i].links);
    }
    println!("greedy: {}", total_dist);
    return total_dist;
}
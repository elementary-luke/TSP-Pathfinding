use std::collections::HashMap;
use std::time::Instant;
use std::cmp::{min, max};
use rand::prelude::*;

use crate::shared::*;

pub fn genetic(time_limit: f64, repeatn : usize, population_size : usize, places : Vec<Place>, dist_hm : HashMap<(usize, usize), f64>) -> f64
{
    let start = Instant::now();
    let mut paths : Vec<(Vec<usize>, f64)> = vec![];
    //create base population of random paths
    for _ in 0..population_size
    {
        let mut path = (0..places.len()).collect::<Vec<usize>>();
        path.shuffle(&mut rand::thread_rng());
        let mut dist : f64 = 0.0;

        for i in 0..path.len()
        {
            if i == path.len() - 1
            {
                break;
            }
            dist += dist_hm.get(&(min(path[i], path[i + 1]), max(path[i], path[i + 1]))).unwrap();
        }
        dist += dist_hm.get(&(min(path[0], path[path.len() - 1]), max(path[0], path[path.len() - 1]))).unwrap();

        paths.push((path, dist));
    }
    paths.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    paths = paths[0..population_size / 2].to_vec();

    for _ in 0..repeatn
    {
        if start.elapsed().as_secs_f64() > time_limit
        {
            break;
        }
        let mut new_paths = paths.clone();
        for i in 0..new_paths.len()
        {
            let ind1 = rand::thread_rng().gen_range(0..new_paths[i].0.len());
            let ind2 = rand::thread_rng().gen_range(0..new_paths[i].0.len());
            new_paths[i].0.swap(ind1, ind2);
        }

        for i in 0..new_paths.len()
        {
            let mut dist : f64 = 0.0;
            for j in 0..new_paths[i].0.len()
            {
                if j == new_paths[i].0.len() - 1
                {
                    break;
                }
                dist += dist_hm.get(&(min(new_paths[i].0[j], new_paths[i].0[j + 1]), max(new_paths[i].0[j], new_paths[i].0[j + 1]))).unwrap();
            }
            dist += dist_hm.get(&(min(new_paths[i].0[0], new_paths[i].0[new_paths[i].0.len() - 1]), max(new_paths[i].0[0], new_paths[i].0[new_paths[i].0.len() - 1]))).unwrap();
            new_paths[i].1 = dist;
        }
        paths.append(&mut new_paths);
        paths.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap()); // sort by dist
        paths = paths[0..population_size / 2].to_vec(); // get rid off bottom half
    }
    println!("random natural selection: {}", paths[0].1);
    return paths[0].1;
}
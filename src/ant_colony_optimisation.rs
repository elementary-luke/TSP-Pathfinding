use rand::distributions::WeightedIndex;
use rand::prelude::*;
use std::time::Instant;
use std::collections::HashMap;
use std::cmp::{min, max};

use crate::shared::*;

pub fn ant_colony(time_limit: f64, repeatn : usize, population_size : usize, dweight_multiplier: f64, rweight_multiplier: f64, places : Vec<Place>, dist_vec : Vec<((usize, usize), f64)>, dist_hm : HashMap<(usize, usize), f64>) -> f64
{
    let startt = Instant::now();
    let antn = population_size;
    let mut reward_matrix = vec![vec![1.0; places.len()]; places.len()];
    let mut best_cost = f64::INFINITY;

    for _ in 0..repeatn
    {
        if startt.elapsed().as_secs_f64() > time_limit
        {
            break;
        }
        for i in 0..antn
        {
            let mut edges : Vec<(usize, usize)> = vec![];
            let start : usize = rand::thread_rng().gen_range(0..places.len());
            let mut current = start;
            let mut visited : Vec<usize> = vec![];
            let mut cost : f64= 0.0;
            loop
            {
                if visited.len() == places.len() - 1
                {
                    break;
                }
                //get possible edges
                let mut dists_to_neighbours = dist_vec.clone();
                dists_to_neighbours.retain(|&x| x.0.0 == current || x.0.1 == current);
                dists_to_neighbours.retain(|&x| !visited.contains(&x.0.0) && !visited.contains(&x.0.1));
                dists_to_neighbours.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

                //calculate distribution
                let sum_of_inverses = dists_to_neighbours.iter().map(|x| (1.0 / x.1) * dweight_multiplier).sum::<f64>();
                for i in 0..dists_to_neighbours.len()
                {
                    let reward = reward_matrix[dists_to_neighbours[i].0.0][dists_to_neighbours[i].0.1];
                    let sum_adj_rewards = reward_matrix[dists_to_neighbours[i].0.0].iter().sum::<f64>() + reward_matrix[dists_to_neighbours[i].0.1].iter().sum::<f64>();
                    dists_to_neighbours[i].1 = (1.0 / dists_to_neighbours[i].1 * dweight_multiplier) * reward * rweight_multiplier / (sum_of_inverses * sum_adj_rewards * rweight_multiplier);
                }

                //get next position based on distribution
                let choices = dists_to_neighbours.iter().map(|x| x.0).collect::<Vec<(usize, usize)>>();
                let weights = dists_to_neighbours.iter().map(|x| x.1).collect::<Vec<f64>>();
                let dist = WeightedIndex::new(&weights).unwrap();
                let mut rng = rand::thread_rng();
                let next = choices[dist.sample(&mut rng)];

                edges.push(next);
                cost += dist_hm.get(&next).unwrap();
                visited.push(current);
                if next.0 == current
                {
                    current = next.1;
                }
                else
                {
                    current = next.0;
                }
            }

            //add final link back to start
            edges.push((current, start));
            cost += dist_hm.get(&(min(current, start), max(current, start))).unwrap();

            //update best
            if cost < best_cost
            {
                best_cost = cost;
            }

            //update reward matrix
            for j in edges
            {
                reward_matrix[j.0][j.1] += 1.0 / cost;
                reward_matrix[j.1][j.0] += 1.0 / cost;
            }
        }
    }
    println!("ant colony: {}", best_cost);
    return best_cost;
}
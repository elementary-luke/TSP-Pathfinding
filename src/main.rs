// ant colony
use std::f32::consts::E;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, self};
use itertools::{Itertools as it, enumerate};
use rand::distributions::WeightedIndex;
use rand::distributions::weighted::alias_method::Weight;
use rayon::vec;
use std::collections::HashMap;
use std::cmp::{min, max};
use std::time::{Duration, Instant};
use rand::prelude::*;

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
    for comb in items.iter().combinations(2).unique() // time to get from 2nd place to 3rd is the same as 3rd to 2nd
    {
        let dist = ((places[*comb[0]].x - places[*comb[1]].x).powf(2.0) + (places[*comb[0]].y - places[*comb[1]].y).powf(2.0)).sqrt();
        dist_vec.push( ((*comb[0], *comb[1]), dist) );
    }
    let mut dist_hm : HashMap<(usize, usize), f64> = HashMap::new();
    for comb in items.iter().combinations(2).unique() // time to get from 2nd place to 3rd is the same as 3rd to 2nd
    {
        let dist = ((places[*comb[0]].x - places[*comb[1]].x).powf(2.0) + (places[*comb[0]].y - places[*comb[1]].y).powf(2.0)).sqrt();
        dist_hm.insert((min(*comb[0], *comb[1]), max(*comb[0], *comb[1])), dist);
    }
    
    let mut start = Instant::now();

    let antn = 5;
    let mut reward_matrix = vec![vec![1.0; places.len()]; places.len()];
    let mut best_edges : Vec<(usize, usize)> = vec![];
    let mut best_cost = f64::INFINITY;

    for _ in 0..8000
    {
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
                let sum_of_inverses = dists_to_neighbours.iter().map(|x| 1.0 / x.1).sum::<f64>();
                for i in 0..dists_to_neighbours.len()
                {
                    let reward = reward_matrix[dists_to_neighbours[i].0.0][dists_to_neighbours[i].0.1];
                    dists_to_neighbours[i].1 = (1.0 / dists_to_neighbours[i].1) * reward / (sum_of_inverses * reward);
                }

                //get next position based on distribution
                let choices = dists_to_neighbours.iter().map(|x| x.0).collect::<Vec<(usize, usize)>>();
                let weights = dists_to_neighbours.iter().map(|x| x.1).collect::<Vec<f64>>();
                let dist = WeightedIndex::new(&weights).unwrap();
                let mut rng = rand::thread_rng();
                let next = choices[dist.sample(&mut rng)];
                // if next.0 != current
                // {
                //     let temp = next.0;
                //     next.0 = next.1;
                //     next.1 = temp;
                // }

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
                best_edges = edges.clone();
            }

            //update reward matrix
            for j in edges
            {
                reward_matrix[j.0][j.1] += 1.0 / cost;
                reward_matrix[j.1][j.0] += 1.0 / cost;
            }
        }
    }
    println!("best edges: {:?}", best_edges);
    println!("best cost: {}", best_cost);
}

fn find_mst(mut places : Vec<Place>, dist_vec : Vec<((usize, usize), f64)>) -> (Vec<Place>, f64)
{
    let mut visited : Vec<usize> = vec![0];
    let mut mst_cost = 0.0;

    // println!("{places:?}");
    loop
    {
        if visited.len() == places.len()
        {
            break;
        }
        let mut mst_edges = dist_vec.clone();
        mst_edges.retain(|&x| visited.contains(&x.0.0) != visited.contains(&x.0.1));
        mst_edges.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        
        let old_places = places.clone();
        places[mst_edges[0].0.0].links.push(mst_edges[0].0.1);
        places[mst_edges[0].0.1].links.push(mst_edges[0].0.0);
        if closed_loop(&places, mst_edges[0].0.0)
        {
            places = old_places;
            continue;
        }
        mst_cost += mst_edges[0].1;
        
        if visited.contains(&mst_edges[0].0.0)
        {
            visited.push(mst_edges[0].0.1);
        }
        else
        {
            visited.push(mst_edges[0].0.0);
        }
    }
    // for i in 0..places.len()
    // {
    //     println!("{}: {:?}", i, places[i].links);
    // }
    // println!("{visited:?}    {mst_cost}");
    return (places, mst_cost);
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

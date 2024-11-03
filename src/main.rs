mod one_tree;
mod ant_colony_optimisation;
mod nearest_neighbour;
mod greedy;
mod genetic;
mod simulated_annealing;
mod random_swapping;
mod shared;

use crate::ant_colony_optimisation::*;
use crate::nearest_neighbour::*;
use crate::greedy::*;
use crate::genetic::*;
use crate::one_tree::*;
use crate::simulated_annealing::*;
use crate::random_swapping::*;
use crate::shared::*;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use itertools::Itertools as it;
use std::collections::HashMap;
use std::cmp::{min, max};
use rand::prelude::*;
use rand::thread_rng;
use std::fs;
fn main() 
{
    loop
    {
        //create a vector of random co-ordinates that represent nodes on a graph
        let mut places : Vec<Place> = vec![];

        for _ in 0..thread_rng().gen_range(4..50)
        {
            places.push(Place::new(thread_rng().gen_range(0.0..100.0), thread_rng().gen_range(0.0..100.0)));
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

        //iterative algorithms stop when timeout or iterations complete, whichever comes first.
        let max_mins : f64 = 5.0;
        let max_iterations : usize = 10000000000000;
        
        println!("no. of places: {:?}", places.len());
        let lower_bound = get_highest_cost_one_tree(places.clone(), dist_vec.clone()).1;
        println!("lower_bound: {}", lower_bound);
        let greedy_dist = greedy(places.clone(), dist_vec.clone(), dist_hm.clone());
        let nearest_neighbour_dist = nearest_neighbour(places.clone());
        let natural_selection_dist = genetic(max_mins * 60.0, max_iterations, 100000, places.clone(), dist_hm.clone());
        let ant_colony_dist = ant_colony(max_mins * 60.0, max_iterations, 5000, 1.0, 0.2,places.clone(), dist_vec.clone(), dist_hm.clone());
        let random_swapping = random_swapping(max_mins* 60.0, max_iterations, places.clone(), dist_hm.clone());
        let annealing_dist = annealing(max_mins * 60.0, max_iterations, places.clone(), dist_hm.clone());
        let name = format!("./results/{:?}.txt", rand::random::<u64>());
        output(name.to_string(), format!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}", places.len(), lower_bound, greedy_dist, nearest_neighbour_dist, natural_selection_dist, ant_colony_dist, random_swapping, annealing_dist));
    }
}

fn get_places_from_file() -> Vec<Place>
{
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
    return places;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn output(name : String, string : String)
{
    fs::write(name, string).expect("Unable to write file");
}
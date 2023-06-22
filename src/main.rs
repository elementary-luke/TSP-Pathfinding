// ant colony
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, self};
use itertools::{Itertools as it, enumerate};
use rand::distributions::WeightedIndex;
use std::collections::HashMap;
use std::cmp::{min, max};
use std::time::{Duration, Instant};
use rand::prelude::*;
use rand::thread_rng;
use std::fs;

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
    loop
    {
        //let end_pos : Place = Place::new(40.0, 40.0);
        // create positions from text file
        let mut places : Vec<Place> = vec![];
        // if let Ok(lines) = read_lines("places.txt") 
        // {
        //     for line in lines 
        //     {
        //         if let Ok(l) = line 
        //         {
        //             let temp = l.split(",").map(|x| x.parse::<f64>().unwrap()).collect::<Vec<f64>>();
        //             places.push(Place::new(temp[0], temp[1]));
        //         }
        //     }
        // }

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
        
        println!("please wait by default 20 mins for results to be generated in the results folder:");
        println!("no. of places: {:?}", places.len());
        let lower_bound = get_highest_cost_one_tree(places.clone(), dist_vec.clone()).1;
        println!("lower_bound: {}", lower_bound);
        let greedy_dist = greedy(places.clone(), dist_vec.clone(), dist_hm.clone());
        let nearest_neighbour_dist = nearest_neighbour(places.clone(), dist_vec.clone(), dist_hm.clone());
        let natural_selection_dist = natural_selection(5.0 * 60.0, 10000000000000, 100000, places.clone(), dist_vec.clone(), dist_hm.clone());
        let ant_colony_dist = ant_colony(5.0 * 60.0, 10000000000000, 5000, 1.0, 0.2,places.clone(), dist_vec.clone(), dist_hm.clone());
        let random_swapping = random_swapping(5.0*60.0, 100000000000000, places.clone(), dist_vec.clone(), dist_hm.clone());
        let annealing_dist = annealing(5.0*60.0, 100000000000000, places.clone(), dist_vec.clone(), dist_hm.clone());
        let name = format!("./results/{:?}.txt", rand::random::<u64>());
        output(name.to_string(), format!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}", places.len(), lower_bound, greedy_dist, nearest_neighbour_dist, natural_selection_dist, ant_colony_dist, random_swapping, annealing_dist));
    }
}

fn random_swapping(time_limit: f64, repeatn : usize, mut places : Vec<Place>, mut dist_vec : Vec<((usize, usize), f64)>, dist_hm : HashMap<(usize, usize), f64>) ->f64
{
    let start = Instant::now();

    let mut path = (0..places.len()).collect::<Vec<usize>>();
    let path_len = path.len();
    path.shuffle(&mut rand::thread_rng());
    let mut eu_path = path.clone();
    eu_path.push(path[0]);//path is now a loop so cost can be calculated
    let mut current_cost = cost_calc(eu_path.clone(), dist_hm.clone());

    for i in 0..repeatn
    {
        if start.elapsed().as_secs_f64() > time_limit
        {
            return current_cost;
        }
        let mut temp : f64 = 0.999994_f64.powf(i as f64);
        
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

fn annealing(time_limit: f64, repeatn : usize, mut places : Vec<Place>, mut dist_vec : Vec<((usize, usize), f64)>, dist_hm : HashMap<(usize, usize), f64>) -> f64
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
        let mut temp : f64 = 0.999994_f64.powf(i as f64);
        
        let original_path = path.clone();
        path.swap(thread_rng().gen_range(0..path_len), thread_rng().gen_range(0..path_len));
        eu_path = path.clone();
        eu_path.push(path[0]);
        let new_cost = cost_calc(eu_path.clone(), dist_hm.clone());
        if new_cost > current_cost //if switch is worse accept with probability modelled by exponential
        {
            let prob = core::f64::consts::E.powf( (current_cost - new_cost) / temp); 
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

fn natural_selection(time_limit: f64, repeatn : usize, population_size : usize, mut places : Vec<Place>, mut dist_vec : Vec<((usize, usize), f64)>, dist_hm : HashMap<(usize, usize), f64>) -> f64
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

        let mut dist : f64 = 0.0;

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

fn greedy(mut places : Vec<Place>, mut dist_vec : Vec<((usize, usize), f64)>, dist_hm : HashMap<(usize, usize), f64>) -> f64
{
    let mut start = Instant::now();
    dist_vec.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let mut total_dist : f64 = 0.0;


    //repeatedly add closest edge
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

fn nearest_neighbour(mut places : Vec<Place>, dist_vec : Vec<((usize, usize), f64)>, dist_hm : HashMap<(usize, usize), f64>) -> f64
{
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
        for j in dist_vec.iter() // take path to closest neighbour city
        {
            if j.0.0 == current && j.0.1 != 0 // make sure we dont go back to the start point prematurely
            {
                let last = current.to_owned();
                total_dist += j.1;
                path.push(current);
                current = j.0.1;
                //println!("{} -> {} : {}m", last, current, j.1);
                if j.0.0 != 0
                {
                    dist_vec.retain(|x| x.0.1 != last);
                }
                break;
            }
        }
    }
    // add the distance from the last place to the start
    dist_vec.retain(|x| x.0.0 == current && x.0.1 == 0);
    path.push(current);

    total_dist += dist_vec[0].1; 
    //println!("{} -> 0 : {}m", current, dist_vec[0].1);
    path.push(0);
    println!("nearest neighbour: {}", total_dist);
    return total_dist;
}

fn ant_colony(time_limit: f64, repeatn : usize, population_size : usize, dweight_multiplier: f64, rweight_multiplier: f64, mut places : Vec<Place>, dist_vec : Vec<((usize, usize), f64)>, dist_hm : HashMap<(usize, usize), f64>) -> f64
{
    let startt = Instant::now();
    let antn = population_size;
    let mut reward_matrix = vec![vec![1.0; places.len()]; places.len()];
    let mut best_edges : Vec<(usize, usize)> = vec![];
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
    println!("ant colony: {}", best_cost);
    return best_cost;
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
        mst_edges.retain(|&x| visited.contains(&x.0.0) != visited.contains(&x.0.1)); // get edges that connect visited and unvisited
        mst_edges.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        
        let old_places = places.clone();
        places[mst_edges[0].0.0].links.push(mst_edges[0].0.1);
        places[mst_edges[0].0.1].links.push(mst_edges[0].0.0);

        // make sure no closed loops
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

        //if both links are in visited then there is a closed loop
        if visited.contains(&places[index].links[0]) && visited.contains(&places[index].links[1])
        {
            return true
        }

        //if one link is in visited then go to the other link
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

fn get_highest_cost_one_tree(places : Vec<Place>, dist_vec : Vec<((usize, usize), f64)>) -> (Vec<Place>, f64)
{
    let mut one_trees : Vec<(Vec<Place>, f32)> = vec![];

    //get every possible one tree, excluding every city once
    for i in 0..places.len()
    {
        one_trees.push(one_tree(places.clone(), dist_vec.clone(), i));
    }

    //sort so highest cost is first
    one_trees.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    one_trees.reverse();
    
    // for i in 0..one_trees[0].0.len()
    // {
    //     println!("{i}: {:?}", one_trees[0].0[i].links);
    // }
    
    (one_trees[0].clone().0, one_trees[0].1 as f64)
}

fn one_tree(mut places : Vec<Place>, dist_vec : Vec<((usize, usize), f64)>, exclude : usize) -> (Vec<Place>, f32)
{
    let mut visited : Vec<usize> = vec![0];
    if exclude == 0
    {
        visited = vec![1];
    }
    let mut mst_cost = 0.0;


    // get mst with exclusion
    loop
    {
        if visited.len() == places.len() - 1
        {
            break;
        }
        let mut mst_edges = dist_vec.clone();
        mst_edges.retain(|&x| (visited.contains(&x.0.0) != visited.contains(&x.0.1)) && (x.0.0 != exclude && x.0.1 != exclude));
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
    //add last city back in
    let mut mst_edges = dist_vec.clone();
    mst_edges.retain(|&x| x.0.0 == exclude || x.0.1 == exclude);
    mst_edges.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    //add last 2 links
    places[mst_edges[0].0.0].links.push(mst_edges[0].0.1);
    places[mst_edges[0].0.1].links.push(mst_edges[0].0.0);
    mst_cost += mst_edges[0].1;

    places[mst_edges[1].0.0].links.push(mst_edges[1].0.1);
    places[mst_edges[1].0.1].links.push(mst_edges[1].0.0);
    mst_cost += mst_edges[1].1;
    visited.push(exclude);

    return (places, mst_cost as f32)
}


fn cost_calc(path : Vec<usize>, dist_hm : HashMap<(usize, usize), f64>) -> f64
{
    let mut cost = 0.0;
    for i in 0..(path.len() - 1)
    {
        cost += dist_hm.get(&(min(path[i], path[i + 1]), max(path[i], path[i + 1]))).unwrap();
    }
    return  cost;
}

fn output(name : String, string : String)
{
    fs::write(name, string).expect("Unable to write file");
}
// one tree prims algorithm


use std::f32::consts::E;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use itertools::{Itertools as it, enumerate};
use std::collections::HashMap;
use std::cmp::{min, max};
use std::time::{Duration, Instant};

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
    for perm in items.iter().combinations(2).unique() // time to get from 2nd place to 3rd is the same as 3rd to 2nd
    {
        let dist = ((places[*perm[0]].x - places[*perm[1]].x).powf(2.0) + (places[*perm[0]].y - places[*perm[1]].y).powf(2.0)).sqrt();
        dist_vec.push( ((*perm[0], *perm[1]), dist) );
    }
    let mut dist_hm : HashMap<(usize, usize), f64> = HashMap::new();
    for perm in items.iter().combinations(2).unique() // time to get from 2nd place to 3rd is the same as 3rd to 2nd
    {
        let dist = ((places[*perm[0]].x - places[*perm[1]].x).powf(2.0) + (places[*perm[0]].y - places[*perm[1]].y).powf(2.0)).sqrt();
        dist_hm.insert((min(*perm[0], *perm[1]), max(*perm[0], *perm[1])), dist);
    }
    
    let mut start = Instant::now();

    let mut one_trees : Vec<(Vec<Place>, f32)> = vec![];

    for i in 0..places.len()
    {
        one_trees.push(one_tree(places.clone(), &mut dist_vec, i));
    }

    //make highest cost first
    one_trees.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    one_trees.reverse();
    
    for i in 0..one_trees[0].0.len()
    {
        println!("{i}: {:?}", one_trees[0].0[i].links);
    }
    println!("{:?}", one_trees[0].1);

}

fn one_tree(mut places : Vec<Place>, dist_vec : &mut Vec<((usize, usize), f64)>, exclude : usize) -> (Vec<Place>, f32)
{
    let mut visited : Vec<usize> = vec![0];
    if exclude == 0
    {
        visited = vec![1];
    }
    let mut mst_cost = 0.0;


    // do with exclusion
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
    //add last 1 back in
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

fn closed_loop(places : &Vec<Place>, mut index : usize) -> bool
{
    let mut visited : Vec<usize> = vec![];

    loop
    {
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
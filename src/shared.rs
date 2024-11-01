use std::collections::HashMap;
use std::cmp::{min, max};

#[derive(Clone, Debug)]
pub struct Place 
{
    pub x: f64,
    pub y: f64,
    pub links : Vec<usize>,
}

impl Place
{
    pub fn new(x: f64, y: f64) -> Place
    {
        Place {x, y, links : vec![]}
    }

}

pub fn cost_calc(path : Vec<usize>, dist_hm : HashMap<(usize, usize), f64>) -> f64
{
    let mut cost = 0.0;
    for i in 0..(path.len() - 1)
    {
        cost += dist_hm.get(&(min(path[i], path[i + 1]), max(path[i], path[i + 1]))).unwrap();
    }
    return  cost;
}

pub fn closed_loop(places : &Vec<Place>, mut index : usize) -> bool
{
    let mut visited : Vec<usize> = vec![];

    loop
    {
        //println!("{}", index);
        if places[index].links.len() == 1
        {
            return false
        }

        //if both links have been visited then there is a closed loop
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

pub fn find_mst(mut places : Vec<Place>, dist_vec : Vec<((usize, usize), f64)>) -> (Vec<Place>, f64)
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
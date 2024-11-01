use crate::shared::*;

pub fn get_highest_cost_one_tree(places : Vec<Place>, dist_vec : Vec<((usize, usize), f64)>) -> (Vec<Place>, f64)
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

pub fn one_tree(mut places : Vec<Place>, dist_vec : Vec<((usize, usize), f64)>, exclude : usize) -> (Vec<Place>, f32)
{
    let mut visited : Vec<usize> = vec![0];
    if exclude == 0
    {
        visited = vec![1];
    }
    let mut mst_cost = 0.0;


    // get min span tree with exclusion
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
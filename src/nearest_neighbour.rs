use crate::shared::*;
use itertools::Itertools as it;

pub fn nearest_neighbour(places : Vec<Place>) -> f64
{
    let mut dist_vec : Vec<((usize, usize), f64)> = vec![];
    let items : Vec<usize> = (0..places.len()).collect();
    for perm in items.iter().permutations(2).unique() // time to get from 2nd place to 3rd is the same as 3rd to 2nd
    {
        let dist = ((places[*perm[0]].x - places[*perm[1]].x).powf(2.0) + (places[*perm[0]].y - places[*perm[1]].y).powf(2.0)).sqrt();
        dist_vec.push( ((*perm[0], *perm[1]), dist) );
    }
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
                //make sure link can't be made again to the node we were at before
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
// christofides
use std::f32::consts::E;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use itertools::{Itertools as it, enumerate};
use rayon::vec;
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
    dist_vec.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    //get min span tree
    places = find_mst(places.clone(), dist_vec.clone()).0;
    let mut odd_degrees : Vec<usize> = vec![];

    for i in 0..places.len()
    {
        if places[i].links.len() % 2 != 0
        {
            odd_degrees.push(i);
        }
    }

    //find minimum weight matching
    let mut best_pairs : Vec<(usize, usize)> = vec![];
    let mut best_time : f64 = f64::INFINITY;
    let old_odd_degrees = odd_degrees.clone();
    for i in 0..old_odd_degrees.len()
    {
        odd_degrees = old_odd_degrees.clone();
        odd_degrees.remove(i);
        odd_degrees.insert(0, old_odd_degrees[i]);
        let mut time : f64= 0.0;
        let mut pairs : Vec<(usize, usize)> = vec![];
        loop 
        {
            if odd_degrees.len() == 0
            {
                break;
            }
            let current = odd_degrees[0];

            let mut dv = dist_vec.clone();
            dv.retain(|&x| (odd_degrees.contains(&x.0.0) && x.0.1 == current || x.0.0 == current && odd_degrees.contains(&x.0.1)));
            pairs.push(dv[0].0);
            time += dv[0].1;
            let index = odd_degrees.iter().position(|x| *x == dv[0].0.0).unwrap();
            odd_degrees.remove(index);
            let index = odd_degrees.iter().position(|x| *x == dv[0].0.1).unwrap();
            odd_degrees.remove(index);
        }
        if time < best_time
        {
            best_time = time;
            best_pairs = pairs.clone();
            //println!("{}: {:?}", best_time, best_pairs)
        }
    }
    println!("{:?}", best_pairs);

    let mut need_to_visit : Vec<usize> = vec![];
    for i in 0..places.len()
    {
        for j in 0..places[i].links.len()
        {
            if !need_to_visit.contains(&places[i].links[j])
            {
                need_to_visit.push(places[i].links[j]);
            }
        }
    }
    for i in best_pairs.clone()
    {
        need_to_visit.push(i.0);
        need_to_visit.push(i.1);
    }

    //combine into multigraph
    for i in best_pairs
    {
        places[i.0].links.push(i.1);
        places[i.1].links.push(i.0);
    }
    for i in 0..places.len()
    {
        println!("{i}: {:?}", places[i].links);
    }

    //do eularian tour
    let mut path : Vec<usize> = vec![];
    let mut current = 0;
    
    'outer : loop
    {
        if places[current].links.len() == 0
        {
            break;
        }
        path.push(current);
        let mut next : usize = places[current].links[0];
        for i in places[current].links.iter()
        {
            // if current == 6
            // {
            //     println!("{:?}",  places[current].links[0]);
            //     println!("{i}: {:?}", places[*i].links);
            // }
            if places[*i].links.len() == 2 && places[*i].links[0] == current && places[*i].links[1] == current
            {
                println!("ASDASD {}", *i);
                path.push(*i);
                let index = places[current].links.iter().position(|x| *x == *i).unwrap();
                places[current].links.remove(index);
                continue 'outer
            }
        }
        places[current].links.remove(0);
        let index = places[next].links.iter().position(|x| *x == current).unwrap();
        places[next].links.remove(index);
        current = next;
    }
    println!("{:?}", path);

    let mut new_path : Vec<usize> = vec![];
    let mut visited : Vec<usize> = vec![];

    for i in path
    {
        if !visited.contains(&i)
        {
            visited.push(i);
            new_path.push(i);
        }
    }
    new_path.push(0);
    println!("{:?}", new_path);
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

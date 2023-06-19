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

    //find mimimum weight matching
    //odd_degrees.reverse();
    println!("odd_degrees: {:?}", odd_degrees.len());
    let mut matrix :Vec<Vec<f64>> = vec![];
    for i in 0..odd_degrees.len()
    {
        let mut row : Vec<f64> = vec![];
        for j in 0..odd_degrees.len()
        {
            if i == j
            {
                row.push(-4.0);
                continue;
            }
            row.push(dist_hm.get(&(min(odd_degrees[i], odd_degrees[j]), max(odd_degrees[i], odd_degrees[j]))).unwrap().clone());
        }
        matrix.push(row);
    }

    for i in 0..matrix.len()
    {
        println!("{:.2?}", matrix[i]);
    }
    println!("\n");

    //row reduction
    for i in 0..matrix.len()
    {
        let min = matrix[i].iter().filter(|x| **x >= 0.0).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap().clone();
       matrix[i] = matrix[i].iter().map(|x| x - min).collect::<Vec<f64>>();
    }
    for i in 0..matrix.len()
    {
        println!("{:.2?}", matrix[i]);
    }
    println!("\n");

    //column reduction
    for i in 0..matrix.len()
    {
        let mut col : Vec<f64> = vec![];
        for j in 0..matrix.len()
        {
            col.push(matrix[j][i]);
        }
        let min = col.iter().filter(|x| **x >= 0.0).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap().clone();
        for j in 0..matrix.len()
        {
            matrix[j][i] -= min;
        }
    }
    for i in 0..matrix.len()
    {
        println!("{:.2?}", matrix[i]);
    }
    println!("\n");

    let mut pairs : Vec<(usize, usize)> = vec![];
    //pick 0s
    for _ in 0..3
    {
        if pairs.len() == 100
        {
            break;
        }
        for i in 0..matrix.len()
        {
            if matrix[i].iter().filter(|&x| *x == 0.0 || *x == -0.0).count() == 1
            {
                let j = matrix[i].iter().position(|x| *x == 0.0 || *x == -0.0).unwrap();
                matrix[i][j] = -6.0;
                
                pairs.push((odd_degrees[i], odd_degrees[j]));
                // pairs.push((i, j));

                //strike out 0s in same column
                for k in 0..matrix.len()
                {
                    matrix[k][j] = -6.0
                }
                matrix[j][i] = -6.0;
                break;
            }
            
        }
        for k in 0..matrix.len()
        {
            println!("{:.2?}", matrix[k]);
        }
        println!("a\n");
    }
    println!("{:?}", pairs);

    //vid original
    let mut c = 0.0;
    c += dist_hm.get(&(7, 10)).unwrap().clone();
    c += dist_hm.get(&(3, 6)).unwrap().clone();
    c += dist_hm.get(&(9, 11)).unwrap().clone();
    c += dist_hm.get(&(1, 12)).unwrap().clone();

    let mut d = 0.0;
    d += dist_hm.get(&(1, 12)).unwrap().clone();
    d += dist_hm.get(&(3, 6)).unwrap().clone();
    d += dist_hm.get(&(7, 11)).unwrap().clone();
    d += dist_hm.get(&(9, 10)).unwrap().clone();

    println!("{odd_degrees:?} {c} {d}");

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

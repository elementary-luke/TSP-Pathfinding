use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use itertools::Itertools as it;
use std::collections::HashMap;
use std::cmp::{min, max};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Place {
    x: f64,
    y: f64,
    g_cost : f64,
    h_cost : f64,
    f_cost : f64,
    connection : Option<usize>,
}

impl Place
{
    fn new(x: f64, y: f64) -> Place
    {
        Place {x, y, g_cost: 10000.0, h_cost: 100000.0, f_cost: 10000.0, connection : None}
    }

}

fn main() {
    let start_pos : Place = Place::new(0.0, 0.0);
    let end_pos : Place = Place::new(40.0, 40.0);
    // create positions from text file
    let mut places : Vec<Place> = vec![];
    places.push(start_pos);
    if let Ok(lines) = read_lines("pslaces.txt") {
        for line in lines {
            if let Ok(l) = line {
                let temp = l.split(",").map(|x| x.parse::<f64>().unwrap()).collect::<Vec<f64>>();
                places.push(Place::new(temp[0], temp[1]));
            }
        }
    }
    places.push(end_pos);

    //create dictionary with distance from any place to another
    let mut dist_hm : HashMap<(usize, usize), f64> = HashMap::new();
    let items : Vec<usize> = (0..places.len()).collect();
    for perm in items.iter().combinations(2).unique() // time to get from 2nd place to 3rd is the same as 3rd to 2nd
    {
        let time = ((places[*perm[0]].x - places[*perm[1]].x).powf(2.0) + (places[*perm[0]].y - places[*perm[1]].y).powf(2.0)).sqrt();
        dist_hm.insert((min(*perm[0], *perm[1]), max(*perm[0], *perm[1])), time);
    }
    drop(items);

    let mut to_search : Vec<usize> = vec![0];
    let mut processed : Vec<usize> = vec![];


    loop
    {
        let mut current = 0;
        for i in to_search.to_owned()
        {
            let current_p = &places[current];
            let to = &places[i];
            if to.f_cost < current_p.f_cost || to.f_cost == current_p.f_cost && to.h_cost < current_p.h_cost
            {
                current = i;
            }
        }

        processed.push(current.to_owned());
        to_search.retain(|&x| x != current);

        let mut neighbours : Vec<usize> = (0..places.len()).collect();
        neighbours.retain(|&x| !processed.contains(&x));
        for n in neighbours
        {
            let in_search = to_search.contains(&n);
            let cost_to_neighbour = dist_hm.get(&(min(current, n), max(current, n))).unwrap();

            if !in_search || cost_to_neighbour < &places[n].g_cost
            {
                places[n].g_cost = cost_to_neighbour.to_owned();
                places[n].f_cost = places[n].g_cost + places[n].h_cost;
                places[n].connection = Some(current);
                if !in_search
                {
                    places[n].h_cost = ((places[n].x - places[places.len()].x).powf(2.0) + (places[n].y - places[places.len()].y).powf(2.0)).sqrt();
                    places[n].f_cost = places[n].g_cost + places[n].h_cost;
                    to_search.push(n);
                }
            }
        }


        if to_search.len() == 0
        {
            break;
        }
    }  


}


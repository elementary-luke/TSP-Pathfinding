use itertools::Itertools as it;
mod clintersect;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::sync::Mutex;
use rand::thread_rng;
use rand::seq::SliceRandom;
use rayon::prelude::*;
use std::time::{Duration, Instant};



fn main()
{   
    let start = Instant::now();
    let mut x : f64 = 0.0;
    let mut y : f64 = 0.0;
    let upspeed : f64 = 1.0;
    let downspeed : f64 = 4.0;
    let flatspeed : f64 = 2.0;
    let mons_a_angle : f64 = (0.75 as f64).atan();
    let mut mountains : Vec<Vec<f64>> = vec![];
    
    if let Ok(lines) = read_lines("mountains.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(l) = line {
                let mut temp = l.split(",").map(|x| x.parse::<f64>().unwrap()).collect::<Vec<f64>>();
                temp[2] *= 0.1875;
                temp.push(temp[2] / mons_a_angle.tan());
                mountains.push(temp.clone());
            }
        }
    }
    
    let mut shores : Vec<Vec<f64>> = vec![];
    if let Ok(lines) = read_lines("closest_shores.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(l) = line {
                shores.push(l.split(",").map(|x| x.parse::<f64>().unwrap()).collect::<Vec<f64>>());
            }
        }
    }
    //println!("{:?}", mountains.len());

    //let items : <Vec<usize>> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60]
    //    .iter().map(|x| x-1).collect::<Vec<usize>>();
    
    let mut time_hm : HashMap<(usize, usize), f64> = HashMap::new();

    for i in 0..mountains.len()
    {
        let mons = mountains[i].clone();
        let mut time_rn : f64 = 0.0;

        x = shores[i][0];
        y = shores[i][1];

        if (x, y) != (mons[0], mons[1])
        {
            let fd = ((x-mons[0]).powf(2.0) + (y-mons[1]).powf(2.0)).sqrt();
            time_rn += (fd - mons[3]) / flatspeed + mons[3] / upspeed;
        }

        time_hm.insert((69420, i), time_rn.clone());

        time_rn = 0.0;

        let tx = x;
        let ty = y;

        if (mons[0], mons[1]) != (tx, ty)
        {
            let fd = ((mons[0]-tx).powf(2.0) + (mons[1]-ty).powf(2.0)).sqrt();
            time_rn += mons[3] / downspeed + (fd - mons[3]) / flatspeed;
        }
        time_hm.insert((i, 69420), time_rn.clone());
    }


    let items : Vec<usize> = (0..mountains.len()).collect();
    for perm in items.iter().permutations(2).unique()
    {   
        let mut time_rn : f64 = 0.0;
        let on_rn = &mountains[perm[0].clone()];
        let going_to = &mountains[perm[1].clone()];
        
        
        //check if she dies as shes now in a mountain (needs to enclose?) OR if she cant get to a mountain in a mountain
        if circle_enclosed(on_rn[0], on_rn[1], on_rn[3], going_to[0], going_to[1], going_to[3]) || circle_enclosed(going_to[0], going_to[1], going_to[3], on_rn[0], on_rn[1], on_rn[3])
        {
            //println!("enclosed");
            continue;
        }

        if ((going_to[0] - on_rn[0]).powf(2.0) + (going_to[1] - on_rn[1]).powf(2.0)).sqrt() - on_rn[3] - going_to[3] > 0.0
        {
            time_rn += on_rn[3] / downspeed + going_to[3] / upspeed + (((going_to[0] - on_rn[0]).powf(2.0)+ (going_to[1] - on_rn[1]).powf(2.0)).sqrt() - on_rn[3] - going_to[3]) / 2.0;
            time_hm.insert((perm[0].clone(), perm[1].clone()), time_rn.clone());
            
        }
        else
        {
            let fd = ((going_to[0] - on_rn[0]).powf(2.0) + (going_to[1] - on_rn[1]).powf(2.0)).sqrt(); //flat distant between centre of the 2 mountains
            let down_dist = (fd + on_rn[3] - going_to[3]) / 2.0;
            time_rn += down_dist / downspeed;

            let up_dist = fd - down_dist;
            time_rn += up_dist / upspeed;
            time_hm.insert((perm[0].clone(), perm[1].clone()), time_rn.clone());
        }
    }

    //[([0, 1, 3], 2.0), ([3, 0, 1], 5.0)]
    let mut gene_pool : Vec<(Vec<usize>, f64)> = vec![];
    let sgene_num : usize = 20000;//20000

    //println!("{:?}", time_hm);
    for _i in 0..sgene_num
    {   
        let mut mountain_seq : Vec<usize>= (0..mountains.len()).collect();
        mountain_seq.shuffle(&mut thread_rng());
        let mut time_rn : f64 = 0.0;
        match time_hm.get(&(69420 as usize, mountain_seq[0]))
        {
            Some(time) => time_rn += time,
            None => println!("shore to {} doesnt exist", mountain_seq[0])
        }
        for i in 0..mountain_seq.len()-1
        {
            match time_hm.get(&(mountain_seq[i] , mountain_seq[i+1]))
            {
                Some(time) => time_rn += time,
                None => 
                {
                    //println!("{} to {} is enclosed", mountain_seq[0], mountain_seq[1]);
                    time_rn = -1.0;
                    break;
                }
            }
        }
        if time_rn == -1.0
        {
            continue;
        }
        match time_hm.get(&(mountain_seq[mountain_seq.len() - 1], 69420 as usize))
        {
            Some(time) => time_rn += time,
            None => println!("shore to {} doesnt exist", mountain_seq[0])
        }

        gene_pool.push((mountain_seq.clone(), time_rn.clone()));
    }

    let mut locked : HashMap<usize, usize> = HashMap::new();
    let mut gen : i64 = 1;
    loop
    {
        if locked.keys().len() >= 60
        {
            break;
        }
        println!("{} {}", locked.keys().len(), gene_pool.len());
        gene_pool.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        // for i in (0..5)
        // {
        //     println!("{:?}", gene_pool[i]);
        // }
        
        //(seq_number, array_pos), number of occurences
        let mut common_hm : HashMap<(usize, usize), usize> = HashMap::new();

        for i in 0..(sgene_num/10) // in the top 100 results, whats most common
        {
            for j in 0..(gene_pool[i].0.len())
            {
                if !locked.contains_key(&j)
                {
                    common_hm.entry((gene_pool[i].0[j], j)).and_modify(|i| *i += 1).or_insert(1);
                }
            }
        }

        let mut common_vec : Vec<((usize, usize), usize)> = common_hm.into_iter().collect();
        common_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        let keep_top_n : usize = 1;
        for i in 0..keep_top_n
        {
            locked.insert(common_vec[i].0.1, common_vec[i].0.0);
        }
        //[(seq_number, array_pos), (seq_number, array_pos)]
        
        gene_pool.clear();
        let seqs : Vec<Vec<usize>> = (0..sgene_num).into_par_iter().map(|_j| {
            //println!("{:?}", locked);
            let mut numbers_remaining : Vec<usize> = (0..mountains.len()).collect();
            let mut seq : Vec<usize> = vec![0 as usize; mountains.len()];
            for i in locked.iter()
            {
                seq[*i.0] = *i.1;
                numbers_remaining.retain(|x| *x != *i.1);
                numbers_remaining.shuffle(&mut thread_rng());
            }
            for i in 0..seq.len()
            {
                if !locked.contains_key(&i) // not locked
                {
                    match numbers_remaining.pop()
                    {
                        Some(x) => 
                        {
                            if locked.values().contains(&x)
                            {
                                println!("{}", &x);
                            }
                            seq[i] = x;
                            //println!("{:?}", numbers_remaining)
                        }
                        None => {}//println!("YA KELB {:?}", seq[i])
                    }
                }
                else
                {
                    match locked.get(&i)
                    {
                        Some(x) => seq[i] = *x,
                        None => println!("DASDAS")
                    }
                }
            }
            seq
        }).collect();
        // for i in &seqs
        // {
        //     println!{"{:?}", i};
        // }
            
        gene_pool = (0..seqs.len()).into_par_iter()
        .map(|j| {
            let mut time_rn : f64 = 0.0;
            match time_hm.get(&(69420 as usize, seqs[j][0]))
            {
                Some(time) => time_rn += time,
                None => println!("shore to {} doesnt exist", seqs[j][0])
            }
            for i in 0..seqs[j].len()-1
            {
                match time_hm.get(&(seqs[j][i] , seqs[j][i+1]))
                {
                    Some(time) => time_rn += time,
                    None => 
                    {
                        //println!("{} to {} is enclosed", mountain_seqs[j][0], mountain_seqs[j][1]);
                        time_rn = -1.0;
                        break;
                    }
                }
            }
            if (time_rn != -1.0)
            {
                match time_hm.get(&(seqs[j][seqs[j].len() - 1], 69420 as usize))
                {
                    Some(time) => time_rn += time,
                    None => println!("shore to {} doesnt exist", seqs[j][0])
                }
            }

            (seqs[j].clone(), time_rn.clone())
        })
        .filter(|i| i.1 != -1.0)
        .collect::<Vec<(Vec<usize>, f64)>>();
        

        let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("answer.txt")
        .unwrap();
        use std::io::prelude::*;
        if let Err(e) = writeln!(file, "GEN:{} BEST:{:?} LOCKED:{}", gen, gene_pool[0], locked.keys().len()) {
            eprintln!("Couldn't write to file: {}", e);
        }
        println!("{:?}", gene_pool[0]);
        gen += 1;
    }

    gene_pool.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    for i in 0..3
    {
        println!("{:?}", gene_pool[i])
    }

    let mut file = std::fs::OpenOptions::new()
    .write(true)
    .append(true)
    .open("answer.txt")
    .unwrap();
    use std::io::prelude::*;
    if let Err(e) = writeln!(file, "GEN:{} BESTS:1st{:?} || 2nd{:?} || 3rd{:?} LOCKED:{}", gen, gene_pool[0],gene_pool[1], gene_pool[2], locked.keys().len()) {
        eprintln!("Couldn't write to file: {}", e);
    }
    let mut input_string = String::from("aa");
    println!("took {} seconds to complete", start.elapsed().as_secs());
    io::stdin().read_line(&mut input_string).unwrap();
}

fn circle_enclosed(x1:f64, y1:f64, r1:f64, x2:f64, y2:f64, r2:f64) -> bool
{
    let d = ((x2-x1).powf(2.0) + (y2-y1).powf(2.0)).sqrt();
    if r1 > (d+r2)
        {return true;}
    else
        {return false;}
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
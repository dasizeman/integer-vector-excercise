use rand::prelude::*;
use std::collections::HashMap;

//fn random_int_hard(start: i32, end: i32) -> i32 {
//    let min = start as f64;
//    let max = end as f64;
//    let result = (max * random::<f64>() + min).floor() as i32;
//    result
//}

fn random_int(start: i32, end: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(start..end)
}

fn random_vec_build(start: i32, end: i32, len: i32) -> Vec<i32> {
    let mut vec = Vec::new();
    for _ in 0..len {
        vec.push(random_int(start, end));
    }
    vec
}

fn i32_vec_mean(vec: &Vec<i32>) -> f32 {
    let mut total = 0;
    for value in vec {
        total += value;
    }
    
    let num_items = vec.len() as f32;
    let total = total as f32;
    total / num_items
}

fn i32_vec_median(vec: &Vec<i32>) -> i32 {
    let mut sorted = vec.clone();
    sorted.sort();
    let midpoint = sorted.len() as f32;
    let midpoint = (midpoint / 2.0).floor() as usize;
    if let Some(value) = sorted.get(midpoint) {
        *value
    } else {
        panic!("Midpoint was out of vector range!")
    }
}

fn i32_vec_mode(vec: &Vec<i32>) -> (i32, i32) {
    let mut tally = HashMap::new();
    let mut max_value = 0;
    let mut max_count = 0;
    tally.insert(max_value, max_count);
    for value in vec {
        let value_count = tally.entry(*value).or_insert(0);
        *value_count += 1;
        let value_count = *value_count;
        if let Some(count) = tally.get(&max_value) {
            max_count = *count;
            if value_count > max_count {
                max_value = *value;
            }
        } 
    }
    (max_value, max_count)
}

fn main() {
    let rng_i32_vec = random_vec_build(1, 100, 25);
    println!("There are {} numbers:", rng_i32_vec.len());
    for item in &rng_i32_vec {
        println!("{}", item);
    }
    println!("The mean is {}", i32_vec_mean(&rng_i32_vec));
    println!("The median is {}", i32_vec_median(&rng_i32_vec));
    let (mode, count) = i32_vec_mode(&rng_i32_vec);
    println!("The mode is {} which occured {} times", mode, count);
}

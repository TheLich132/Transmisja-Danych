#![allow(dead_code)]
#![allow(non_snake_case)]

use std::f64::consts::PI;

pub fn interpolate(vector: &[f64], n_points: usize) -> Vec<f64> {
    if n_points == 0 {
        return vector.to_vec();
    }
    
    let mut vector_interp: Vec<f64> = vec!();

    for i in 0..vector.len() - 1 {
        let p_start: f64 = vector[i];
        let p_end: f64 = vector[i + 1];

        vector_interp.push(p_start);

        for j in 1..=n_points {
            let fraction: f64 = j as f64 / (n_points as f64 + 1.0);
            let value_interp: f64 = p_start + fraction * (p_end - p_start);
            vector_interp.push(value_interp);
        }
    }

    vector_interp.push(*vector.last().unwrap());
    vector_interp
}

pub fn str_to_bin(str: &str) -> Vec<String>{
    str.chars().clone().map(|c: char| format!("{:b}", c as u32)).collect()
}

pub fn ask(msg: &str, t: &[f64], f_n: &f64, A_1: &f64, A_2: &f64) -> Vec<f64>{
    let mut to_return: Vec<f64> = vec![0.0; t.len()];
    let mut n: usize = 0;
    for i in 0..t.len() {
        if msg.chars().nth(n) == Some('0'){
            to_return[i] = A_1 * f64::sin(2.*PI * f_n * t[i]);
        } else {
            to_return[i] = A_2 * f64::sin(2.*PI * f_n * t[i]);
        }
        
        if (i != 0) && i % (t.len()/msg.len()) == 0{
            n+=1;
        }
    }
    to_return
}

pub fn psk(msg: &str, t: &[f64], f_n: &f64) -> Vec<f64>{
    let mut to_return: Vec<f64> = vec![0.0; t.len()];
    let mut n: usize = 0;
    for i in 0..t.len() {
        if msg.chars().nth(n) == Some('0'){
            to_return[i] = f64::sin(2.*PI * f_n * t[i]);
        } else {
            to_return[i] = f64::sin(2.*PI * f_n * t[i] + PI);
        }

        if (i != 0) && i % (t.len()/msg.len()) == 0{
            n+=1;
        }
    }
    to_return
}

pub fn fsk(msg: &str, t: &[f64], f_n1: &f64, f_n2: &f64) -> Vec<f64>{
    let mut to_return: Vec<f64> = vec![0.0; t.len()];
    let mut n: usize = 0;
    for i in 0..t.len() {
        if msg.chars().nth(n) == Some('0'){
            to_return[i] = f64::sin(2.*PI * f_n1 * t[i]);
        } else {
            to_return[i] = f64::sin(2.*PI * f_n2 * t[i]);
        }
        
        if (i != 0) && i % (t.len()/msg.len()) == 0{
            n+=1;
        }
    }
    to_return
}

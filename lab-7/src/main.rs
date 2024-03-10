#![allow(dead_code)]
use rand::Rng;
use gnuplot::{Figure, AxesCommon};

mod dft;
mod hamming;
mod modulation;
mod demodulation;

use core::f64::consts::E;

const M: usize = 4;
const TB: f64 = 0.1;
const W: f64 = 2.;
const F_S: f64 = 24000.;
const F_N1: f64 = (W - 1.)/TB;
const A_1: f64 = 1.;
const A_2: f64 = 0.5;

const MODES: [&str; 3] = ["ASK", "PSK", "FSK"];

fn arange(start: f64, end: f64, step: f64) -> Vec<f64>{
    let mut to_return: Vec<f64> = Vec::new();
    to_return.push(start);
    let mut num: f64 = start;
    
    while num < end{
        num += step;
        to_return.push(num);
    }

    to_return
}

fn hamming_distance(str1: &str, str2: &str) -> usize{
    str1.chars().zip(str2.chars()).filter(|(c1,c2)| c1 != c2).count()
}

// Funkcja pomocnicza z ChatGPT, zmodyfikowana pod moje potrzeby
fn normalize_vector(vector: &mut [f64]) {
    let min_val: f64 = vector.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_val: f64 = vector.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    for value in vector {
        *value = (*value - min_val) / (max_val - min_val) * 2.0 - 1.0;
    }
}

// Funkcja pomocnicza z ChatGPT, zmodyfikowana pod moje potrzeby
fn generate_white_noise(length: usize) -> Vec<f64> {
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    let mut noise: Vec<f64> = Vec::with_capacity(length);

    for _ in 0..length {
        let sample = rng.gen::<f64>() * 2.0 - 1.0; // Generowanie liczby losowej z zakresu [-1.0, 1.0)
        noise.push(sample);
    }

    noise
}

fn noise_signal(signal: &mut [f64], noise: &[f64], alpha: &f64){
    for (x, y) in signal.iter_mut().zip(noise.iter()){
        *x += alpha * y;
    }
}

fn suppression(signal: &mut [f64], t: &[f64], beta: &f64){
    for (x, t_val) in signal.iter_mut().zip(t.iter()){
        *x *= f64::powf(E, (-1.* beta) * *t_val);
    }
}

fn nadajnik(msg: &str, mode: &str) -> (Vec<f64>, Vec<f64>){
    let f_n: f64 = W * f64::powi(TB, -1);

    let mut msg_split: Vec<Vec<usize>> = vec![];
    let mut temp: Vec<usize> = vec![];
    for c in msg.chars().enumerate(){
        temp.push(c.1.to_digit(10).unwrap() as usize);
        if c.0 != 0 && (c.0+1) % M == 0{
            msg_split.push(temp.clone());
            temp.clear();
        }
    }

    let mut msg_ham: Vec<String> = vec![];
    for v in msg_split{
        let temp: Vec<usize> = hamming::ham_74_coding(&v);
        msg_ham.push(temp.iter().map(|x| x.to_string()).collect());
    }

    let msg_ham_joined: String = msg_ham.join("");

    let tc: f64 = TB * msg_ham_joined.len() as f64;
    let n: f64 = tc * F_S;
    let t: Vec<f64> = (0..n as usize).map(|x| x as f64/F_S).collect();

    let msg_return: Vec<f64>;
    if mode == "ASK" {
        msg_return = modulation::ask(&msg_ham_joined, &t, &f_n, &A_1, &A_2)
    } else if mode == "PSK" {
        msg_return = modulation::psk(&msg_ham_joined, &t, &f_n);
    } else {
        msg_return = modulation::fsk(&msg_ham_joined, &t, &f_n, &F_N1);
    }

    (msg_return, t)
}

fn odbiornik(msg: &[f64], t: &[f64], mode: &str) -> String {
    let f_n: f64 = W * f64::powi(TB, -1);

    let msg_demod: String;
    if mode == "ASK" {
        msg_demod = demodulation::demod_ask(msg, &f_n, &F_S, &TB, t)
    } else if mode == "PSK" {
        msg_demod = demodulation::demod_psk(msg, &f_n, &F_S, &TB, t);
    } else {
        msg_demod = demodulation::demod_fsk(msg, &f_n, &F_N1, &F_S, &TB, t);
    }
    
    let mut msg_demod_split: Vec<Vec<usize>> = vec![];
    let mut temp: Vec<usize> = vec![];
    for c in msg_demod.chars().enumerate(){
        temp.push(c.1.to_digit(10).unwrap() as usize);
        if c.0 != 0 && (c.0+1) % 7 == 0{
            msg_demod_split.push(temp.clone());
            temp.clear();
        }
    }

    let mut msg_end_vec: Vec<String> = vec![];
    for v in msg_demod_split{
        let temp: Vec<usize> = hamming::ham_74_decoding(&v);
        msg_end_vec.push(temp.iter().map(|x| x.to_string()).collect());
    }

    let msg_end: String = msg_end_vec.join("");

    msg_end
}

fn zad1(msg_to_bin: &str, mode: &str){
    // ---------- ZAD. 1 ----------
    let (mut msg_sender, t) = nadajnik(msg_to_bin, mode);

    normalize_vector(&mut msg_sender);
    let msg_recived: String = odbiornik(&msg_sender, &t, mode);

    println!("{:?} = msg_og, len={}", msg_to_bin, msg_to_bin.len());
    println!("\"{}\" = msg_recived", msg_recived);

    let counter = hamming_distance(msg_to_bin, &msg_recived);

    let ber: f64 = counter as f64 / msg_to_bin.len() as f64;
    println!("BER = {:.2}%", ber * 100.);
}

fn zad2(msg_to_bin: &str, noise_generator: &dyn Fn(usize) -> Vec<f64>, mode: &str, alpha: &f64) -> f64{
    // ---------- ZAD. 2 ----------
    let (mut msg_sender, t) = nadajnik(msg_to_bin, mode);
    let mut noise: Vec<f64> = noise_generator(msg_sender.len());
    normalize_vector(&mut noise);

    normalize_vector(&mut msg_sender);
    noise_signal(&mut msg_sender, &noise, alpha);

    let msg_recived: String = odbiornik(&msg_sender, &t, mode);

    let counter = hamming_distance(msg_to_bin, &msg_recived);

    let ber: f64 = counter as f64 / msg_to_bin.len() as f64;
    ber
}

fn zad3(msg_to_bin: &str, mode: &str, beta: &f64) -> f64{
    // ---------- ZAD. 3 ----------
    let (mut msg_sender, t) = nadajnik(msg_to_bin, mode);

    suppression(&mut msg_sender, &t, beta);

    let msg_recived: String = odbiornik(&msg_sender, &t, mode);

    let counter: usize = hamming_distance(msg_to_bin, &msg_recived);

    let ber: f64 = counter as f64 / msg_to_bin.len() as f64;
    ber
}

fn zad4_1(msg_to_bin: &str, noise_generator: &dyn Fn(usize) -> Vec<f64>, mode: &str, alpha: &f64, beta: &f64) -> f64{
    // ---------- ZAD. 4 (I + II) ----------
    let (mut msg_sender, t) = nadajnik(msg_to_bin, mode);

    let mut noise: Vec<f64> = noise_generator(msg_sender.len());

    normalize_vector(&mut noise);
    normalize_vector(&mut msg_sender);

    noise_signal(&mut msg_sender, &noise, alpha); // Moduł I
    suppression(&mut msg_sender, &t, beta); // Moduł II

    let msg_recived: String = odbiornik(&msg_sender, &t, mode);

    let counter: usize = hamming_distance(msg_to_bin, &msg_recived);

    let ber: f64 = counter as f64 / msg_to_bin.len() as f64;
    ber
}

fn zad4_2(msg_to_bin: &str, noise_generator: &dyn Fn(usize) -> Vec<f64>, mode: &str, alpha: &f64, beta: &f64) -> f64{
    // ---------- ZAD. 4 (II + I) ----------
    let (mut msg_sender, t) = nadajnik(msg_to_bin, mode);

    let mut noise: Vec<f64> = noise_generator(msg_sender.len());

    normalize_vector(&mut noise);
    normalize_vector(&mut msg_sender);

    suppression(&mut msg_sender, &t, beta); // Moduł II
    noise_signal(&mut msg_sender, &noise, alpha); // Moduł I

    let msg_recived: String = odbiornik(&msg_sender, &t, mode);

    let counter = hamming_distance(msg_to_bin, &msg_recived);

    let ber: f64 = counter as f64 / msg_to_bin.len() as f64;
    ber
}

fn test(msg_to_bin: &str, noise_generator: &dyn Fn(usize) -> Vec<f64>, alpha: &f64){
    // ---------- ZAD. 2 ----------
    let (mut msg_sender, t) = nadajnik(msg_to_bin, "ASK");
    let noise: Vec<f64> = noise_generator(msg_sender.len());

    normalize_vector(&mut msg_sender);
    noise_signal(&mut msg_sender, &noise, alpha);

    let msg_recived: String = odbiornik(&msg_sender, &t, "ASK");

    println!("msg = {:?}, len={}", msg_to_bin, msg_to_bin.len());
    println!("msg_recived = {}", msg_recived);

    let counter: usize = hamming_distance(msg_to_bin, &msg_recived);

    let ber: f64 = counter as f64 / msg_to_bin.len() as f64;
    println!("BER = {:.2}%", ber * 100.);
}

fn main() {
    let msg: String = "XXXQWERTYXXX".to_string();
    let msg_to_bin: String = (modulation::str_to_bin(&msg)).join("");

    let alpha_vec: Vec<f64> = arange(0., 2., 0.1);
    let beta_vec: Vec<f64> = arange(0., 20., 0.5);
    
    let mut fg: Figure = Figure::new();

    for mode in MODES.iter(){
        println!("Modulation mode: {}", mode);

        let zad2_wyniki: Vec<f64> = alpha_vec.iter()
                                    .map(|alpha| zad2(&msg_to_bin, &generate_white_noise, mode, alpha))
                                    .collect();
                                
        println!("zad2_wyniki = {:.5?}", zad2_wyniki);

        fg.clear_axes();
        fg.axes2d()
            .lines(&alpha_vec, &zad2_wyniki, &[])
            .set_x_label("Alpha", &[])
            .set_y_label("BER", &[])
            .set_x_range(gnuplot::Fix(0.), gnuplot::Fix(alpha_vec[alpha_vec.len()-1]))
            .set_y_range(gnuplot::Fix(-0.1), gnuplot::Fix(1.));
        let name: String = format!("./figures/{}/zad2_ber_alpha.png", mode);
        fg.save_to_png(name, 1920, 1080).unwrap();


        let zad3_wyniki: Vec<f64> = beta_vec.iter()
                                    .map(|beta| zad3(&msg_to_bin, mode, beta))
                                    .collect();
        println!("zad3_wyniki = {:.5?}", zad3_wyniki);

        fg.clear_axes();
        fg.axes2d()
            .lines(&beta_vec, &zad3_wyniki, &[])
            .set_x_label("Beta", &[])
            .set_y_label("BER", &[])
            .set_x_range(gnuplot::Fix(0.), gnuplot::Fix(beta_vec[beta_vec.len()-1]))
            .set_y_range(gnuplot::Fix(-0.1), gnuplot::Fix(1.)); 
        let name: String = format!("./figures/{}/zad3_ber_beta.png", mode);
        fg.save_to_png(name , 1920, 1080).unwrap();



        let mut zad4_1_wyniki: Vec<Vec<f64>> = Vec::new();
        let mut zad4_2_wyniki: Vec<Vec<f64>> = Vec::new();
        for alpha in alpha_vec.iter(){
            let mut temp1: Vec<f64> = Vec::new();
            let mut temp2: Vec<f64> = Vec::new();
            for beta in beta_vec.iter(){
                temp1.push(zad4_1(&msg_to_bin, &generate_white_noise, mode, alpha, beta));
                temp2.push(zad4_2(&msg_to_bin, &generate_white_noise, mode, alpha, beta));
            }
            zad4_1_wyniki.push(temp1);
            zad4_2_wyniki.push(temp2);
        }

        println!("zad4_1_wyniki = {:.5?}", zad4_1_wyniki);
        println!("zad4_2_wyniki = {:.5?}", zad4_2_wyniki);

        let mut temp1: Vec<f64> = Vec::new();
        let mut temp2: Vec<f64> = Vec::new();
        let mut temp3: Vec<f64> = Vec::new();
        for (alpha, row) in alpha_vec.iter().zip(zad4_1_wyniki.iter()){
            for (beta, value) in beta_vec.iter().zip(row.iter()){
                temp1.push(*alpha);
                temp2.push(*beta);
                temp3.push(*value);
            }
        }

        fg.clear_axes();

        fg.axes3d()
        .lines(&temp1, &temp2, &temp3, &[])
        .set_x_label("Alpha", &[])
        .set_y_label("Beta", &[])
        .set_z_label("BER", &[])
        .set_z_range(gnuplot::Fix(0.), gnuplot::Fix(1.));
        let name: String = format!("./figures/{}/zad4_1_ber_alpha+beta.png", mode);
        fg.save_to_png(name, 1920, 1080).unwrap();


        let mut temp3: Vec<f64> = Vec::new();
        for row in zad4_2_wyniki.iter(){
            for value in row.iter(){
                temp3.push(*value);
            }
        }

        fg.clear_axes();

        fg.axes3d()
        .lines(&temp1, &temp2, &temp3, &[])
        .set_x_label("Alpha", &[])
        .set_y_label("Beta", &[])
        .set_z_label("BER", &[])
        .set_z_range(gnuplot::Fix(0.), gnuplot::Fix(1.));
        let name: String = format!("./figures/{}/zad4_2_ber_alpha+beta.png", mode);
        fg.save_to_png(name, 1920, 1080).unwrap();
    }
}

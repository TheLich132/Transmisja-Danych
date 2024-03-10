#![allow(non_snake_case)]

use core::f64::consts::PI;

const PRINT_VECTORS: bool = false;

// Funkcja do sprawdzania czy rozmiar tablicy jest potęgą 2
fn pow_of_2(num: i32) -> bool{
    num != 0 && num & (num-1) == 0
}

fn get_power_of_2(num: usize) -> usize{
    let power = f64::log2(num as f64).ceil() as u32;
    2_i32.pow(power) as usize
}

pub fn dft(x: &Vec<f64>) -> Vec<[f64; 2]> {
    let mut X: Vec<[f64; 2]> = Vec::new();
    let N = x.len();
    for k in x.iter().enumerate(){
        let mut suma_real = 0.0;
        let mut suma_img = 0.0;
        for (n, elem) in x.iter().enumerate().take(N){
            suma_real += elem * (2.0*PI * (n as f64) * (k.0 as f64) / (N as f64)).cos();
            suma_img += elem * (2.0*PI * (n as f64) * (k.0 as f64) / (N as f64)).sin();
        }
        let complex = [suma_real, suma_img];
        X.push(complex);
    }

    // Wyświetlanie tablicy po DFT
    if PRINT_VECTORS {
        for elem in X.iter().take(N){
            println!("{:?}", elem);
        }
    }
    X
}

pub fn fft(x: &Vec<f64>) -> Vec<[f64; 2]> {
    let mut X: Vec<[f64; 2]> = Vec::new();
    if pow_of_2(x.len() as i32){
        for elem in x{
            X.push([*elem, 0.0]);
        }
    } else {
        X.clear();
        X.resize(get_power_of_2(x.len()), [0., 0.]);
        for i in 0..x.len(){
            X[i][0] = x[i];
        }
    }
    if x.len() == 1{
        return X;
    }

    // Część rekurencyjna funkcji FFT
    fft_rec(&mut X);

    // Wyświetlanie tablicy po FFT
    if PRINT_VECTORS {
        for elem in X.iter(){
            println!("{:?}", elem);
        }
    }
    X
}


// Przerobiony kod z chatgpt, by działał bez liczb zespolonych
fn fft_rec(X: &mut Vec<[f64; 2]>){
    let N = X.len();
    if N == 1{
        return;
    }

    let mut even: Vec<[f64; 2]> = Vec::with_capacity(N/2);
    let mut odd: Vec<[f64; 2]> = Vec::with_capacity(N/2);

    for (i, item) in X.iter().enumerate().take(N) {
        if i % 2 == 0 {
            even.push([item[0], item[1]]);
        } else {
            odd.push([item[0], item[1]]);
        }
    }

    fft_rec(&mut even);
    fft_rec(&mut odd);

    for k in 0..N/2{
        let t_re = (-2.0*PI * (k as f64) / (N as f64)).cos();
        let t_im = (-2.0*PI * (k as f64) / (N as f64)).sin();

        X[k][0] = even[k][0] + t_re * odd[k][0] - t_im * odd[k][1];
        X[k][1] = even[k][1] + t_re * odd[k][1] + t_im * odd[k][0];

        X[k+N/2][0] = even[k][0] - t_re * odd[k][0] + t_im * odd[k][1];
        X[k+N/2][1] = even[k][1] - t_re * odd[k][1] - t_im * odd[k][0];
    }
}

pub fn compare(X_dft: &[[f64; 2]], X_fft: &[[f64; 2]]){
    println!("Liczba próbek: {} - Porównywanie...", X_dft.len());
    for n in 0..X_dft.len(){
        let &dft_re = &X_dft[n][0];
        let &dft_im = &X_dft[n][1];

        let &fft_re = &X_fft[n][0];
        let &fft_im = &X_fft[n][1];

        assert!( dft_re - fft_re < 0.001, "próbka {n} [re]: {dft_re} != {fft_re}" );
        assert!( dft_im - fft_im < 0.001, "próbka {n}[im]: {dft_im} != {fft_im}" );
    }
}

pub fn regen_amplitude(x: &[[f64; 2]], M: &mut Vec<f64>, M_prim: &mut Vec<f64>, fk: &mut Vec<f64>, f_s: &f64){
    let N: f64 = x.len() as f64;
    if M.len() + M_prim.len() + fk.len() != 0 {
        M.clear();
        M.resize(0, 0.);
        M_prim.clear();
        M_prim.resize(0, 0.);
        fk.clear();
        fk.resize(0, 0.);
    }  

    for k in 0..(N/2.0) as usize{
        M.push((x[k][0].powf(2.0) + x[k][1].powf(2.0)).sqrt());
        M_prim.push(10.0 * (M[k].log10()));
        fk.push(k as f64 * f_s / N);
    }
}

pub fn bandwidth(M_prim: &[f64], fk: &[f64]) -> Vec<f64>{
    let dB_vec: [f64; 3] = [3., 6., 12.];
    let mut to_return: Vec<f64> = Vec::new();
    let max = M_prim.iter().copied().fold(f64::NAN, f64::max);
    let M_prim_lowered: Vec<f64> = M_prim.iter().copied().map(|x| x - max).collect();

    for dB in &dB_vec{
        let dB_neg = &-dB;
        let mut f_min: usize = 0;
        let mut f_max: usize = 0;

        for i in 1..M_prim_lowered.len()-1{
            if M_prim_lowered[i] >= *dB_neg || (M_prim_lowered[i-1] < *dB_neg && M_prim_lowered[i+1] > *dB_neg){
                f_min = i;
                break;
            }
        }

        let mut i = M_prim_lowered.len()-2;
        while i > 1{
            if M_prim_lowered[i] >= *dB_neg || (M_prim_lowered[i - 1] > *dB_neg && M_prim_lowered[i + 1] < *dB_neg){
                f_max = i;
                break;
            }
            i -= 1;
        }
        to_return.push(f64::ceil(fk[f_max] - fk[f_min]));
    }
    println!("{:?}", to_return);
    to_return
}
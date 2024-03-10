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

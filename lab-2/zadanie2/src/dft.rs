#![allow(non_snake_case)]

use core::f32::consts::PI;
use std::time::Instant;

const PRINT_VECTORS: bool = false;

// Funkcja do sprawdzania czy rozmiar tablicy jest potęgą 2
fn pow_of_2(num: i32) -> bool{
    if num != 0 && num & (num-1) == 0{
        return true;
    } else {
        return false;
    }
}

pub fn dft(x: &Vec<f32>, X: &mut Vec<[f32; 2]>){
    let N = x.len();
    for k in 0..N{
        let mut suma_real = 0.0;
        let mut suma_img = 0.0;
        for n in 0..N{
            suma_real += x[n] * (-2.0*PI * (n as f32) * (k as f32) / (N as f32)).cos();
            suma_img += x[n] * (-2.0*PI * (n as f32) * (k as f32) / (N as f32)).sin();
        }
        let complex = [suma_real, suma_img];
        X.push(complex);
    }

    // Wyświetlanie tablicy po DFT
    if PRINT_VECTORS {
        for k in 0..N{
            println!("{:?}", X[k]);
        }
    }
}

pub fn fft(x: &Vec<f32>, X: &mut Vec<[f32; 2]>){
    assert!(pow_of_2(x.len() as i32), "Liczba obiektów w tablicy z próbkami w dziedzinie czasu musi być wielokrotnością 2!");

    for elem in x{
        X.push([*elem, 0.0]);
    }
    if x.len() == 1{
        return;
    }

    // Część rekurencyjna funkcji FFT
    fft_rec(X);

    // Wyświetlanie tablicy po FFT
    if PRINT_VECTORS {
        for elem in X{
            println!("{:?}", elem);
        }
    }
}


// Przerobiony kod z chatgpt, by działał bez liczb zespolonych
fn fft_rec(X: &mut Vec<[f32; 2]>){
    let N = X.len();
    if N == 1{
        return;
    }

    let mut even: Vec<[f32; 2]> = Vec::with_capacity(N/2);
    let mut odd: Vec<[f32; 2]> = Vec::with_capacity(N/2);

    for i in 0..N {
        if i % 2 == 0 {
            even.push([X[i][0], X[i][1]]);
        } else {
            odd.push([X[i][0], X[i][1]]);
        }
    }

    fft_rec(&mut even);
    fft_rec(&mut odd);

    for k in 0..N/2{
        let t_re = (-2.0*PI * (k as f32) / (N as f32)).cos();
        let t_im = (-2.0*PI * (k as f32) / (N as f32)).sin();

        X[k][0] = even[k][0] + t_re * odd[k][0] - t_im * odd[k][1];
        X[k][1] = even[k][1] + t_re * odd[k][1] + t_im * odd[k][0];

        X[k+N/2][0] = even[k][0] - t_re * odd[k][0] + t_im * odd[k][1];
        X[k+N/2][1] = even[k][1] - t_re * odd[k][1] - t_im * odd[k][0];
    }
}

pub fn compare(X_dft: &Vec<[f32; 2]>, X_fft: &Vec<[f32; 2]>){
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

// Funkcja pomocnicza do obliczania wartości pierwiastków jedności.
fn get_twiddle_factor(n: usize, k: usize) -> [f32; 2] {
    let exponent = -2.0 * PI * k as f32 / n as f32;
    [exponent.cos(), exponent.sin()]
}

// Rekurencyjna funkcja FFT.
pub fn fft2(x: &mut Vec<[f32; 2]>) {
    let n = x.len();
    if n == 1 {
        return; // Jeśli n = 1, zakończ rekurencję.
    }
    let mut even: Vec<[f32; 2]> = Vec::with_capacity(n / 2);
    let mut odd: Vec<[f32; 2]> = Vec::with_capacity(n / 2);
    for i in 0..n / 2 {
        even.push([x[2 * i][0], 0.0]); // Część rzeczywista to x[2*i], część urojona to 0.
        odd.push([x[2 * i + 1][0], 0.0]); // Część rzeczywista to x[2*i+1], część urojona to 0.
    }
    fft2(&mut even); // Rekurencyjnie oblicz FFT dla części parzystych.
    fft2(&mut odd); // Rekurencyjnie oblicz FFT dla części nieparzystych.
    // Łączenie części parzystych i nieparzystych z wykorzystaniem twiddle factors.
    for k in 0..n / 2 {
        let twiddle = get_twiddle_factor(n, k);
        let mut t_real = twiddle[0] * odd[k][0] - twiddle[1] * odd[k][1];
        let mut t_imag = twiddle[0] * odd[k][1] + twiddle[1] * odd[k][0];
        x[k] = [even[k][0] + t_real, even[k][1] + t_imag];
        x[k + n / 2] = [even[k][0] - t_real, even[k][1] - t_imag];
    }
}

fn main() {
    const MAX_ORDER: i32 = 15;

    for o in 1..MAX_ORDER {
        let N = 1 << o; // rozmiar problemu (potęga 2)
        println!("Liczba próbek: {N}");
        let mut x: Vec<f32> = vec!();
        for n in 0..N {
            x.push(n as f32/ N as f32); // dane funkcji liniowej
        }


        let mut X_dft: Vec<[f32; 2]> = Vec::with_capacity(x.len()); // Układ vectora: [[wart_real, wart_img], [wart_real, wart_img], [wart_real, wart_img], ....]
        let mut X_fft: Vec<[f32; 2]> = Vec::with_capacity(x.len()); // Układ vectora: [[wart_real, wart_img], [wart_real, wart_img], [wart_real, wart_img], ....]
        let mut start = Instant::now();
        dft(&x, &mut X_dft);
        let dft_time = start.elapsed();

        start = Instant::now();
        fft(&x, &mut X_fft);
        let fft_time = start.elapsed();

        //compare(&X_dft, &X_fft);

        println!("Czas wykonania DFT dla tablicy {} elementów: {:?}", x.len(), dft_time);
        println!("Czas wykonania FFT dla tablicy {} elementów: {:?}", x.len(), fft_time);
        print!("\n\n");
    }
}

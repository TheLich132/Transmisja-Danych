#![allow(non_snake_case)]

mod dft;
use core::f32::consts::PI;
use gnuplot::{Figure, AxesCommon};

fn main() {
    let fs = 2000; // Częstotliwość próbkowania
    let Tc = 2;
    let N = fs*Tc; // Liczba próbek
    let f = 100; // Częstotliwość sygnału

    let mut x = vec![];
    for i in 0..=N{
        let t = i as f32 / fs as f32;
        let value = (2.0*PI * f as f32 * t).sin();
        x.push(value);
    }

    let mut X_dft: Vec<[f32; 2]> = Vec::with_capacity(x.len()); // Układ vectora: [[wart_real, wart_img], [wart_real, wart_img], [wart_real, wart_img], ....]
    dft::dft(&x, &mut X_dft);

    let mut M: Vec<f32> = vec![]; // Widmo amplitudowe
    let mut M_prim: Vec<f32> = vec![]; // Widmo amplitudowe w skali decybelowej
    let mut fk: Vec<f32> = vec![];
    for k in 0..N/2 - 1{
        M.push((X_dft[k][0].powf(2.0) + X_dft[k][1].powf(2.0)).sqrt());
        M_prim.push(10.0 * (M[k].log10()));
        fk.push(k as f32 * fs as f32 / N as f32);
    }
    let mut fg = Figure::new();
    fg.axes2d()
        .set_title("Widmo amplitudowe - sinus", &[])
        .set_x_label("częstotliwość [Hz]", &[])
        .set_y_label("amplituda widma [dB]", &[])
        .lines(
            &fk,
            &M_prim,
            &[]
        );
    fg.show().unwrap();
}

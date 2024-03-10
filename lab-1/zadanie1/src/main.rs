use core::f32::consts::PI;
use gnuplot::{Figure, AxesCommon};

// Lekka pomoc od Marcin Czułada
fn zad1() { // Wzór 12
    let fs = 16000; // Częstotliwość próbkowania
    let Tc = 5; // Czas trwania sygnału (w sekundach)
    let N = &fs*&Tc; // Liczba próbek
    let f = 5;

    let mut x = vec![];
    for i in (0..N).map(|x| x as f32){
        let t: f32 = i/(fs as f32);
        //let value = (2.0*PI * (f as f32) * t.powf(2.0)).sin().powf(13.0).abs() + (2.0*PI * t).cos(); // Funkcja, którą porównywałem czy dobrze zapisuję funkcje z zadaniem Marcina
        let value = (PI * (f as f32) / 4.0 * t).sin() + (1.4*PI * (f as f32) * t).sin() - (0.3*PI * (f as f32) * t).cos();
        x.push(value);
    }
    let t: Vec<f32> = (0..N).map(|i| (i as f32)/(fs as f32)).collect();
    let mut fg = Figure::new();
    fg.axes2d()
        .set_title("x(t) = sin(Π * f / 4 * t) + sin(1.4*Π * f * t) - cos(0.3*Π * f * t)", &[])
        .set_x_label("t", &[])
        .set_y_label("x(t)", &[])
        .lines(
            &t,
            &x,
            &[]
        );
    fg.show().unwrap();
}


fn main() {
    zad1();
}

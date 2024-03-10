// Lp. 9 z tabeli 4

use core::f32::consts::PI;
use gnuplot::{Figure, AxesCommon};

static fs: i32 = 22050; // Częstotliwość próbkowania (22.05 kHz)
static Tc: i32 = 1; // Czas trwania sygnału (w sekundach)
static N : i32= fs * Tc; // Liczba próbek
static H : [i32; 3] = [2, 20, 60];

fn main() {
    let mut counter: i32 = 0;
    for h_end in &H{
        counter += 1;
        let mut b: Vec<f32> = vec![];
        for i in (0..N).map(|x| x as f32){
            let t: f32 = i/(fs as f32);
            let mut value: f32 = 0.0;
            for h in (0..*h_end).map(|x| x as f32){
                value += (-1.0_f32).powf(h) * ( (2.0*PI * h * t).sin() + (6.0*PI * h * t).cos());
            }
            b.push(value.abs());
        }
        let t: Vec<f32> = (0..(N as i32)).map(|i| (i as f32)/(fs as f32)).collect();
        let mut fg = Figure::new();
        let title: String = format!("Wykres funkcji b{}(t)", counter);
        let y_label: String = format!("b{}(t)", counter);
        fg.axes2d()
            .set_title(&title, &[])
            .set_x_label("t", &[])
            .set_y_label(&y_label, &[])
            .lines(
                &t,
                &*b,
                &[]
            );
        fg.show().unwrap();
    }
}

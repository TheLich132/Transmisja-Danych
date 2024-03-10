use core::f32::consts::PI;
use gnuplot::{Figure, AxesCommon};

static fs: i32 = 16000; // Częstotliwość próbkowania
static Tc: f32 = 4.5; // Czas trwania sygnału (w sekundach)
static f : i32= 5;


// Wzory z Lp. 6 z tabeli 3
fn main() {
    let N : f32= ((fs as f32) * Tc).ceil(); // Liczba próbek
    let mut u: Vec<f32> = vec![];
    for i in (0..(N as i32)).map(|x| x as f32){
        let t: f32 = i/(fs as f32);
        let mut value = 0.0;
        if t >= 0.0  && t < 1.8 {
            value = (-t / 2.0) * (20.0 * t.powf(3.0) - 18.0 * t.powf(2.0)).sin();
        } else if t >= 1.8 && t < 3.0 {
            value = (5.0*PI * t).cos() * (12.0*PI * t.powf(2.0)).sin();
        } else if t >= 3.0 && t < 4.5 {
            value = (t-3.0)/3.0 * ((12.0 - t)*PI * t.powf(2.0)).sin();
        }
        u.push(value);
    }
    let t: Vec<f32> = (0..(N as i32)).map(|i| (i as f32)/(fs as f32)).collect();
    let mut fg = Figure::new();
    fg.axes2d()
        .set_title("Wykres funkcji u(t)", &[])
        .set_x_label("t", &[])
        .set_y_label("u(t)", &[])
        .lines(
            &t,
            &*u,
            &[]
        );
    fg.show().unwrap();
}

use core::f32::consts::PI;
use gnuplot::{Figure, AxesCommon};

static fs: i32 = 16000; // Częstotliwość próbkowania
static Tc: i32 = 5; // Czas trwania sygnału (w sekundach)
static N : i32= fs * Tc; // Liczba próbek
static f : i32= 5;

// Funkcja generująca próbki funkcji x(t)
// Wzór 12
fn gen_x(x:&mut Vec<f32>){
    for i in (0..N).map(|x| x as f32){
        let t: f32 = i/(fs as f32);
        let value = (PI * (f as f32) / 4.0 * t).sin() + (1.4*PI * (f as f32) * t).sin() - (0.3*PI * (f as f32) * t).cos();
        x.push(value);
    }
}

// Wzory na y(t), z(t), v(t) z Lp. 8, tabela 2
// Funkcja generująca próbki funkcji y(t)
fn gen_y(y:&mut Vec<f32>){
    for i in (0..N).map(|x| x as f32){
        let t: f32 = i/(fs as f32);
        let value = ( (-1.0) * (t / ( (3.0 * t.powf(2.0)).cos().abs() )) ).exp();
        y.push(value);
    }
    let t: Vec<f32> = (0..N).map(|i| (i as f32)/(fs as f32)).collect();
    let mut fg = Figure::new();
    fg.axes2d()
        .set_title("Wykres funkcji y(t)", &[])
        .set_x_label("t", &[])
        .set_y_label("y(t)", &[])
        .lines(
            &t,
            &*y,
            &[]
        );
    fg.show().unwrap();
}

// Funkcja generująca próbki funkcji z(t)
fn gen_z(z:&mut Vec<f32>, x:&mut Vec<f32>, y:&mut Vec<f32>){
    for i in (0..N).map(|x| x as f32){
        let t: f32 = i/(fs as f32);
        let x_index = x[i as usize];
        let y_index = y[i as usize];
        let value = x_index + ( 0.17 * (y_index + x_index).abs().log2() ) + ( 3.0 * (4.0 * t.powf(2.0)).sin() );
        z.push(value);
    }
    let t: Vec<f32> = (0..N).map(|i| (i as f32)/(fs as f32)).collect();
    let mut fg = Figure::new();
    fg.axes2d()
        .set_title("Wykres funkcji z(t)", &[])
        .set_x_label("t", &[])
        .set_y_label("z(t)", &[])
        .lines(
            &t,
            &*z,
            &[]
        );
    fg.show().unwrap();
}

// Funkcja generująca próbki funkcji v(t)
fn gen_v(v:&mut Vec<f32>, x:&mut Vec<f32>, y:&mut Vec<f32>, z:&mut Vec<f32>){
    for i in (0..N).map(|x| x as f32){
        let x_index = x[i as usize];
        let y_index = y[i as usize];
        let z_index = z[i as usize];
        let value = ((1.0 - x_index) * (1.0 - y_index) * (1.0 - z_index)).abs().sqrt();
        v.push(value);
    }
    let t: Vec<f32> = (0..N).map(|i| (i as f32)/(fs as f32)).collect();
    let mut fg = Figure::new();
    fg.axes2d()
        .set_title("Wykres funkcji v(t)", &[])
        .set_x_label("t", &[])
        .set_y_label("v(t)", &[])
        .lines(
            &t,
            &*v,
            &[]
        );
    fg.show().unwrap();
}


fn main() {
    let mut x: Vec<f32> = vec![];
    let mut y: Vec<f32> = vec![];
    let mut z: Vec<f32> = vec![];
    let mut v: Vec<f32> = vec![];
    gen_x(&mut x);
    gen_y(&mut y);
    gen_z(&mut z, &mut x, &mut y);
    gen_v(&mut v, &mut x, &mut y, &mut z);
}

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

mod dft;
use core::f64::consts::PI;
use std::vec;
use gnuplot::{Figure, AxesCommon};

const T_c: f64 = 3.;
const f_m: f64 = 250.; // Częstotliwość sygnału
const f_n: f64 = 5000.;
const f_s: f64 = 5. * [f_m, f_n][(f_m < f_n) as usize]; // Częstotliwość próbkowania

fn am_mod(k: f64, m: f64, t: f64) -> f64{
    (k * m + 1.) * f64::cos(2. * PI * f_n * t)
}

fn pm_mod(k: f64, m: f64, t: f64) -> f64{
    f64::cos(2. * PI * f_n * t + k * m)
}

fn fm_mod(k: f64, m: f64, t: f64) -> f64{
    f64::cos(2. * PI * f_n * t + k/f_m * m)
}

fn func_sin(t: f64) -> f64{
    f64::sin(2. * PI * f_m * t)
}

fn regen_amplitude(x: &Vec<[f64; 2]>, M: &mut Vec<f64>, M_prim: &mut Vec<f64>, fk: &mut Vec<f64>){
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

fn bandwidth(M_prim: &[f64], fk: &[f64]) -> Vec<f64>{
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

fn main() {
    let N: f64 = f_s*T_c; // Liczba próbek

    let k_aa: f64 = 0.75;
    let k_ab: f64 = 6.66;
    let k_ac: f64 = 51.01;

    let k_pa: f64 = 0.10;
    let k_pb: f64 = PI / 2.;
    let k_pc: f64 = 3.*PI;

    let k_fa: f64 = 0.99;
    let k_fb: f64 = PI - 1.;
    let k_fc: f64 = 3.*PI;

    let mut t: Vec<f64> = vec![];
    let mut m: Vec<f64> = vec![];
    for i in 0..N as usize{
        let t_val: f64 = i as f64 / f_s;
        t.push(t_val);
        m.push(func_sin(t_val));
    }
    let mut z_aa: Vec<f64> = vec![];
    let mut z_ab: Vec<f64> = vec![];
    let mut z_ac: Vec<f64> = vec![];

    let mut z_pa: Vec<f64> = vec![];
    let mut z_pb: Vec<f64> = vec![];
    let mut z_pc: Vec<f64> = vec![];

    let mut z_fa: Vec<f64> = vec![];
    let mut z_fb: Vec<f64> = vec![];
    let mut z_fc: Vec<f64> = vec![];
    for i in 0..N as usize{
        z_aa.push(am_mod(k_aa, m[i], t[i]));
        z_ab.push(am_mod(k_ab, m[i], t[i]));
        z_ac.push(am_mod(k_ac, m[i], t[i]));

        z_pa.push(pm_mod(k_pa, m[i], t[i]));
        z_pb.push(pm_mod(k_pb, m[i], t[i]));
        z_pc.push(pm_mod(k_pc, m[i], t[i]));

        z_fa.push(fm_mod(k_fa, m[i], t[i]));
        z_fb.push(fm_mod(k_fb, m[i], t[i]));
        z_fc.push(fm_mod(k_fc, m[i], t[i]));
    }

    // FFT na sygnałach i generowanie wykresów 
    {
        let mut M: Vec<f64> = vec![]; // Widmo amplitudowe
        let mut M_prim: Vec<f64> = vec![]; // Widmo amplitudowe w skali decybelowej
        let mut fk: Vec<f64> = vec![];
        let mut title: String;

        // modulacja amplitudy
        {
            let z_aa_fft = dft::fft(&z_aa);

            regen_amplitude(&z_aa_fft, &mut M, &mut M_prim, &mut fk);
            bandwidth(&M_prim, &fk);

            let mut fg = Figure::new();
            title = format!("Widmo amplitudowe - modulacja amplitudy - k = {}", k_aa);
            fg.axes2d()
                .set_title(&title, &[])
                .set_x_label("częstotliwość [Hz]", &[])
                .set_y_label("amplituda widma [dB]", &[])
                .set_y_range(gnuplot::Fix(-20.), gnuplot::Fix(50.))
                .lines(
                    &fk,
                    &M_prim,
                    &[]
                );
            fg.save_to_png("./za_a.png", 1920, 1080).unwrap();


            let z_ab_fft = dft::fft(&z_ab);

            regen_amplitude(&z_ab_fft, &mut M, &mut M_prim, &mut fk);
            bandwidth(&M_prim, &fk);

            let mut fg = Figure::new();
            title = format!("Widmo amplitudowe - modulacja amplitudy - k = {}", k_ab);
            fg.axes2d()
                .set_title(&title, &[])
                .set_x_label("częstotliwość [Hz]", &[])
                .set_y_label("amplituda widma [dB]", &[])
                .set_y_range(gnuplot::Fix(-20.), gnuplot::Fix(50.))
                .lines(
                    &fk,
                    &M_prim,
                    &[]
                );
            fg.save_to_png("./za_b.png", 1920, 1080).unwrap();


            let z_ac_fft = dft::fft(&z_ac);

            regen_amplitude(&z_ac_fft, &mut M, &mut M_prim, &mut fk);
            bandwidth(&M_prim, &fk);

            let mut fg = Figure::new();
            title = format!("Widmo amplitudowe - modulacja amplitudy - k = {}", k_ac);
            fg.axes2d()
                .set_title(&title, &[])
                .set_x_label("częstotliwość [Hz]", &[])
                .set_y_label("amplituda widma [dB]", &[])
                .set_y_range(gnuplot::Fix(-20.), gnuplot::Fix(60.))
                .lines(
                    &fk,
                    &M_prim,
                    &[]
                );
            fg.save_to_png("./za_c.png", 1920, 1080).unwrap();
        }

        // modulacja fazy
        {
            let z_pa_fft = dft::fft(&z_pa);

            regen_amplitude(&z_pa_fft, &mut M, &mut M_prim, &mut fk);
            bandwidth(&M_prim, &fk);

            let mut fg = Figure::new();
            title = format!("Widmo amplitudowe - modulacja fazy - k = {}", k_pa);
            
            fg.axes2d()
                .set_title(&title, &[])
                .set_x_label("częstotliwość [Hz]", &[])
                .set_y_label("amplituda widma [dB]", &[])
                .set_y_range(gnuplot::Fix(-20.), gnuplot::Fix(50.))
                .lines(
                    &fk,
                    &M_prim,
                    &[]
                );
            fg.save_to_png("./zp_a.png", 1920, 1080).unwrap();


            let z_pb_fft = dft::fft(&z_pb);

            regen_amplitude(&z_pb_fft, &mut M, &mut M_prim, &mut fk);
            bandwidth(&M_prim, &fk);

            let mut fg = Figure::new();
            title = format!("Widmo amplitudowe - modulacja fazy - k = {}", k_pb);
            
            fg.axes2d()
                .set_title(&title, &[])
                .set_x_label("częstotliwość [Hz]", &[])
                .set_y_label("amplituda widma [dB]", &[])
                .set_y_range(gnuplot::Fix(-20.), gnuplot::Fix(50.))
                .lines(
                    &fk,
                    &M_prim,
                    &[]
                );
            fg.save_to_png("./zp_b.png", 1920, 1080).unwrap();


            let z_pc_fft = dft::fft(&z_pc);

            regen_amplitude(&z_pc_fft, &mut M, &mut M_prim, &mut fk);
            bandwidth(&M_prim, &fk);
            
            let mut fg = Figure::new();
            title = format!("Widmo amplitudowe - modulacja fazy - k = {}", k_pc);
            
            fg.axes2d()
                .set_title(&title, &[])
                .set_x_label("częstotliwość [Hz]", &[])
                .set_y_label("amplituda widma [dB]", &[])
                .set_y_range(gnuplot::Fix(-20.), gnuplot::Fix(50.))
                .lines(
                    &fk,
                    &M_prim,
                    &[]
                );
            fg.save_to_png("./zp_c.png", 1920, 1080).unwrap();
        }
        
        // modulacja częstotliwości
        {
            let z_fa_fft = dft::fft(&z_fa);

            regen_amplitude(&z_fa_fft, &mut M, &mut M_prim, &mut fk);
            bandwidth(&M_prim, &fk);

            let mut fg = Figure::new();
            title = format!("Widmo amplitudowe - modulacja częstotliwości - k = {}", k_fa);
            
            fg.axes2d()
                .set_title(&title, &[])
                .set_x_label("częstotliwość [Hz]", &[])
                .set_y_label("amplituda widma [dB]", &[])
                .set_y_range(gnuplot::Fix(-20.), gnuplot::Fix(50.))
                .lines(
                    &fk,
                    &M_prim,
                    &[]
                );
            fg.save_to_png("./zf_a.png", 1920, 1080).unwrap();


            let z_fb_fft = dft::fft(&z_fb);

            regen_amplitude(&z_fb_fft, &mut M, &mut M_prim, &mut fk);
            bandwidth(&M_prim, &fk);

            let mut fg = Figure::new();
            title = format!("Widmo amplitudowe - modulacja częstotliwości - k = {}", k_fb);
            
            fg.axes2d()
                .set_title(&title, &[])
                .set_x_label("częstotliwość [Hz]", &[])
                .set_y_label("amplituda widma [dB]", &[])
                .set_y_range(gnuplot::Fix(-20.), gnuplot::Fix(50.))
                .lines(
                    &fk,
                    &M_prim,
                    &[]
                );
            fg.save_to_png("./zf_b.png", 1920, 1080).unwrap();


            let z_fc_fft = dft::fft(&z_fc);

            regen_amplitude(&z_fc_fft, &mut M, &mut M_prim, &mut fk);
            bandwidth(&M_prim, &fk);

            let mut fg = Figure::new();
            title = format!("Widmo amplitudowe - modulacja częstotliwości - k = {}", k_fc);
            
            fg.axes2d()
                .set_title(&title, &[])
                .set_x_label("częstotliwość [Hz]", &[])
                .set_y_label("amplituda widma [dB]", &[])
                .set_y_range(gnuplot::Fix(-20.), gnuplot::Fix(50.))
                .lines(
                    &fk,
                    &M_prim,
                    &[]
                );
            fg.save_to_png("./zf_c.png", 1920, 1080).unwrap();
        }
    }

}

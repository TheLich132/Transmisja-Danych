#![allow(dead_code)]
#![allow(non_snake_case)]

mod dft;
use gnuplot::{Figure, AxesCommon};

use std::f64::consts::PI;



fn interpolate(vector: &[f64], n_points: usize) -> Vec<f64> {
    if n_points == 0 {
        return vector.to_vec();
    }
    
    let mut vector_interp = vec!();

    for i in 0..vector.len() - 1 {
        let p_start = vector[i];
        let p_end = vector[i + 1];

        vector_interp.push(p_start);

        for j in 1..=n_points {
            let fraction = j as f64 / (n_points as f64 + 1.0);
            let value_interp = p_start + fraction * (p_end - p_start);
            vector_interp.push(value_interp);
        }
    }

    vector_interp.push(*vector.last().unwrap());
    vector_interp
}

fn str_to_bin(str: &str) -> Vec<String>{
    str.chars().clone().map(|c| format!("{:b}", c as u32)).collect()
}

fn ask(msg: &[String], t: &[f64], f_n: &f64, A_1: &f64, A_2: &f64) -> Vec<f64>{
    let mut to_return: Vec<f64> = vec![0.0; t.len()];
    let msg_flat = msg.join("");
    let mut n: usize = 0;
    for i in 0..t.len() {
        if msg_flat.chars().nth(n).unwrap() == '0'{
            to_return[i] = A_1 * f64::sin(2.*PI * f_n * t[i]);
        } else {
            to_return[i] = A_2 * f64::sin(2.*PI * f_n * t[i]);
        }
        
        if (i != 0) && i % (t.len()/msg_flat.len()) == 0{
            n+=1;
        }
    }
    to_return
}

fn psk(msg: &[String], t: &[f64], f_n: &f64) -> Vec<f64>{
    let mut to_return: Vec<f64> = vec![0.0; t.len()];
    let msg_flat = msg.join("");
    let mut n: usize = 0;
    for i in 0..t.len() {
        if msg_flat.chars().nth(n).unwrap() == '0'{
            to_return[i] = f64::sin(2.*PI * f_n * t[i]);
        } else {
            to_return[i] = f64::sin(2.*PI * f_n * t[i] + PI);
        }
        
        if (i != 0) && i % (t.len()/msg_flat.len())== 0{
            n+=1;
        }
    }
    to_return
}

fn fsk(msg: &[String], t: &[f64], f_n1: &f64, f_n2: &f64) -> Vec<f64>{
    let mut to_return: Vec<f64> = vec![0.0; t.len()];
    let msg_flat = msg.join("");
    let mut n: usize = 0;
    for i in 0..t.len() {
        if msg_flat.chars().nth(n).unwrap() == '0'{
            to_return[i] = f64::sin(2.*PI * f_n1 * t[i]);
        } else {
            to_return[i] = f64::sin(2.*PI * f_n2 * t[i]);
        }
        
        if (i != 0) && i % (t.len()/msg_flat.len()) == 0{
            n+=1;
        }
    }
    to_return
}

fn zad3(ask_vec: &[f64], psk_vec: &[f64], fsk_vec: &[f64], t: &[f64], str_msg: &String){
    let mut title: String = format!("ASK\nWiadomość: {}", str_msg);
    let mut fg = Figure::new();
    fg.axes2d()
            .set_title(&title, &[])
            .lines(
                t,
                ask_vec,
                &[]
            );
    fg.save_to_png("za.png", 1920, 1080).unwrap();

    fg.clear_axes();
    title = format!("PSK\nWiadomość: {}", str_msg);
    fg.axes2d()
            .set_title(&title, &[])
            .lines(
                t,
                psk_vec,
                &[]
            );
    fg.save_to_png("zp.png", 1920, 1080).unwrap();

    fg.clear_axes();
    title = format!("FSK\nWiadomość: {}", str_msg);
    fg.axes2d()
            .set_title(&title, &[])
            .lines(
                t,
                fsk_vec,
                &[]
            );
    fg.save_to_png("zf.png", 1920, 1080).unwrap();
}

fn zad4(ask_fft: &[[f64; 2]], psk_fft: &[[f64; 2]], fsk_fft: &[[f64; 2]], f_s: &f64){
    let mut M: Vec<f64> = vec![]; // Widmo amplitudowe
    let mut M_prim: Vec<f64> = vec![]; // Widmo amplitudowe w skali decybelowej
    let mut fk: Vec<f64> = vec![];
    let mut title: String;

    dft::regen_amplitude(ask_fft, &mut M, &mut M_prim, &mut fk, f_s);
    let mut fg = Figure::new();
    title = ("Widmo amplitudowe - ASK").to_string();
    fg.axes2d()
        .set_title(&title, &[])
        .set_x_label("częstotliwość [Hz]", &[])
        .set_y_label("amplituda widma [dB]", &[])
        //.set_y_range(gnuplot::Fix(-20.), gnuplot::Fix(50.))
        .set_x_range(gnuplot::Fix(0.), gnuplot::Fix(200.))
        .set_x_log(Some(2.))
        .lines(
            &fk,
            &M_prim,
            &[]
        );
    fg.save_to_png("./za_widmo.png", 1920, 1080).unwrap();

    fg.clear_axes();
    dft::regen_amplitude(psk_fft, &mut M, &mut M_prim, &mut fk, f_s);
    title = ("Widmo amplitudowe - PSK").to_string();
    fg.axes2d()
        .set_title(&title, &[])
        .set_x_label("częstotliwość [Hz]", &[])
        .set_y_label("amplituda widma [dB]", &[])
        //.set_y_range(gnuplot::Fix(-20.), gnuplot::Fix(50.))
        .set_x_range(gnuplot::Fix(0.), gnuplot::Fix(200.))
        .set_x_log(Some(2.))
        .lines(
            &fk,
            &M_prim,
            &[]
        );
    fg.save_to_png("./zp_widmo.png", 1920, 1080).unwrap();

    fg.clear_axes();
    dft::regen_amplitude(fsk_fft, &mut M, &mut M_prim, &mut fk, f_s);
    title = ("Widmo amplitudowe - FSK").to_string();
    fg.axes2d()
        .set_title(&title, &[])
        .set_x_label("częstotliwość [Hz]", &[])
        .set_y_label("amplituda widma [dB]", &[])
        //.set_y_range(gnuplot::Fix(-20.), gnuplot::Fix(50.))
        .set_x_range(gnuplot::Fix(0.), gnuplot::Fix(200.))
        .set_x_log(Some(2.))
        .lines(
            &fk,
            &M_prim,
            &[]
        );
    fg.save_to_png("./zf_widmo.png", 1920, 1080).unwrap();
}

fn zad5(ask_fft: &[[f64; 2]], psk_fft: &[[f64; 2]], fsk_fft: &[[f64; 2]], f_s: &f64){
    let mut M: Vec<f64> = vec![]; // Widmo amplitudowe
    let mut M_prim: Vec<f64> = vec![]; // Widmo amplitudowe w skali decybelowej
    let mut fk: Vec<f64> = vec![];

    dft::regen_amplitude(ask_fft, &mut M, &mut M_prim, &mut fk, f_s);
    let mut M_prim_interp: Vec<f64> = interpolate(&M_prim, 5);
    let mut fk_interp: Vec<f64> = interpolate(&fk, 5);
    dft::bandwidth(&M_prim_interp, &fk_interp);

    dft::regen_amplitude(psk_fft, &mut M, &mut M_prim, &mut fk, f_s);
    M_prim_interp = interpolate(&M_prim, 5);
    fk_interp = interpolate(&fk, 5);
    dft::bandwidth(&M_prim_interp, &fk_interp);

    dft::regen_amplitude(fsk_fft, &mut M, &mut M_prim, &mut fk, f_s);
    M_prim_interp = interpolate(&M_prim, 5);
    fk_interp = interpolate(&fk, 5);
    dft::bandwidth(&M_prim_interp, &fk_interp);

}

fn main() {
    let str_msg: String = "ABC".to_string();
    let msg: Vec<String> = str_to_bin(&str_msg);

    let T_b: f64 = 0.1;
    let T_c: f64 = T_b * 7.*msg.len() as f64;
    let A_1: f64 = 100.;
    let A_2: f64 = 500.;
    let W: f64 = 2.;
    let f_n: f64 = W * f64::powi(T_b, -1);
    let f_n1: f64 = (W - 1.)/T_b;
    let f_s: f64 = 8000.;
    let N: f64 = T_c * f_s;
    let t: Vec<f64> = (0..N as usize).map(|x| x as f64/f_s).collect();

    let ask_vec = ask(&msg, &t, &f_n, &A_1, &A_2);
    let psk_vec = psk(&msg, &t, &f_n);
    let fsk_vec = fsk(&msg, &t, &f_n1, &f_n);

    //zad3(&ask_vec, &psk_vec, &fsk_vec, &t, &str_msg);
    
    let ask_fft = dft::fft(&ask_vec);
    let psk_fft = dft::fft(&psk_vec);
    let fsk_fft = dft::fft(&fsk_vec);

    //zad4(&ask_fft, &psk_fft, &fsk_fft, &f_s);

    zad5(&ask_fft, &psk_fft, &fsk_fft, &f_s);

}

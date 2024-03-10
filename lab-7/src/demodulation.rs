#![allow(dead_code)]
#![allow(non_snake_case)]

use gnuplot::Figure;

use std::f64::consts::PI;

const DRAW_PLOTS: bool = false; // NIE WŁĄCZAĆ DLA BIAŁEGO SZUMU

fn bin_to_str(msg_bin: &[Vec<usize>]) -> String {
    let mut msg: String = String::new();
    for char in msg_bin.iter(){
        let mut char_bin: String = String::new();

        for bit in char.iter(){
            char_bin.push_str(&bit.to_string());
        }

        let decimal: u8 = u8::from_str_radix(&char_bin, 2).unwrap_or(0);
        if let Some(ch) = std::char::from_u32(decimal.into()) {
            msg.push(ch);
        }
    }
    msg
}

fn ct_to_bin(ct: &[usize], f_s: &f64, T_b: &f64) -> Vec<Vec<usize>>{
    let mut msg_bin: Vec<Vec<usize>> = vec!();
    let mut char_bin: Vec<usize> = vec!();
    let mut count1: usize = 0;
    let mut count0: usize = 0;
    for (i, x) in ct.iter().enumerate() {
        if *x == 0 {
            count0 += 1;
        } else {
            count1 += 1;
        }
        if i != 0 && i.rem_euclid((f_s * T_b) as usize - 1) == 0{
            if count0 > count1{
                char_bin.push(0);
            } else {
                char_bin.push(1);
            }
            if char_bin.len() == 7{
                msg_bin.push(char_bin.clone());
                char_bin.clear();
            }
            count0 = 0;
            count1 = 0;
        }
    }
    msg_bin
}

pub fn demod_ask(zt: &[f64], f_n: &f64, f_s: &f64, T_b: &f64, t: &[f64]) -> String{
    let mut fg: Figure = Figure::new();
    if DRAW_PLOTS {
        fg.axes2d()
        .lines(t, zt, &[]);
        fg.save_to_png("ask_z.png", 1920, 1080).unwrap();
    }

    let xt: Vec<f64> = t.iter().enumerate().map(|(i, &x)| 1. * f64::sin(2. * PI * f_n * x) * zt[i]).collect();
    if DRAW_PLOTS {
        fg.clear_axes();
        fg.axes2d()
        .lines(t, &xt, &[]);
        fg.save_to_png("ask_x.png", 1920, 1080).unwrap();
    }

    let mut pt: Vec<f64> = vec!();
    let mut suma: f64 = 0.;
    for (i, x) in xt.iter().enumerate(){
        suma += x;
        if i.rem_euclid((f_s * T_b) as usize) == 0{
            suma = 0.;
        }
        pt.push(suma);
    }

    if DRAW_PLOTS {
        fg.clear_axes();
        fg.axes2d()
        .lines(t, &pt, &[]);
        fg.save_to_png("ask_p.png", 1920, 1080).unwrap();
    }

    let max: f64 = pt.iter().copied().fold(f64::NAN, f64::max);
    let h: f64 = (max / 2.) - 25.;

    let ct: Vec<usize> = pt.iter().copied().map(|t| !(t > h) as usize).collect();
    if DRAW_PLOTS {
        fg.clear_axes();
        fg.axes2d()
        .lines(t, &ct, &[]);
        fg.save_to_png("ask_c.png", 1920, 1080).unwrap();
    }

    let msg_bin: Vec<Vec<usize>> = ct_to_bin(&ct, f_s, T_b);
    
    let mut msg: String = String::new();
    for char in msg_bin.iter(){
        let mut char_bin: String = String::new();

        for bit in char.iter(){
            char_bin.push_str(&bit.to_string());
        }
        msg += &char_bin;
    }
    msg
}

pub fn demod_psk(zt: &[f64], f_n: &f64, f_s: &f64, T_b: &f64, t: &[f64]) -> String{
    let mut fg: Figure = Figure::new();
    if DRAW_PLOTS {
        fg.axes2d()
        .lines(t, zt, &[]);
        fg.save_to_png("psk_z.png", 1920, 1080).unwrap();
    }

    let xt: Vec<f64> = t.iter().enumerate().map(|(i, &x)| 1. * f64::sin(2. * PI * f_n * x) * zt[i]).collect();
    if DRAW_PLOTS {
        fg.clear_axes();
        fg.axes2d()
        .lines(t, &xt, &[]);
        fg.save_to_png("psk_x.png", 1920, 1080).unwrap();
    }

    let mut pt: Vec<f64> = vec!();
    let mut suma: f64 = 0.;
    for (i, x) in xt.iter().enumerate(){
        suma += x;
        if i.rem_euclid((f_s * T_b) as usize) == 0{
            suma = 0.;
        }
        pt.push(suma);
    }
    if DRAW_PLOTS {
        fg.clear_axes();
        fg.axes2d()
        .lines(t, &pt, &[]);
        fg.save_to_png("psk_p.png", 1920, 1080).unwrap();
    }

    let ct: Vec<usize> = pt.iter().copied().map(|t| (t < 0.) as usize).collect();
    if DRAW_PLOTS {
        fg.clear_axes();
        fg.axes2d()
        .lines(t, &ct, &[]);
        fg.save_to_png("psk_c.png", 1920, 1080).unwrap();
    }

    let msg_bin: Vec<Vec<usize>> = ct_to_bin(&ct, f_s, T_b);
    let mut msg: String = String::new();

    for char in msg_bin.iter(){
        let mut char_bin: String = String::new();

        for bit in char.iter(){
            char_bin.push_str(&bit.to_string());
        }
        msg += &char_bin;
    }

    msg
}

pub fn demod_fsk(zt: &[f64], f_n1: &f64, f_n2: &f64, f_s: &f64, T_b: &f64, t: &[f64]) -> String{
    let mut fg: Figure = Figure::new();
    if DRAW_PLOTS {
        fg.axes2d()
        .lines(t, zt, &[]);
        fg.save_to_png("fsk_z.png", 1920, 1080).unwrap();
    }

    let xt1: Vec<f64> = t.iter().enumerate().map(|(i, &x)| 1. * f64::sin(2. * PI * f_n1 * x) * zt[i]).collect();
    let xt2: Vec<f64> = t.iter().enumerate().map(|(i, &x)| 1. * f64::sin(2. * PI * f_n2 * x) * zt[i]).collect();
    if DRAW_PLOTS {
        fg.clear_axes();
        fg.axes2d()
        .lines(t, &xt1, &[]);
        fg.save_to_png("fsk_x1.png", 1920, 1080).unwrap();
        fg.clear_axes();
        fg.axes2d()
        .lines(t, &xt2, &[]);
        fg.save_to_png("fsk_x2.png", 1920, 1080).unwrap();
    }

    let mut pt1: Vec<f64> = vec!();
    let mut suma: f64 = 0.;
    for (i, x) in xt1.iter().enumerate(){
        suma += x;
        if i.rem_euclid((f_s * T_b) as usize) == 0{
            suma = 0.;
        }
        pt1.push(suma);
    }
    if DRAW_PLOTS {
        fg.clear_axes();
        fg.axes2d()
        .lines(t, &pt1, &[]);
        fg.save_to_png("fsk_p1.png", 1920, 1080).unwrap();
    }

    let mut pt2: Vec<f64> = vec!();
    suma = 0.;
    for (i, x) in xt2.iter().enumerate(){
        suma += x;
        if i.rem_euclid((f_s * T_b) as usize) == 0{
            suma = 0.;
        }
        pt2.push(suma);
    }
    if DRAW_PLOTS {
        fg.clear_axes();
        fg.axes2d()
        .lines(t, &pt2, &[]);
        fg.save_to_png("fsk_p2.png", 1920, 1080).unwrap();
    }

    let pt: Vec<f64> = pt1.iter().zip(pt2.iter()).map(|(&x, &y)| -x + y).collect();
    if DRAW_PLOTS {
        fg.clear_axes();
        fg.axes2d()
        .lines(t, &pt, &[]);
        fg.save_to_png("fsk_p.png", 1920, 1080).unwrap();
    }

    let ct: Vec<usize> = pt.iter().copied().map(|t| (t > 0.) as usize).collect();
    if DRAW_PLOTS {
        fg.clear_axes();
        fg.axes2d()
        .lines(t, &ct, &[]);
        fg.save_to_png("fsk_c.png", 1920, 1080).unwrap();
    }

    let msg_bin: Vec<Vec<usize>> = ct_to_bin(&ct, f_s, T_b);
    let mut msg: String = String::new();

    for char in msg_bin.iter(){
        let mut char_bin: String = String::new();

        for bit in char.iter(){
            char_bin.push_str(&bit.to_string());
        }
        msg += &char_bin;
    }

    msg
}

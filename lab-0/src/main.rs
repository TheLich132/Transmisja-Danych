fn main() {
    use gnuplot::{Figure, AxesCommon};
    let mut x = vec![];
    for i in (0..100).map(|x| x as f32*0.1) {
        x.push(i);
    }
    let y: Vec<f32> = x.iter().map(|&i| i.sin()).collect();
    let mut fg = Figure::new();
    fg.axes2d()
        .set_title("Wykres sin(x)", &[])
        .set_x_label("x", &[])
        .set_y_label("y", &[])
        .lines(
            &x,
            &y,
            &[]
        );
    fg.show().unwrap();
}

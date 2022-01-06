use ray_tracer::{
    color::Color,
    ppm::{Ppm, P3},
};

fn main() {
    let width = 255;
    let height = 255;
    let mut ppm = Ppm::new(width, height);
    for (row, col, p3) in ppm.rows_mut().rev().enumerate().flat_map(|(row_idx, row)| {
        row.iter_mut()
            .enumerate()
            .map(move |(col_index, p3)| (row_idx, col_index, p3))
    }) {
        let c = Color::new(
            col as f64 / (width - 1) as f64,
            row as f64 / (height - 1) as f64,
            0.25,
        );
        *p3 = P3::from_color(c);
    }
    println!("{}", ppm.output());
}

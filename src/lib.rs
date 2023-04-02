extern crate statrs;
use clap::Parser;
use rand::Rng;
use rayon::prelude::*;
use statrs::statistics::*;
// use plotters::prelude::*;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use std::fmt::Write;

pub fn mt_simulate<F>(nb_iterations: usize, f: F) -> Vec<f64>
where
    F: Fn() -> usize + Sync + Send,
{
    assert!(nb_iterations > 0);
    let f = std::sync::Arc::new(f);
    let pb = ProgressBar::new(nb_iterations.try_into().unwrap());
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})",
        )
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
        })
        .progress_chars("#>-"),
    );
    let observations: Vec<_> = (0..nb_iterations)
        .into_par_iter()
        .map(|_| {
            pb.inc(1);
            f() as f64
        })
        .collect();
    pb.finish();
    observations
}

pub fn mt_simulate_no_progressbar<F>(nb_iterations: usize, f: F) -> Vec<f64>
where
    F: Fn() -> usize + Sync + Send,
{
    assert!(nb_iterations > 0);
    let f = std::sync::Arc::new(f);

    let observations: Vec<_> = (0..nb_iterations)
        .into_par_iter()
        .map(|_| f() as f64)
        .collect();
    observations
}

pub fn create_histogram(values: &[f64], num_bins: u32) -> (f64, f64, Vec<u32>) {
    let min = values.iter().fold(f64::INFINITY, |acc, &val| acc.min(val));
    let max = values
        .iter()
        .fold(f64::NEG_INFINITY, |acc, &val| acc.max(val));
    let bin_width = (max - min) / num_bins as f64;

    let mut bins = vec![0; num_bins as usize];

    for &val in values {
        let bin = ((val - min) / bin_width) as u32;
        if bin < num_bins {
            bins[bin as usize] += 1;
        }
    }

    (min, max, bins)
}

use plotters::prelude::*;
const OUT_FILE_NAME: &str = "histogram.png";

pub fn draw_histogram(histogram: &Vec<u32>, nb_bins: u32) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(OUT_FILE_NAME, (640, 480)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("Histogram", ("sans-serif", 50.0))
        .build_cartesian_2d(
            (0_u32..(nb_bins as u32)).into_segmented(),
            0_u32..(*histogram.iter().max().unwrap()),
        )?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(WHITE.mix(0.3))
        .y_desc("Count")
        .x_desc("Bucket")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .data(histogram.iter().enumerate().map(|(i, x)| (i as u32, *x))),
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);

    Ok(())
}

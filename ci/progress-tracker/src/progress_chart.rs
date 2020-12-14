use super::problems::Problems;
use super::solutions;
use chrono::{DateTime, TimeZone, Utc};
use git2::{Repository, Tree};
use plotters::chart::ChartBuilder;
use plotters::drawing::IntoDrawingArea;
use plotters::prelude::SVGBackend;
use plotters::series::LineSeries;
use plotters::style::colors;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::Path;

fn make_problem_hits_cache(problems: &Problems) -> HashMap<String, bool> {
    let mut result = HashMap::new();

    for problem in &problems.problems {
        result.insert(problem.get_id(), false);
    }

    result
}

fn get_progress(tree: &Tree, hits_cache: &mut HashMap<String, bool>) -> f64 {
    solutions::get(tree, |problem_id, _| {
        if let Some(value) = hits_cache.get_mut(problem_id) {
            *value = true;
        }
    });

    let mut hits = 0_usize;

    for value in hits_cache.values_mut() {
        if *value {
            *value = false;

            hits += 1;
        }
    }

    (hits as f64) / (hits_cache.len() as f64)
}

fn draw_chart<P: AsRef<Path>>(data: &[(DateTime<Utc>, f64)], output: P) {
    const H_MARGIN: u32 = 40;
    const V_MARGIN: u32 = 30;

    let backend = SVGBackend::new(output.as_ref(), (987, 610))
        .into_drawing_area()
        .margin(10, 0, 0, H_MARGIN);

    let mut chart = ChartBuilder::on(&backend)
        .x_label_area_size(V_MARGIN)
        .y_label_area_size(H_MARGIN)
        .build_cartesian_2d(data.first().unwrap().0..data.last().unwrap().0, 0.0..101.0)
        .unwrap();

    chart
        .configure_mesh()
        .x_labels(10)
        .x_label_formatter(&|v| v.date().format("%F").to_string())
        .y_labels(10)
        .y_label_formatter(&|v| format!("{}%", v))
        .draw()
        .unwrap();

    chart
        .draw_series(LineSeries::new(data.iter().copied(), &colors::RED))
        .unwrap();
}

pub fn draw<P: AsRef<Path>>(repository: &Repository, problems: &Problems, output: P) {
    let mut hits_cache = make_problem_hits_cache(problems);
    let mut revwalk = repository.revwalk().unwrap();

    revwalk.push_ref("HEAD").unwrap();

    let mut progress_data = revwalk
        .map(|oid| {
            let commit = repository.find_commit(oid.unwrap()).unwrap();
            let date = commit.author().when();
            let progress = get_progress(&commit.tree().unwrap(), &mut hits_cache);

            (
                Utc.timestamp(date.seconds() - i64::from(date.offset_minutes() * 60), 0),
                progress * 100.0,
            )
        })
        .collect::<Vec<_>>();

    progress_data.sort_by(|(lhs_date_time, lhs_progress), (rhs_date_time, rhs_progress)| {
        match lhs_date_time.date().cmp(&rhs_date_time.date()) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => rhs_progress.partial_cmp(&lhs_progress).unwrap(),
            Ordering::Greater => Ordering::Greater,
        }
    });

    progress_data.dedup_by_key(|(date_time, _)| date_time.date());

    draw_chart(&progress_data, output);
}

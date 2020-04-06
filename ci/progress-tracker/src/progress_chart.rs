use super::problems::Problems;
use super::solutions;
use chrono::{Date, TimeZone, Utc};
use git2::{Repository, Tree};
use plotters::chart::ChartBuilder;
use plotters::drawing::{IntoDrawingArea, SVGBackend};
use plotters::series::LineSeries;
use plotters::style::colors::RED;
use std::collections::HashMap;
use std::path::Path;

fn make_problem_hits_cache(problems: &Problems) -> HashMap<String, bool> {
    let mut result = HashMap::new();

    for problem in problems.problems.iter() {
        result.insert(problem.get_id(), false);
    }

    result
}

fn get_progress(tree: &Tree, hits_cache: &mut HashMap<String, bool>) -> f64 {
    solutions::get_solutions(tree, |problem_id, _| {
        if let Some(value) = hits_cache.get_mut(problem_id) {
            *value = true;
        }
    });

    let mut hits = 0usize;

    for value in hits_cache.values_mut() {
        if *value {
            *value = false;

            hits += 1;
        }
    }

    (hits as f64) / (hits_cache.len() as f64)
}

fn draw_chart<P: AsRef<Path>>(data: &[(Date<Utc>, f64)], output: P) {
    const H_MARGIN: u32 = 40;
    const V_MARGIN: u32 = 30;

    let backend = SVGBackend::new(output.as_ref(), (987, 610))
        .into_drawing_area()
        .margin(10, 0, 0, H_MARGIN);

    let mut chart = ChartBuilder::on(&backend)
        .x_label_area_size(V_MARGIN)
        .y_label_area_size(H_MARGIN)
        .build_ranged(data.first().unwrap().0..data.last().unwrap().0, 0.0..101.0)
        .unwrap();

    chart
        .configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .y_label_formatter(&|v| format!("{}%", v))
        .draw()
        .unwrap();

    chart.draw_series(LineSeries::new(data.iter().copied(), &RED)).unwrap();
}

pub fn draw_progress_chart<P: AsRef<Path>>(repository: &Repository, problems: &Problems, output: P) {
    let mut hits_cache = make_problem_hits_cache(problems);
    let mut revwalk = repository.revwalk().unwrap();

    revwalk.push_ref("refs/heads/master").unwrap();

    let mut progress_data = revwalk
        .map(|oid| {
            let commit = repository.find_commit(oid.unwrap()).unwrap();
            let date = commit.author().when();
            let progress = get_progress(&commit.tree().unwrap(), &mut hits_cache);

            (
                Utc.timestamp(date.seconds() - (date.offset_minutes() * 60) as i64, 0)
                    .date(),
                progress * 100.0,
            )
        })
        .collect::<Vec<_>>();

    progress_data.sort_by_key(|(date, _)| *date);
    progress_data.dedup_by_key(|(date, _)| *date);

    draw_chart(&progress_data, output);
}

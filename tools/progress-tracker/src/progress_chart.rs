use crate::problems::Problems;
use crate::solutions;
use chrono::{DateTime, TimeZone, Utc};
use git2::{Repository, Tree};
use plotters::chart::ChartBuilder;
use plotters::drawing::IntoDrawingArea;
use plotters::prelude::SVGBackend;
use plotters::series::LineSeries;
use plotters::style::{colors, Color, ShapeStyle, TextStyle};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;

fn make_problem_hits_cache(problems: &Problems) -> HashMap<String, bool> {
    let mut result = HashMap::new();

    for problem in &problems.problems {
        result.insert(problem.get_id(), false);
    }

    result
}

fn get_hits(tree: &Tree, hits_cache: &mut HashMap<String, bool>) -> usize {
    solutions::list(tree, |solution| {
        if let Some(value) = hits_cache.get_mut(&solution.problem_id) {
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

    hits
}

fn fix_svg_size(svg: &str, zoom: u32) -> String {
    let mut iter = svg.split('"');

    let part_1 = iter.next().unwrap();
    let width_str = iter.next().unwrap();
    let part_2 = iter.next().unwrap();
    let height_str = iter.next().unwrap();

    format!(
        r#"{}"{}"{}"{}{}"#,
        part_1,
        width_str.parse::<f64>().unwrap() / f64::from(zoom),
        part_2,
        height_str.parse::<f64>().unwrap() / f64::from(zoom),
        &svg[part_1.len() + width_str.len() + part_2.len() + height_str.len() + 3..]
    )
}

fn get_progress(hits: usize, total: usize) -> f64 {
    ((hits * 100) as f64) / (total as f64)
}

fn draw_chart(data: &[(DateTime<Utc>, usize)], total: usize, output: &Path) {
    const ZOOM: u32 = 16;
    const IMAGE_WIDTH: u32 = 987;
    const IMAGE_HEIGHT: u32 = 610;
    const TITLE_FONT_SIZE: f64 = 16.0;
    const LABEL_FONT_SIZE: f64 = 12.0;
    const H_MARGIN: u32 = 48;
    const V_MARGIN: u32 = 32;
    const TOP_MARGIN: u32 = 12;
    let mut svg = String::new();

    {
        let &(latest_date_time, latest_hits) = data.last().unwrap();

        let backend = SVGBackend::with_string(&mut svg, (IMAGE_WIDTH * ZOOM, IMAGE_HEIGHT * ZOOM))
            .into_drawing_area()
            .titled(
                &format!(
                    "Total: {}, Done: {}, Remaining: {}, Progress: {:.2} %, Updated: {:?}",
                    total,
                    latest_hits,
                    total - latest_hits,
                    get_progress(latest_hits, total),
                    latest_date_time
                ),
                ("sans-serif", TITLE_FONT_SIZE * 1.24 * f64::from(ZOOM)),
            )
            .unwrap()
            .margin(TOP_MARGIN * ZOOM, 0, 0, H_MARGIN * ZOOM);

        let mut chart = ChartBuilder::on(&backend)
            .x_label_area_size(V_MARGIN * ZOOM)
            .y_label_area_size(H_MARGIN * ZOOM)
            .build_cartesian_2d(data.first().unwrap().0..data.last().unwrap().0, 0.0..101.0)
            .unwrap();

        chart
            .configure_mesh()
            .axis_style(ShapeStyle::from(&colors::BLACK).stroke_width(ZOOM))
            .bold_line_style(ShapeStyle::from(&colors::BLACK.mix(0.2)).stroke_width(ZOOM))
            .label_style(TextStyle::from((
                "sans-serif",
                LABEL_FONT_SIZE * 1.24 * f64::from(ZOOM),
            )))
            .light_line_style(ShapeStyle::from(&colors::BLACK.mix(0.1)).stroke_width(ZOOM))
            .set_all_tick_mark_size(5 * ZOOM)
            .x_label_formatter(&|v| v.date().format("%F").to_string())
            .x_labels(10)
            .y_label_formatter(&|v| format!("{}%", v))
            .y_labels(10)
            .draw()
            .unwrap();

        chart
            .draw_series(LineSeries::new(
                data.iter().map(|&(date, hits)| (date, get_progress(hits, total))),
                ShapeStyle::from(&colors::RED).stroke_width(ZOOM),
            ))
            .unwrap();
    }

    fs::File::create(output)
        .unwrap()
        .write_all(fix_svg_size(&svg, ZOOM).as_bytes())
        .unwrap();
}

pub fn draw(repository: &Repository, problems: &Problems, output: &Path) {
    let mut hits_cache = make_problem_hits_cache(problems);
    let mut revwalk = repository.revwalk().unwrap();

    revwalk.push_head().unwrap();

    let mut progress_data = revwalk
        .map(|oid| {
            let commit = repository.find_commit(oid.unwrap()).unwrap();
            let date = commit.author().when();
            let hits = get_hits(&commit.tree().unwrap(), &mut hits_cache);

            (Utc.timestamp(date.seconds(), 0), hits)
        })
        .collect::<Vec<_>>();

    progress_data.sort_by_key(|(date_time, _)| *date_time);

    draw_chart(&progress_data, problems.problems.len(), output);
}

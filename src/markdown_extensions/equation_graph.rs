//! NOTE: For some reason, this only works on the "Release" profile.

use markdown_it::parser::block::{BlockRule, BlockState};
use markdown_it::{MarkdownIt, Node, NodeValue, Renderer};
use meval::{Error, Expr};
use plotters::prelude::{ChartBuilder, IntoDrawingArea, SVGBackend};
use plotters::series::LineSeries;
use plotters::style::{IntoFont, BLACK, RED};

#[derive(Debug)]
pub struct BlockEquationGraph {
    equation: String,
}

impl NodeValue for BlockEquationGraph {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        let x_max = 10.0f32;
        let x_min = -10.0f32;
        let y_max = 10.0f32;
        let y_min = -10.0f32;

        let x_total = x_max - x_min;
        let per_x_pixels = (640f32 / x_total).floor() as i32;

        let x_avg = (x_max + x_min) / 2.0;
        let y_avg = (y_max + y_min) / 2.0;

        let mut attrs_div = node.attrs.clone();
        attrs_div.push(("class", "equation-graph".into()));

        let mut root_svg = Default::default();

        fmt.cr();
        fmt.open("div", &attrs_div);

        let expr: Result<Expr, Error> = self.equation.parse();
        match expr {
            Ok(expr) => {
                {
                    let func = expr.bind("x").unwrap();
                    let root =
                        SVGBackend::with_string(&mut root_svg, (640, 480)).into_drawing_area();
                    let mut chart = ChartBuilder::on(&root);
                    let mut cartesian = chart
                        .build_cartesian_2d(x_min..x_max, y_min..y_max)
                        .unwrap();
                    cartesian.configure_mesh().draw().unwrap();
                    let iter = x_min as i32 * per_x_pixels..x_max as i32 * per_x_pixels;
                    cartesian
                        .draw_series(LineSeries::new(
                            iter.map(|x| x as f32 / per_x_pixels as f32)
                                .map(|x| (x, func(x as f64) as f32)),
                            &RED,
                        ))
                        .unwrap();
                    cartesian
                        .draw_series(LineSeries::new(
                            vec![(x_avg, y_max), (x_avg, y_min)],
                            &BLACK,
                        ))
                        .unwrap();
                    cartesian
                        .draw_series(LineSeries::new(
                            vec![(x_max, y_avg), (x_min, y_avg)],
                            &BLACK,
                        ))
                        .unwrap();
                }
                fmt.text_raw(&root_svg);
            }
            Err(err) => {
                {
                    let root =
                        SVGBackend::with_string(&mut root_svg, (640, 480)).into_drawing_area();
                    let mut chart = ChartBuilder::on(&root);
                    chart.caption(
                        format!("Error: {}", err.to_string()),
                        ("serif", 50).into_font(),
                    );
                }
                fmt.text_raw(&root_svg);
                ()
            }
        }

        fmt.close("div");
        fmt.cr();
    }
}

struct EquationGraphBlockScanner;

impl BlockRule for EquationGraphBlockScanner {
    fn run(state: &mut BlockState) -> Option<(Node, usize)> {
        let line = state.get_line(state.line).trim();
        if !line.starts_with("++") {
            return None;
        }
        if !line.ends_with("++") {
            return None;
        }

        let content = line.trim_matches('+').trim_matches('+');
        let mut prelude = String::new();
        let mut equation = String::new();
        if content.chars().nth(0).unwrap_or(' ') == '{' {
            for (i, char) in content.chars().enumerate() {
                if char == '}' {
                    equation = content[i + 1..].to_string();
                    break;
                }
                prelude.push(char);
            }
        } else {
            equation = content.to_string();
        }

        Some((
            Node::new(BlockEquationGraph {
                equation: equation.to_string(),
            }),
            1,
        ))
    }
}

pub fn add(md: &mut MarkdownIt) {
    md.block.add_rule::<EquationGraphBlockScanner>();
}

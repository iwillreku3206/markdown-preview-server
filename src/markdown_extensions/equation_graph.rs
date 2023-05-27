use std::fmt::format;

use markdown_it::parser::block::{BlockRule, BlockState};
use markdown_it::{MarkdownIt, Node, NodeValue, Renderer};
use meval::{Error, Expr};
use plotters::prelude::{
    BitMapBackend, ChartBuilder, DrawingBackend, IntoDrawingArea, PathElement, SVGBackend,
};
use plotters::series::LineSeries;
use plotters::style::full_palette::WHITE;
use plotters::style::{IntoFont, BLACK, RED};

#[derive(Debug)]
pub struct BlockEquationGraph {
    equation: String,
}

impl NodeValue for BlockEquationGraph {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        let mut attrs_div = node.attrs.clone();
        attrs_div.push(("class", "equation-graph".into()));

        let mut root_svg = Default::default();

        fmt.cr();
        fmt.open("div", &attrs_div);

        let expr: Result<Expr, Error> = self.equation.parse();
        match expr {
            Ok(expr) => {
                let func = expr.bind("x").unwrap();
                {
                    let root =
                        SVGBackend::with_string(&mut root_svg, (640, 480)).into_drawing_area();
                    let mut chart = ChartBuilder::on(&root);
                    let mut cartesian = chart
                        .x_label_area_size(30)
                        .y_label_area_size(30)
                        .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)
                        .unwrap();
                    cartesian.configure_mesh().draw().unwrap();
                    cartesian
                        .draw_series(LineSeries::new(
                            (-50..=50)
                                .map(|x| x as f32 / 50.0)
                                .map(|x| (x, func(x as f64) as f32)),
                            &RED,
                        ))
                        .unwrap()
                        .label("y = x^2")
                        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

                    cartesian
                        .configure_series_labels()
                        .background_style(&WHITE)
                        .border_style(&BLACK)
                        .draw()
                        .unwrap();
                }
                fmt.text_raw(&root_svg);
            }
            Err(err) => {
                {
                let root = SVGBackend::with_string(&mut root_svg, (640, 480)).into_drawing_area();
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

        let equation = line.trim_matches('+').trim_matches('+');

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

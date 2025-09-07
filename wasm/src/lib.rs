use svg::{
    node::element::{path::Data, Path},
    Document,
};
use wasm_bindgen::prelude::*;

mod utils;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

#[wasm_bindgen]
impl Point {
    pub fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[wasm_bindgen]
impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Line {
    points: Vec<Point>,
    pub color: Color,
    name: String,
}

#[wasm_bindgen]
impl Line {
    pub fn new(color: Color, name: String) -> Self {
        Self {
            points: Vec::new(),
            color,
            name,
        }
    }
}

#[wasm_bindgen]
pub struct TransitMap {
    lines: Vec<Line>,
}

#[wasm_bindgen]
impl TransitMap {
    pub fn new() -> Self {
        Self { lines: Vec::new() }
    }

    pub fn push_line(&mut self, line: Line) {
        self.lines.push(line);
    }

    pub fn push_point(&mut self, line: usize, point: Point) {
        self.lines[line].points.push(point);
    }

    pub fn svg(&self) -> String {
        let paths = self.lines.iter().filter_map(|line| {
            let mut data = Data::new();
            let mut iter = line.points.iter();

            let first = iter.next()?;
            data = data.move_to((first.x, first.y));

            for point in &line.points {
                data = data.line_to((point.x, point.y));
            }

            Some(
                Path::new()
                    .set("fill", "none")
                    .set(
                        "stroke",
                        format!("rgb({}, {}, {})", line.color.r, line.color.b, line.color.g),
                    )
                    .set("stroke-linecap", "round")
                    .set("stroke-linejoin", "round")
                    .set("stroke-width", 1)
                    .set("d", data),
            )
        });

        let mut document = Document::new();
        for path in paths {
            document = document.add(path);
        }

        let mut svg = document.to_string();
        utils::strip(&mut svg);

        svg
    }
}

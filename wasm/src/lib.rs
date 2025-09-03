use svg::node::element::Text;
use svg::{
    node::element::{path::Data, Path},
    Document,
};
use wasm_bindgen::prelude::*;

mod utils;

#[wasm_bindgen]
pub fn map_data() -> String {
    let text = Text::new("test")
        .set("x", 0)
        .set("y", 16)
        .set("font-size", 16)
        .set("fill", "black");

    let data = Data::new();

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 3)
        .set("d", data);

    let document = Document::new().add(path).add(text);

    let mut svg = document.to_string();
    utils::strip(&mut svg);

    svg
}

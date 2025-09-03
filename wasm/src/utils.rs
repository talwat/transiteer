/// Remove the outer <svg ...>...</svg> wrapper in-place.
/// If no valid wrapper is found, the input is cleared.
pub fn strip(svg: &mut String) {
    let lower = svg.to_ascii_lowercase();

    let open_start = match lower.find("<svg") {
        Some(idx) => idx,
        None => {
            svg.clear();
            return;
        }
    };

    // find end of the opening tag '>'
    let open_end = match lower[open_start..].find('>') {
        Some(idx) => idx + open_start,
        None => {
            svg.clear();
            return;
        }
    };

    // find closing </svg
    let close_start = match lower.rfind("</svg") {
        Some(idx) => idx,
        None => {
            svg.clear();
            return;
        }
    };

    // find end of the closing tag '>'
    let close_end = match lower[close_start..].find('>') {
        Some(idx) => idx + close_start,
        None => {
            svg.clear();
            return;
        }
    };

    if close_start <= open_end {
        svg.clear();
        return;
    }

    // Drain suffix first (after closing </svg>)
    svg.drain(close_start..=close_end);
    // Then drain prefix (up to and including the opening >)
    svg.drain(..=open_end);
}

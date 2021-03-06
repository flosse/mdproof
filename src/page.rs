
use span::{Span, PositionedSpan};

#[derive(Clone)]
pub struct Page {
    positioned_spans: Vec<PositionedSpan>,
}

impl Page {
    pub fn new() -> Self {
        Self {
            positioned_spans: vec![],
        }
    }

    pub fn render_spans(&mut self, spans: &[Span], start_x: f32, start_y: f32) {
        let mut x = start_x;
        let y = start_y;
        for span in spans {
            self.positioned_spans.push(PositionedSpan::new(span.clone(), x, y));
            x += span.width();
        }
    }

    pub fn clear(&mut self) {
        self.positioned_spans.clear();
    }

    pub fn into_vec(self) -> Vec<PositionedSpan> {
        self.positioned_spans
    }
}


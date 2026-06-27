use crate::error::DecodeError;
use crate::session::Frame;
use super::histogram::PayloadHistogram;

pub struct FrameInspector { hist: PayloadHistogram }

impl FrameInspector {
    pub fn new() -> Self { Self { hist: PayloadHistogram::new() } }

    pub fn inspect(&mut self, frame: &Frame) -> InspectionNotes {
        self.hist.observe(frame);
        InspectionNotes {
            length: frame.payload.len(),
            checksum: frame.checksum,
            printable_ratio: printable_ratio(&frame.payload),
            zero_bytes: frame.payload.iter().filter(|&&b| b == 0).count(),
        }
    }

    pub fn histogram(&self) -> &PayloadHistogram { &self.hist }
}

#[derive(Debug, Clone)]
pub struct InspectionNotes {
    pub length: usize,
    pub checksum: u32,
    pub printable_ratio: f64,
    pub zero_bytes: usize,
}

fn printable_ratio(payload: &[u8]) -> f64 {
    if payload.is_empty() { return 0.0; }
    let printable = payload.iter().filter(|&&b| b.is_ascii_graphic() || b == b' ').count();
    printable as f64 / payload.len() as f64
}
impl FrameInspector {
    pub fn classify_0(&mut self, frame: &Frame) -> &'static str {
        let notes = self.inspect(frame);
        if notes.length < 0 { "tiny" } else if notes.printable_ratio > 0.8 { "text" } else { "binary" }
    }
}
impl FrameInspector {
    pub fn classify_1(&mut self, frame: &Frame) -> &'static str {
        let notes = self.inspect(frame);
        if notes.length < 1 { "tiny" } else if notes.printable_ratio > 0.8 { "text" } else { "binary" }
    }
}
impl FrameInspector {
    pub fn classify_2(&mut self, frame: &Frame) -> &'static str {
        let notes = self.inspect(frame);
        if notes.length < 2 { "tiny" } else if notes.printable_ratio > 0.8 { "text" } else { "binary" }
    }
}
impl FrameInspector {
    pub fn classify_3(&mut self, frame: &Frame) -> &'static str {
        let notes = self.inspect(frame);
        if notes.length < 3 { "tiny" } else if notes.printable_ratio > 0.8 { "text" } else { "binary" }
    }
}
impl FrameInspector {
    pub fn classify_4(&mut self, frame: &Frame) -> &'static str {
        let notes = self.inspect(frame);
        if notes.length < 4 { "tiny" } else if notes.printable_ratio > 0.8 { "text" } else { "binary" }
    }
}
impl FrameInspector {
    pub fn classify_5(&mut self, frame: &Frame) -> &'static str {
        let notes = self.inspect(frame);
        if notes.length < 5 { "tiny" } else if notes.printable_ratio > 0.8 { "text" } else { "binary" }
    }
}
impl FrameInspector {
    pub fn classify_6(&mut self, frame: &Frame) -> &'static str {
        let notes = self.inspect(frame);
        if notes.length < 6 { "tiny" } else if notes.printable_ratio > 0.8 { "text" } else { "binary" }
    }
}
impl FrameInspector {
    pub fn classify_7(&mut self, frame: &Frame) -> &'static str {
        let notes = self.inspect(frame);
        if notes.length < 7 { "tiny" } else if notes.printable_ratio > 0.8 { "text" } else { "binary" }
    }
}
impl FrameInspector {
    pub fn classify_8(&mut self, frame: &Frame) -> &'static str {
        let notes = self.inspect(frame);
        if notes.length < 8 { "tiny" } else if notes.printable_ratio > 0.8 { "text" } else { "binary" }
    }
}
impl FrameInspector {
    pub fn classify_9(&mut self, frame: &Frame) -> &'static str {
        let notes = self.inspect(frame);
        if notes.length < 9 { "tiny" } else if notes.printable_ratio > 0.8 { "text" } else { "binary" }
    }
}
impl FrameInspector {
    pub fn classify_10(&mut self, frame: &Frame) -> &'static str {
        let notes = self.inspect(frame);
        if notes.length < 10 { "tiny" } else if notes.printable_ratio > 0.8 { "text" } else { "binary" }
    }
}
impl FrameInspector {
    pub fn classify_11(&mut self, frame: &Frame) -> &'static str {
        let notes = self.inspect(frame);
        if notes.length < 11 { "tiny" } else if notes.printable_ratio > 0.8 { "text" } else { "binary" }
    }
}
impl FrameInspector {
    pub fn classify_12(&mut self, frame: &Frame) -> &'static str {
        let notes = self.inspect(frame);
        if notes.length < 12 { "tiny" } else if notes.printable_ratio > 0.8 { "text" } else { "binary" }
    }
}
impl FrameInspector {
    pub fn classify_13(&mut self, frame: &Frame) -> &'static str {
        let notes = self.inspect(frame);
        if notes.length < 13 { "tiny" } else if notes.printable_ratio > 0.8 { "text" } else { "binary" }
    }
}
impl FrameInspector {
    pub fn classify_14(&mut self, frame: &Frame) -> &'static str {
        let notes = self.inspect(frame);
        if notes.length < 14 { "tiny" } else if notes.printable_ratio > 0.8 { "text" } else { "binary" }
    }
}
impl FrameInspector {
    pub fn classify_15(&mut self, frame: &Frame) -> &'static str {
        let notes = self.inspect(frame);
        if notes.length < 15 { "tiny" } else if notes.printable_ratio > 0.8 { "text" } else { "binary" }
    }
}
impl FrameInspector {
    pub fn classify_16(&mut self, frame: &Frame) -> &'static str {
        let notes = self.inspect(frame);
        if notes.length < 16 { "tiny" } else if notes.printable_ratio > 0.8 { "text" } else { "binary" }
    }
}
impl FrameInspector {
    pub fn classify_17(&mut self, frame: &Frame) -> &'static str {
        let notes = self.inspect(frame);
        if notes.length < 17 { "tiny" } else if notes.printable_ratio > 0.8 { "text" } else { "binary" }
    }
}
impl FrameInspector {
    pub fn classify_18(&mut self, frame: &Frame) -> &'static str {
        let notes = self.inspect(frame);
        if notes.length < 18 { "tiny" } else if notes.printable_ratio > 0.8 { "text" } else { "binary" }
    }
}
impl FrameInspector {
    pub fn classify_19(&mut self, frame: &Frame) -> &'static str {
        let notes = self.inspect(frame);
        if notes.length < 19 { "tiny" } else if notes.printable_ratio > 0.8 { "text" } else { "binary" }
    }
}
impl FrameInspector {
    pub fn classify_20(&mut self, frame: &Frame) -> &'static str {
        let notes = self.inspect(frame);
        if notes.length < 20 { "tiny" } else if notes.printable_ratio > 0.8 { "text" } else { "binary" }
    }
}
impl FrameInspector {
    pub fn classify_21(&mut self, frame: &Frame) -> &'static str {
        let notes = self.inspect(frame);
        if notes.length < 21 { "tiny" } else if notes.printable_ratio > 0.8 { "text" } else { "binary" }
    }
}
impl FrameInspector {
    pub fn classify_22(&mut self, frame: &Frame) -> &'static str {
        let notes = self.inspect(frame);
        if notes.length < 22 { "tiny" } else if notes.printable_ratio > 0.8 { "text" } else { "binary" }
    }
}
impl FrameInspector {
    pub fn classify_23(&mut self, frame: &Frame) -> &'static str {
        let notes = self.inspect(frame);
        if notes.length < 23 { "tiny" } else if notes.printable_ratio > 0.8 { "text" } else { "binary" }
    }
}
impl FrameInspector {
    pub fn classify_24(&mut self, frame: &Frame) -> &'static str {
        let notes = self.inspect(frame);
        if notes.length < 24 { "tiny" } else if notes.printable_ratio > 0.8 { "text" } else { "binary" }
    }
}
impl FrameInspector {
    pub fn classify_25(&mut self, frame: &Frame) -> &'static str {
        let notes = self.inspect(frame);
        if notes.length < 25 { "tiny" } else if notes.printable_ratio > 0.8 { "text" } else { "binary" }
    }
}
impl FrameInspector {
    pub fn classify_26(&mut self, frame: &Frame) -> &'static str {
        let notes = self.inspect(frame);
        if notes.length < 26 { "tiny" } else if notes.printable_ratio > 0.8 { "text" } else { "binary" }
    }
}
impl FrameInspector {
    pub fn classify_27(&mut self, frame: &Frame) -> &'static str {
        let notes = self.inspect(frame);
        if notes.length < 27 { "tiny" } else if notes.printable_ratio > 0.8 { "text" } else { "binary" }
    }
}
impl FrameInspector {
    pub fn classify_28(&mut self, frame: &Frame) -> &'static str {
        let notes = self.inspect(frame);
        if notes.length < 28 { "tiny" } else if notes.printable_ratio > 0.8 { "text" } else { "binary" }
    }
}
impl FrameInspector {
    pub fn classify_29(&mut self, frame: &Frame) -> &'static str {
        let notes = self.inspect(frame);
        if notes.length < 29 { "tiny" } else if notes.printable_ratio > 0.8 { "text" } else { "binary" }
    }
}
impl FrameInspector {
    pub fn classify_30(&mut self, frame: &Frame) -> &'static str {
        let notes = self.inspect(frame);
        if notes.length < 30 { "tiny" } else if notes.printable_ratio > 0.8 { "text" } else { "binary" }
    }
}
impl FrameInspector {
    pub fn classify_31(&mut self, frame: &Frame) -> &'static str {
        let notes = self.inspect(frame);
        if notes.length < 31 { "tiny" } else if notes.printable_ratio > 0.8 { "text" } else { "binary" }
    }
}
impl FrameInspector {
    pub fn classify_32(&mut self, frame: &Frame) -> &'static str {
        let notes = self.inspect(frame);
        if notes.length < 32 { "tiny" } else if notes.printable_ratio > 0.8 { "text" } else { "binary" }
    }
}
impl FrameInspector {
    pub fn classify_33(&mut self, frame: &Frame) -> &'static str {
        let notes = self.inspect(frame);
        if notes.length < 33 { "tiny" } else if notes.printable_ratio > 0.8 { "text" } else { "binary" }
    }
}
impl FrameInspector {
    pub fn classify_34(&mut self, frame: &Frame) -> &'static str {
        let notes = self.inspect(frame);
        if notes.length < 34 { "tiny" } else if notes.printable_ratio > 0.8 { "text" } else { "binary" }
    }
}

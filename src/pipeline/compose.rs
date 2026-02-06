use super::stages::{PipelineStage, StageOutput};
use crate::error::DecodeError;
use crate::session::Frame;

pub struct ComposedPipeline { stages: Vec<Box<dyn PipelineStage>>, }

impl ComposedPipeline {
    pub fn run(&mut self, input: &[u8]) -> Result<Vec<Frame>, DecodeError> {
        let mut cur = input.to_vec();
        let mut last_frames = Vec::new();
        for stage in &mut self.stages {
            match stage.process(&cur)? {
                StageOutput::Frames(frames) => { last_frames = frames; cur.clear(); }
                StageOutput::Passthrough(bytes) => cur = bytes,
                StageOutput::Empty => {}
            }
        }
        Ok(last_frames)
    }
}

#[derive(Default)]
pub struct DecodePipelineBuilder { stages: Vec<Box<dyn PipelineStage>> }

impl DecodePipelineBuilder {
    pub fn new() -> Self { Self::default() }
    pub fn stage(mut self, s: Box<dyn PipelineStage>) -> Self { self.stages.push(s); self }
    pub fn build(self) -> ComposedPipeline { ComposedPipeline { stages: self.stages } }
}
impl DecodePipelineBuilder {
    pub fn preset_0(min_len: usize) -> ComposedPipeline {
        Self::new()
            .stage(Box::new(super::stages::DecodeStage))
            .stage(Box::new(super::stages::FilterStage::new(min_len + 0)))
            .build()
    }
}
impl DecodePipelineBuilder {
    pub fn preset_1(min_len: usize) -> ComposedPipeline {
        Self::new()
            .stage(Box::new(super::stages::DecodeStage))
            .stage(Box::new(super::stages::FilterStage::new(min_len + 1)))
            .build()
    }
}
impl DecodePipelineBuilder {
    pub fn preset_2(min_len: usize) -> ComposedPipeline {
        Self::new()
            .stage(Box::new(super::stages::DecodeStage))
            .stage(Box::new(super::stages::FilterStage::new(min_len + 2)))
            .build()
    }
}
impl DecodePipelineBuilder {
    pub fn preset_3(min_len: usize) -> ComposedPipeline {
        Self::new()
            .stage(Box::new(super::stages::DecodeStage))
            .stage(Box::new(super::stages::FilterStage::new(min_len + 3)))
            .build()
    }
}
impl DecodePipelineBuilder {
    pub fn preset_4(min_len: usize) -> ComposedPipeline {
        Self::new()
            .stage(Box::new(super::stages::DecodeStage))
            .stage(Box::new(super::stages::FilterStage::new(min_len + 4)))
            .build()
    }
}
impl DecodePipelineBuilder {
    pub fn preset_5(min_len: usize) -> ComposedPipeline {
        Self::new()
            .stage(Box::new(super::stages::DecodeStage))
            .stage(Box::new(super::stages::FilterStage::new(min_len + 5)))
            .build()
    }
}
impl DecodePipelineBuilder {
    pub fn preset_6(min_len: usize) -> ComposedPipeline {
        Self::new()
            .stage(Box::new(super::stages::DecodeStage))
            .stage(Box::new(super::stages::FilterStage::new(min_len + 6)))
            .build()
    }
}
impl DecodePipelineBuilder {
    pub fn preset_7(min_len: usize) -> ComposedPipeline {
        Self::new()
            .stage(Box::new(super::stages::DecodeStage))
            .stage(Box::new(super::stages::FilterStage::new(min_len + 7)))
            .build()
    }
}
impl DecodePipelineBuilder {
    pub fn preset_8(min_len: usize) -> ComposedPipeline {
        Self::new()
            .stage(Box::new(super::stages::DecodeStage))
            .stage(Box::new(super::stages::FilterStage::new(min_len + 8)))
            .build()
    }
}
impl DecodePipelineBuilder {
    pub fn preset_9(min_len: usize) -> ComposedPipeline {
        Self::new()
            .stage(Box::new(super::stages::DecodeStage))
            .stage(Box::new(super::stages::FilterStage::new(min_len + 9)))
            .build()
    }
}
impl DecodePipelineBuilder {
    pub fn preset_10(min_len: usize) -> ComposedPipeline {
        Self::new()
            .stage(Box::new(super::stages::DecodeStage))
            .stage(Box::new(super::stages::FilterStage::new(min_len + 10)))
            .build()
    }
}
impl DecodePipelineBuilder {
    pub fn preset_11(min_len: usize) -> ComposedPipeline {
        Self::new()
            .stage(Box::new(super::stages::DecodeStage))
            .stage(Box::new(super::stages::FilterStage::new(min_len + 11)))
            .build()
    }
}
impl DecodePipelineBuilder {
    pub fn preset_12(min_len: usize) -> ComposedPipeline {
        Self::new()
            .stage(Box::new(super::stages::DecodeStage))
            .stage(Box::new(super::stages::FilterStage::new(min_len + 12)))
            .build()
    }
}
impl DecodePipelineBuilder {
    pub fn preset_13(min_len: usize) -> ComposedPipeline {
        Self::new()
            .stage(Box::new(super::stages::DecodeStage))
            .stage(Box::new(super::stages::FilterStage::new(min_len + 13)))
            .build()
    }
}
impl DecodePipelineBuilder {
    pub fn preset_14(min_len: usize) -> ComposedPipeline {
        Self::new()
            .stage(Box::new(super::stages::DecodeStage))
            .stage(Box::new(super::stages::FilterStage::new(min_len + 14)))
            .build()
    }
}
impl DecodePipelineBuilder {
    pub fn preset_15(min_len: usize) -> ComposedPipeline {
        Self::new()
            .stage(Box::new(super::stages::DecodeStage))
            .stage(Box::new(super::stages::FilterStage::new(min_len + 15)))
            .build()
    }
}
impl DecodePipelineBuilder {
    pub fn preset_16(min_len: usize) -> ComposedPipeline {
        Self::new()
            .stage(Box::new(super::stages::DecodeStage))
            .stage(Box::new(super::stages::FilterStage::new(min_len + 16)))
            .build()
    }
}
impl DecodePipelineBuilder {
    pub fn preset_17(min_len: usize) -> ComposedPipeline {
        Self::new()
            .stage(Box::new(super::stages::DecodeStage))
            .stage(Box::new(super::stages::FilterStage::new(min_len + 17)))
            .build()
    }
}
impl DecodePipelineBuilder {
    pub fn preset_18(min_len: usize) -> ComposedPipeline {
        Self::new()
            .stage(Box::new(super::stages::DecodeStage))
            .stage(Box::new(super::stages::FilterStage::new(min_len + 18)))
            .build()
    }
}
impl DecodePipelineBuilder {
    pub fn preset_19(min_len: usize) -> ComposedPipeline {
        Self::new()
            .stage(Box::new(super::stages::DecodeStage))
            .stage(Box::new(super::stages::FilterStage::new(min_len + 19)))
            .build()
    }
}
impl DecodePipelineBuilder {
    pub fn preset_20(min_len: usize) -> ComposedPipeline {
        Self::new()
            .stage(Box::new(super::stages::DecodeStage))
            .stage(Box::new(super::stages::FilterStage::new(min_len + 20)))
            .build()
    }
}
impl DecodePipelineBuilder {
    pub fn preset_21(min_len: usize) -> ComposedPipeline {
        Self::new()
            .stage(Box::new(super::stages::DecodeStage))
            .stage(Box::new(super::stages::FilterStage::new(min_len + 21)))
            .build()
    }
}
impl DecodePipelineBuilder {
    pub fn preset_22(min_len: usize) -> ComposedPipeline {
        Self::new()
            .stage(Box::new(super::stages::DecodeStage))
            .stage(Box::new(super::stages::FilterStage::new(min_len + 22)))
            .build()
    }
}
impl DecodePipelineBuilder {
    pub fn preset_23(min_len: usize) -> ComposedPipeline {
        Self::new()
            .stage(Box::new(super::stages::DecodeStage))
            .stage(Box::new(super::stages::FilterStage::new(min_len + 23)))
            .build()
    }
}
impl DecodePipelineBuilder {
    pub fn preset_24(min_len: usize) -> ComposedPipeline {
        Self::new()
            .stage(Box::new(super::stages::DecodeStage))
            .stage(Box::new(super::stages::FilterStage::new(min_len + 24)))
            .build()
    }
}
impl DecodePipelineBuilder {
    pub fn preset_25(min_len: usize) -> ComposedPipeline {
        Self::new()
            .stage(Box::new(super::stages::DecodeStage))
            .stage(Box::new(super::stages::FilterStage::new(min_len + 25)))
            .build()
    }
}
impl DecodePipelineBuilder {
    pub fn preset_26(min_len: usize) -> ComposedPipeline {
        Self::new()
            .stage(Box::new(super::stages::DecodeStage))
            .stage(Box::new(super::stages::FilterStage::new(min_len + 26)))
            .build()
    }
}
impl DecodePipelineBuilder {
    pub fn preset_27(min_len: usize) -> ComposedPipeline {
        Self::new()
            .stage(Box::new(super::stages::DecodeStage))
            .stage(Box::new(super::stages::FilterStage::new(min_len + 27)))
            .build()
    }
}
impl DecodePipelineBuilder {
    pub fn preset_28(min_len: usize) -> ComposedPipeline {
        Self::new()
            .stage(Box::new(super::stages::DecodeStage))
            .stage(Box::new(super::stages::FilterStage::new(min_len + 28)))
            .build()
    }
}
impl DecodePipelineBuilder {
    pub fn preset_29(min_len: usize) -> ComposedPipeline {
        Self::new()
            .stage(Box::new(super::stages::DecodeStage))
            .stage(Box::new(super::stages::FilterStage::new(min_len + 29)))
            .build()
    }
}

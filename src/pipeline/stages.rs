use crate::error::DecodeError;
use crate::session::Frame;

#[derive(Debug, Clone)]
pub enum StageOutput {
    Frames(Vec<Frame>),
    Passthrough(Vec<u8>),
    Empty,
}

pub trait PipelineStage: Send + Sync {
    fn name(&self) -> &str;
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError>;
}

pub struct DecodeStage;
impl PipelineStage for DecodeStage {
    fn name(&self) -> &str { "decode" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        Ok(StageOutput::Frames(crate::decode_all(input)?))
    }
}

pub struct FilterStage { min_len: usize }
impl FilterStage {
    pub fn new(min_len: usize) -> Self { Self { min_len } }
}
impl PipelineStage for FilterStage {
    fn name(&self) -> &str { "filter" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let frames = crate::decode_all(input)?;
        Ok(StageOutput::Frames(frames.into_iter().filter(|f| f.payload.len() >= self.min_len).collect()))
    }
}
pub struct MapStage0 { pub add: u8 }
impl PipelineStage for MapStage0 {
    fn name(&self) -> &str { "map_0" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage1 { pub add: u8 }
impl PipelineStage for MapStage1 {
    fn name(&self) -> &str { "map_1" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage2 { pub add: u8 }
impl PipelineStage for MapStage2 {
    fn name(&self) -> &str { "map_2" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage3 { pub add: u8 }
impl PipelineStage for MapStage3 {
    fn name(&self) -> &str { "map_3" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage4 { pub add: u8 }
impl PipelineStage for MapStage4 {
    fn name(&self) -> &str { "map_4" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage5 { pub add: u8 }
impl PipelineStage for MapStage5 {
    fn name(&self) -> &str { "map_5" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage6 { pub add: u8 }
impl PipelineStage for MapStage6 {
    fn name(&self) -> &str { "map_6" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage7 { pub add: u8 }
impl PipelineStage for MapStage7 {
    fn name(&self) -> &str { "map_7" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage8 { pub add: u8 }
impl PipelineStage for MapStage8 {
    fn name(&self) -> &str { "map_8" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage9 { pub add: u8 }
impl PipelineStage for MapStage9 {
    fn name(&self) -> &str { "map_9" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage10 { pub add: u8 }
impl PipelineStage for MapStage10 {
    fn name(&self) -> &str { "map_10" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage11 { pub add: u8 }
impl PipelineStage for MapStage11 {
    fn name(&self) -> &str { "map_11" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage12 { pub add: u8 }
impl PipelineStage for MapStage12 {
    fn name(&self) -> &str { "map_12" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage13 { pub add: u8 }
impl PipelineStage for MapStage13 {
    fn name(&self) -> &str { "map_13" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage14 { pub add: u8 }
impl PipelineStage for MapStage14 {
    fn name(&self) -> &str { "map_14" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage15 { pub add: u8 }
impl PipelineStage for MapStage15 {
    fn name(&self) -> &str { "map_15" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage16 { pub add: u8 }
impl PipelineStage for MapStage16 {
    fn name(&self) -> &str { "map_16" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage17 { pub add: u8 }
impl PipelineStage for MapStage17 {
    fn name(&self) -> &str { "map_17" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage18 { pub add: u8 }
impl PipelineStage for MapStage18 {
    fn name(&self) -> &str { "map_18" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage19 { pub add: u8 }
impl PipelineStage for MapStage19 {
    fn name(&self) -> &str { "map_19" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage20 { pub add: u8 }
impl PipelineStage for MapStage20 {
    fn name(&self) -> &str { "map_20" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage21 { pub add: u8 }
impl PipelineStage for MapStage21 {
    fn name(&self) -> &str { "map_21" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage22 { pub add: u8 }
impl PipelineStage for MapStage22 {
    fn name(&self) -> &str { "map_22" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage23 { pub add: u8 }
impl PipelineStage for MapStage23 {
    fn name(&self) -> &str { "map_23" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage24 { pub add: u8 }
impl PipelineStage for MapStage24 {
    fn name(&self) -> &str { "map_24" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage25 { pub add: u8 }
impl PipelineStage for MapStage25 {
    fn name(&self) -> &str { "map_25" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage26 { pub add: u8 }
impl PipelineStage for MapStage26 {
    fn name(&self) -> &str { "map_26" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage27 { pub add: u8 }
impl PipelineStage for MapStage27 {
    fn name(&self) -> &str { "map_27" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage28 { pub add: u8 }
impl PipelineStage for MapStage28 {
    fn name(&self) -> &str { "map_28" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage29 { pub add: u8 }
impl PipelineStage for MapStage29 {
    fn name(&self) -> &str { "map_29" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage30 { pub add: u8 }
impl PipelineStage for MapStage30 {
    fn name(&self) -> &str { "map_30" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage31 { pub add: u8 }
impl PipelineStage for MapStage31 {
    fn name(&self) -> &str { "map_31" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage32 { pub add: u8 }
impl PipelineStage for MapStage32 {
    fn name(&self) -> &str { "map_32" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage33 { pub add: u8 }
impl PipelineStage for MapStage33 {
    fn name(&self) -> &str { "map_33" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage34 { pub add: u8 }
impl PipelineStage for MapStage34 {
    fn name(&self) -> &str { "map_34" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage35 { pub add: u8 }
impl PipelineStage for MapStage35 {
    fn name(&self) -> &str { "map_35" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage36 { pub add: u8 }
impl PipelineStage for MapStage36 {
    fn name(&self) -> &str { "map_36" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage37 { pub add: u8 }
impl PipelineStage for MapStage37 {
    fn name(&self) -> &str { "map_37" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage38 { pub add: u8 }
impl PipelineStage for MapStage38 {
    fn name(&self) -> &str { "map_38" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage39 { pub add: u8 }
impl PipelineStage for MapStage39 {
    fn name(&self) -> &str { "map_39" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage40 { pub add: u8 }
impl PipelineStage for MapStage40 {
    fn name(&self) -> &str { "map_40" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage41 { pub add: u8 }
impl PipelineStage for MapStage41 {
    fn name(&self) -> &str { "map_41" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage42 { pub add: u8 }
impl PipelineStage for MapStage42 {
    fn name(&self) -> &str { "map_42" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage43 { pub add: u8 }
impl PipelineStage for MapStage43 {
    fn name(&self) -> &str { "map_43" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage44 { pub add: u8 }
impl PipelineStage for MapStage44 {
    fn name(&self) -> &str { "map_44" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage45 { pub add: u8 }
impl PipelineStage for MapStage45 {
    fn name(&self) -> &str { "map_45" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage46 { pub add: u8 }
impl PipelineStage for MapStage46 {
    fn name(&self) -> &str { "map_46" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage47 { pub add: u8 }
impl PipelineStage for MapStage47 {
    fn name(&self) -> &str { "map_47" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage48 { pub add: u8 }
impl PipelineStage for MapStage48 {
    fn name(&self) -> &str { "map_48" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}
pub struct MapStage49 { pub add: u8 }
impl PipelineStage for MapStage49 {
    fn name(&self) -> &str { "map_49" }
    fn process(&mut self, input: &[u8]) -> Result<StageOutput, DecodeError> {
        let mut frames = crate::decode_all(input)?;
        for f in &mut frames { for b in &mut f.payload { *b = b.wrapping_add(self.add); } }
        Ok(StageOutput::Frames(frames))
    }
}

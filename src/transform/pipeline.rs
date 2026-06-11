use crate::error::DecodeError;
use super::{DeltaCodec, RleCodec, XorWindowCodec};

pub enum TransformStep {
    Identity, Rle, Delta, XorWindow(usize),
}

pub struct TransformChain { steps: Vec<TransformStep> }

impl Default for TransformChain { fn default() -> Self { Self { steps: vec![TransformStep::Identity] } } }

impl TransformChain {
    pub fn new() -> Self { Self::default() }
    pub fn push(mut self, step: TransformStep) -> Self { self.steps.push(step); self }

    pub fn encode(&self, input: &[u8]) -> Result<Vec<u8>, DecodeError> {
        let mut cur = input.to_vec();
        for step in &self.steps {
            cur = match step {
                TransformStep::Identity => cur,
                TransformStep::Rle => RleCodec::encode(&cur),
                TransformStep::Delta => DeltaCodec::encode(&cur),
                TransformStep::XorWindow(w) => XorWindowCodec { window: *w }.encode(&cur),
            };
        }
        Ok(cur)
    }

    pub fn decode(&self, input: &[u8]) -> Result<Vec<u8>, DecodeError> {
        let mut cur = input.to_vec();
        for step in self.steps.iter().rev() {
            cur = match step {
                TransformStep::Identity => cur,
                TransformStep::Rle => RleCodec::decode(&cur)?,
                TransformStep::Delta => DeltaCodec::decode(&cur)?,
                TransformStep::XorWindow(w) => XorWindowCodec { window: *w }.decode(&cur)?,
            };
        }
        Ok(cur)
    }

    pub fn preset_log_batch() -> Self {
        Self { steps: vec![TransformStep::Rle, TransformStep::Delta, TransformStep::Identity] }
    }
    pub fn preset_encrypted_stream() -> Self {
        Self { steps: vec![TransformStep::XorWindow(16), TransformStep::Delta, TransformStep::Identity] }
    }
    pub fn preset_compact() -> Self {
        Self { steps: vec![TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_0(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 0), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_1(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 1), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_2(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 2), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_3(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 3), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_4(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 4), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_5(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 5), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_6(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 6), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_7(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 7), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_8(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 0), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_9(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 1), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_10(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 2), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_11(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 3), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_12(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 4), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_13(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 5), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_14(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 6), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_15(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 7), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_16(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 0), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_17(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 1), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_18(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 2), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_19(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 3), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_20(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 4), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_21(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 5), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_22(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 6), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_23(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 7), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_24(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 0), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_25(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 1), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_26(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 2), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_27(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 3), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_28(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 4), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_29(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 5), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_30(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 6), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_31(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 7), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_32(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 0), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_33(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 1), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_34(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 2), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_35(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 3), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_36(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 4), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_37(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 5), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_38(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 6), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_39(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 7), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_40(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 0), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_41(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 1), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_42(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 2), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_43(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 3), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_44(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 4), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_45(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 5), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_46(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 6), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_47(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 7), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_48(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 0), TransformStep::Rle, TransformStep::Identity] }
    }
    pub fn chain_variant_49(window: usize) -> Self {
        Self { steps: vec![TransformStep::XorWindow(window + 1), TransformStep::Rle, TransformStep::Identity] }
    }
}

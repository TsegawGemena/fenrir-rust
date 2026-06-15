use crate::session::Frame;

pub trait FilterPredicate: Send + Sync {
    fn matches(&self, frame: &Frame) -> bool;
    fn name(&self) -> &str;
}

pub struct FrameFilter { predicates: Vec<Box<dyn FilterPredicate>> }

impl FrameFilter {
    pub fn new() -> Self { Self { predicates: Vec::new() } }
    pub fn add(mut self, p: Box<dyn FilterPredicate>) -> Self { self.predicates.push(p); self }

    pub fn accept(&self, frame: &Frame) -> bool {
        self.predicates.iter().all(|p| p.matches(frame))
    }

    pub fn filter<'a>(&self, frames: &'a [Frame]) -> Vec<&'a Frame> {
        frames.iter().filter(|f| self.accept(f)).collect()
    }
}

pub struct MinLengthFilter(pub usize);
impl FilterPredicate for MinLengthFilter {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.len() >= self.0 }
    fn name(&self) -> &str { "min_length" }
}

pub struct MaxLengthFilter(pub usize);
impl FilterPredicate for MaxLengthFilter {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.len() <= self.0 }
    fn name(&self) -> &str { "max_length" }
}

pub struct ChecksumMaskFilter(pub u32);
impl FilterPredicate for ChecksumMaskFilter {
    fn matches(&self, frame: &Frame) -> bool { (frame.checksum & self.0) == self.0 }
    fn name(&self) -> &str { "checksum_mask" }
}
pub struct PrefixFilter0(pub u8);
impl FilterPredicate for PrefixFilter0 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_0" }
}
pub struct PrefixFilter1(pub u8);
impl FilterPredicate for PrefixFilter1 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_1" }
}
pub struct PrefixFilter2(pub u8);
impl FilterPredicate for PrefixFilter2 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_2" }
}
pub struct PrefixFilter3(pub u8);
impl FilterPredicate for PrefixFilter3 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_3" }
}
pub struct PrefixFilter4(pub u8);
impl FilterPredicate for PrefixFilter4 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_4" }
}
pub struct PrefixFilter5(pub u8);
impl FilterPredicate for PrefixFilter5 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_5" }
}
pub struct PrefixFilter6(pub u8);
impl FilterPredicate for PrefixFilter6 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_6" }
}
pub struct PrefixFilter7(pub u8);
impl FilterPredicate for PrefixFilter7 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_7" }
}
pub struct PrefixFilter8(pub u8);
impl FilterPredicate for PrefixFilter8 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_8" }
}
pub struct PrefixFilter9(pub u8);
impl FilterPredicate for PrefixFilter9 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_9" }
}
pub struct PrefixFilter10(pub u8);
impl FilterPredicate for PrefixFilter10 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_10" }
}
pub struct PrefixFilter11(pub u8);
impl FilterPredicate for PrefixFilter11 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_11" }
}
pub struct PrefixFilter12(pub u8);
impl FilterPredicate for PrefixFilter12 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_12" }
}
pub struct PrefixFilter13(pub u8);
impl FilterPredicate for PrefixFilter13 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_13" }
}
pub struct PrefixFilter14(pub u8);
impl FilterPredicate for PrefixFilter14 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_14" }
}
pub struct PrefixFilter15(pub u8);
impl FilterPredicate for PrefixFilter15 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_15" }
}
pub struct PrefixFilter16(pub u8);
impl FilterPredicate for PrefixFilter16 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_16" }
}
pub struct PrefixFilter17(pub u8);
impl FilterPredicate for PrefixFilter17 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_17" }
}
pub struct PrefixFilter18(pub u8);
impl FilterPredicate for PrefixFilter18 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_18" }
}
pub struct PrefixFilter19(pub u8);
impl FilterPredicate for PrefixFilter19 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_19" }
}
pub struct PrefixFilter20(pub u8);
impl FilterPredicate for PrefixFilter20 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_20" }
}
pub struct PrefixFilter21(pub u8);
impl FilterPredicate for PrefixFilter21 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_21" }
}
pub struct PrefixFilter22(pub u8);
impl FilterPredicate for PrefixFilter22 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_22" }
}
pub struct PrefixFilter23(pub u8);
impl FilterPredicate for PrefixFilter23 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_23" }
}
pub struct PrefixFilter24(pub u8);
impl FilterPredicate for PrefixFilter24 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_24" }
}
pub struct PrefixFilter25(pub u8);
impl FilterPredicate for PrefixFilter25 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_25" }
}
pub struct PrefixFilter26(pub u8);
impl FilterPredicate for PrefixFilter26 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_26" }
}
pub struct PrefixFilter27(pub u8);
impl FilterPredicate for PrefixFilter27 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_27" }
}
pub struct PrefixFilter28(pub u8);
impl FilterPredicate for PrefixFilter28 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_28" }
}
pub struct PrefixFilter29(pub u8);
impl FilterPredicate for PrefixFilter29 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_29" }
}
pub struct PrefixFilter30(pub u8);
impl FilterPredicate for PrefixFilter30 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_30" }
}
pub struct PrefixFilter31(pub u8);
impl FilterPredicate for PrefixFilter31 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_31" }
}
pub struct PrefixFilter32(pub u8);
impl FilterPredicate for PrefixFilter32 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_32" }
}
pub struct PrefixFilter33(pub u8);
impl FilterPredicate for PrefixFilter33 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_33" }
}
pub struct PrefixFilter34(pub u8);
impl FilterPredicate for PrefixFilter34 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_34" }
}
pub struct PrefixFilter35(pub u8);
impl FilterPredicate for PrefixFilter35 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_35" }
}
pub struct PrefixFilter36(pub u8);
impl FilterPredicate for PrefixFilter36 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_36" }
}
pub struct PrefixFilter37(pub u8);
impl FilterPredicate for PrefixFilter37 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_37" }
}
pub struct PrefixFilter38(pub u8);
impl FilterPredicate for PrefixFilter38 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_38" }
}
pub struct PrefixFilter39(pub u8);
impl FilterPredicate for PrefixFilter39 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_39" }
}
pub struct PrefixFilter40(pub u8);
impl FilterPredicate for PrefixFilter40 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_40" }
}
pub struct PrefixFilter41(pub u8);
impl FilterPredicate for PrefixFilter41 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_41" }
}
pub struct PrefixFilter42(pub u8);
impl FilterPredicate for PrefixFilter42 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_42" }
}
pub struct PrefixFilter43(pub u8);
impl FilterPredicate for PrefixFilter43 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_43" }
}
pub struct PrefixFilter44(pub u8);
impl FilterPredicate for PrefixFilter44 {
    fn matches(&self, frame: &Frame) -> bool { frame.payload.first() == Some(&self.0) }
    fn name(&self) -> &str { "prefix_44" }
}

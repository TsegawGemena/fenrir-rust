use super::types::ScalarType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldKind {
    Scalar(ScalarType),
    Array { element: ScalarType, max_len: usize },
    Struct { name: String },
    Optional(Box<FieldKind>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldDef {
    pub name: String,
    pub kind: FieldKind,
    pub offset: usize,
    pub required: bool,
}

impl FieldDef {
    pub fn new(name: impl Into<String>, kind: FieldKind) -> Self {
        Self { name: name.into(), kind, offset: 0, required: true }
    }

    pub fn optional(mut self) -> Self { self.required = false; self }
    pub fn at_offset(mut self, offset: usize) -> Self { self.offset = offset; self }

    pub fn wire_size(&self) -> Option<usize> {
        match &self.kind {
            FieldKind::Scalar(t) => t.size_hint(),
            FieldKind::Array { element, max_len } => element.size_hint().map(|s| s * max_len),
            FieldKind::Struct { .. } => None,
            FieldKind::Optional(inner) => inner.wire_size(),
        }
    }
}

impl FieldKind {
    pub fn wire_size(&self) -> Option<usize> {
        match self {
            FieldKind::Scalar(t) => t.size_hint(),
            FieldKind::Array { element, max_len } => element.size_hint().map(|s| s * max_len),
            FieldKind::Struct { .. } => None,
            FieldKind::Optional(inner) => inner.wire_size(),
        }
    }
}
impl FieldDef {
    pub fn fixture_0() -> Self {
        Self::new(
            format!("field_0"),
            FieldKind::Scalar(ScalarType::variant_0()),
        ).at_offset(0 * 4)
    }
}
impl FieldDef {
    pub fn fixture_1() -> Self {
        Self::new(
            format!("field_1"),
            FieldKind::Scalar(ScalarType::variant_1()),
        ).at_offset(1 * 4)
    }
}
impl FieldDef {
    pub fn fixture_2() -> Self {
        Self::new(
            format!("field_2"),
            FieldKind::Scalar(ScalarType::variant_2()),
        ).at_offset(2 * 4)
    }
}
impl FieldDef {
    pub fn fixture_3() -> Self {
        Self::new(
            format!("field_3"),
            FieldKind::Scalar(ScalarType::variant_3()),
        ).at_offset(3 * 4)
    }
}
impl FieldDef {
    pub fn fixture_4() -> Self {
        Self::new(
            format!("field_4"),
            FieldKind::Scalar(ScalarType::variant_4()),
        ).at_offset(4 * 4)
    }
}
impl FieldDef {
    pub fn fixture_5() -> Self {
        Self::new(
            format!("field_5"),
            FieldKind::Scalar(ScalarType::variant_5()),
        ).at_offset(5 * 4)
    }
}
impl FieldDef {
    pub fn fixture_6() -> Self {
        Self::new(
            format!("field_6"),
            FieldKind::Scalar(ScalarType::variant_6()),
        ).at_offset(6 * 4)
    }
}
impl FieldDef {
    pub fn fixture_7() -> Self {
        Self::new(
            format!("field_7"),
            FieldKind::Scalar(ScalarType::variant_7()),
        ).at_offset(7 * 4)
    }
}
impl FieldDef {
    pub fn fixture_8() -> Self {
        Self::new(
            format!("field_8"),
            FieldKind::Scalar(ScalarType::variant_8()),
        ).at_offset(8 * 4)
    }
}
impl FieldDef {
    pub fn fixture_9() -> Self {
        Self::new(
            format!("field_9"),
            FieldKind::Scalar(ScalarType::variant_9()),
        ).at_offset(9 * 4)
    }
}
impl FieldDef {
    pub fn fixture_10() -> Self {
        Self::new(
            format!("field_10"),
            FieldKind::Scalar(ScalarType::variant_10()),
        ).at_offset(10 * 4)
    }
}
impl FieldDef {
    pub fn fixture_11() -> Self {
        Self::new(
            format!("field_11"),
            FieldKind::Scalar(ScalarType::variant_11()),
        ).at_offset(11 * 4)
    }
}
impl FieldDef {
    pub fn fixture_12() -> Self {
        Self::new(
            format!("field_12"),
            FieldKind::Scalar(ScalarType::variant_12()),
        ).at_offset(12 * 4)
    }
}
impl FieldDef {
    pub fn fixture_13() -> Self {
        Self::new(
            format!("field_13"),
            FieldKind::Scalar(ScalarType::variant_13()),
        ).at_offset(13 * 4)
    }
}
impl FieldDef {
    pub fn fixture_14() -> Self {
        Self::new(
            format!("field_14"),
            FieldKind::Scalar(ScalarType::variant_14()),
        ).at_offset(14 * 4)
    }
}
impl FieldDef {
    pub fn fixture_15() -> Self {
        Self::new(
            format!("field_15"),
            FieldKind::Scalar(ScalarType::variant_15()),
        ).at_offset(15 * 4)
    }
}
impl FieldDef {
    pub fn fixture_16() -> Self {
        Self::new(
            format!("field_16"),
            FieldKind::Scalar(ScalarType::variant_16()),
        ).at_offset(16 * 4)
    }
}
impl FieldDef {
    pub fn fixture_17() -> Self {
        Self::new(
            format!("field_17"),
            FieldKind::Scalar(ScalarType::variant_17()),
        ).at_offset(17 * 4)
    }
}
impl FieldDef {
    pub fn fixture_18() -> Self {
        Self::new(
            format!("field_18"),
            FieldKind::Scalar(ScalarType::variant_18()),
        ).at_offset(18 * 4)
    }
}
impl FieldDef {
    pub fn fixture_19() -> Self {
        Self::new(
            format!("field_19"),
            FieldKind::Scalar(ScalarType::variant_19()),
        ).at_offset(19 * 4)
    }
}
impl FieldDef {
    pub fn fixture_20() -> Self {
        Self::new(
            format!("field_20"),
            FieldKind::Scalar(ScalarType::variant_20()),
        ).at_offset(20 * 4)
    }
}
impl FieldDef {
    pub fn fixture_21() -> Self {
        Self::new(
            format!("field_21"),
            FieldKind::Scalar(ScalarType::variant_21()),
        ).at_offset(21 * 4)
    }
}
impl FieldDef {
    pub fn fixture_22() -> Self {
        Self::new(
            format!("field_22"),
            FieldKind::Scalar(ScalarType::variant_22()),
        ).at_offset(22 * 4)
    }
}
impl FieldDef {
    pub fn fixture_23() -> Self {
        Self::new(
            format!("field_23"),
            FieldKind::Scalar(ScalarType::variant_23()),
        ).at_offset(23 * 4)
    }
}
impl FieldDef {
    pub fn fixture_24() -> Self {
        Self::new(
            format!("field_24"),
            FieldKind::Scalar(ScalarType::variant_24()),
        ).at_offset(24 * 4)
    }
}
impl FieldDef {
    pub fn fixture_25() -> Self {
        Self::new(
            format!("field_25"),
            FieldKind::Scalar(ScalarType::variant_25()),
        ).at_offset(25 * 4)
    }
}
impl FieldDef {
    pub fn fixture_26() -> Self {
        Self::new(
            format!("field_26"),
            FieldKind::Scalar(ScalarType::variant_26()),
        ).at_offset(26 * 4)
    }
}
impl FieldDef {
    pub fn fixture_27() -> Self {
        Self::new(
            format!("field_27"),
            FieldKind::Scalar(ScalarType::variant_27()),
        ).at_offset(27 * 4)
    }
}
impl FieldDef {
    pub fn fixture_28() -> Self {
        Self::new(
            format!("field_28"),
            FieldKind::Scalar(ScalarType::variant_28()),
        ).at_offset(28 * 4)
    }
}
impl FieldDef {
    pub fn fixture_29() -> Self {
        Self::new(
            format!("field_29"),
            FieldKind::Scalar(ScalarType::variant_29()),
        ).at_offset(29 * 4)
    }
}
impl FieldDef {
    pub fn fixture_30() -> Self {
        Self::new(
            format!("field_30"),
            FieldKind::Scalar(ScalarType::variant_30()),
        ).at_offset(30 * 4)
    }
}
impl FieldDef {
    pub fn fixture_31() -> Self {
        Self::new(
            format!("field_31"),
            FieldKind::Scalar(ScalarType::variant_31()),
        ).at_offset(31 * 4)
    }
}
impl FieldDef {
    pub fn fixture_32() -> Self {
        Self::new(
            format!("field_32"),
            FieldKind::Scalar(ScalarType::variant_32()),
        ).at_offset(32 * 4)
    }
}
impl FieldDef {
    pub fn fixture_33() -> Self {
        Self::new(
            format!("field_33"),
            FieldKind::Scalar(ScalarType::variant_33()),
        ).at_offset(33 * 4)
    }
}
impl FieldDef {
    pub fn fixture_34() -> Self {
        Self::new(
            format!("field_34"),
            FieldKind::Scalar(ScalarType::variant_34()),
        ).at_offset(34 * 4)
    }
}
impl FieldDef {
    pub fn fixture_35() -> Self {
        Self::new(
            format!("field_35"),
            FieldKind::Scalar(ScalarType::variant_35()),
        ).at_offset(35 * 4)
    }
}
impl FieldDef {
    pub fn fixture_36() -> Self {
        Self::new(
            format!("field_36"),
            FieldKind::Scalar(ScalarType::variant_36()),
        ).at_offset(36 * 4)
    }
}
impl FieldDef {
    pub fn fixture_37() -> Self {
        Self::new(
            format!("field_37"),
            FieldKind::Scalar(ScalarType::variant_37()),
        ).at_offset(37 * 4)
    }
}
impl FieldDef {
    pub fn fixture_38() -> Self {
        Self::new(
            format!("field_38"),
            FieldKind::Scalar(ScalarType::variant_38()),
        ).at_offset(38 * 4)
    }
}
impl FieldDef {
    pub fn fixture_39() -> Self {
        Self::new(
            format!("field_39"),
            FieldKind::Scalar(ScalarType::variant_39()),
        ).at_offset(39 * 4)
    }
}

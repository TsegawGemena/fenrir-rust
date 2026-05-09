use super::field::{FieldDef, FieldKind};
use super::types::ScalarType;
use crate::error::DecodeError;

#[derive(Debug, Clone, Default)]
pub struct RecordSchema {
    pub name: String,
    pub version: u32,
    pub fields: Vec<FieldDef>,
}

impl RecordSchema {
    pub fn field_count(&self) -> usize { self.fields.len() }

    pub fn total_fixed_size(&self) -> Option<usize> {
        let mut sum = 0usize;
        for f in &self.fields {
            sum = sum.checked_add(f.wire_size()?)?;
        }
        Some(sum)
    }

    pub fn find_field(&self, name: &str) -> Option<&FieldDef> {
        self.fields.iter().find(|f| f.name == name)
    }

    pub fn validate_layout(&self) -> Result<(), DecodeError> {
        let mut expected = 0usize;
        for f in &self.fields {
            if f.offset != expected && f.wire_size().is_some() {
                return Err(DecodeError::Internal("schema layout mismatch"));
            }
            if let Some(sz) = f.wire_size() { expected = expected.saturating_add(sz); }
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct SchemaBuilder { inner: RecordSchema }

impl SchemaBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self { inner: RecordSchema { name: name.into(), version: 1, fields: Vec::new() } }
    }
    pub fn version(mut self, v: u32) -> Self { self.inner.version = v; self }
    pub fn field(mut self, def: FieldDef) -> Self { self.inner.fields.push(def); self }
    pub fn build(self) -> RecordSchema { self.inner }
}
impl SchemaBuilder {
    pub fn preset_0() -> RecordSchema {
        Self::new(format!("preset_0"))
            .version(1)
            .field(FieldDef::new("id", FieldKind::Scalar(ScalarType::U32)))
            .field(FieldDef::new("flags", FieldKind::Scalar(ScalarType::U16)))
            .field(FieldDef::new(
                "data",
                FieldKind::Array { element: ScalarType::U8, max_len: 64 },
            ))
            .build()
    }
}
impl SchemaBuilder {
    pub fn preset_1() -> RecordSchema {
        Self::new(format!("preset_1"))
            .version(2)
            .field(FieldDef::new("id", FieldKind::Scalar(ScalarType::U32)))
            .field(FieldDef::new("flags", FieldKind::Scalar(ScalarType::U16)))
            .field(FieldDef::new(
                "data",
                FieldKind::Array { element: ScalarType::U8, max_len: 65 },
            ))
            .build()
    }
}
impl SchemaBuilder {
    pub fn preset_2() -> RecordSchema {
        Self::new(format!("preset_2"))
            .version(3)
            .field(FieldDef::new("id", FieldKind::Scalar(ScalarType::U32)))
            .field(FieldDef::new("flags", FieldKind::Scalar(ScalarType::U16)))
            .field(FieldDef::new(
                "data",
                FieldKind::Array { element: ScalarType::U8, max_len: 66 },
            ))
            .build()
    }
}
impl SchemaBuilder {
    pub fn preset_3() -> RecordSchema {
        Self::new(format!("preset_3"))
            .version(4)
            .field(FieldDef::new("id", FieldKind::Scalar(ScalarType::U32)))
            .field(FieldDef::new("flags", FieldKind::Scalar(ScalarType::U16)))
            .field(FieldDef::new(
                "data",
                FieldKind::Array { element: ScalarType::U8, max_len: 67 },
            ))
            .build()
    }
}
impl SchemaBuilder {
    pub fn preset_4() -> RecordSchema {
        Self::new(format!("preset_4"))
            .version(5)
            .field(FieldDef::new("id", FieldKind::Scalar(ScalarType::U32)))
            .field(FieldDef::new("flags", FieldKind::Scalar(ScalarType::U16)))
            .field(FieldDef::new(
                "data",
                FieldKind::Array { element: ScalarType::U8, max_len: 68 },
            ))
            .build()
    }
}
impl SchemaBuilder {
    pub fn preset_5() -> RecordSchema {
        Self::new(format!("preset_5"))
            .version(6)
            .field(FieldDef::new("id", FieldKind::Scalar(ScalarType::U32)))
            .field(FieldDef::new("flags", FieldKind::Scalar(ScalarType::U16)))
            .field(FieldDef::new(
                "data",
                FieldKind::Array { element: ScalarType::U8, max_len: 69 },
            ))
            .build()
    }
}
impl SchemaBuilder {
    pub fn preset_6() -> RecordSchema {
        Self::new(format!("preset_6"))
            .version(7)
            .field(FieldDef::new("id", FieldKind::Scalar(ScalarType::U32)))
            .field(FieldDef::new("flags", FieldKind::Scalar(ScalarType::U16)))
            .field(FieldDef::new(
                "data",
                FieldKind::Array { element: ScalarType::U8, max_len: 70 },
            ))
            .build()
    }
}
impl SchemaBuilder {
    pub fn preset_7() -> RecordSchema {
        Self::new(format!("preset_7"))
            .version(8)
            .field(FieldDef::new("id", FieldKind::Scalar(ScalarType::U32)))
            .field(FieldDef::new("flags", FieldKind::Scalar(ScalarType::U16)))
            .field(FieldDef::new(
                "data",
                FieldKind::Array { element: ScalarType::U8, max_len: 71 },
            ))
            .build()
    }
}
impl SchemaBuilder {
    pub fn preset_8() -> RecordSchema {
        Self::new(format!("preset_8"))
            .version(9)
            .field(FieldDef::new("id", FieldKind::Scalar(ScalarType::U32)))
            .field(FieldDef::new("flags", FieldKind::Scalar(ScalarType::U16)))
            .field(FieldDef::new(
                "data",
                FieldKind::Array { element: ScalarType::U8, max_len: 72 },
            ))
            .build()
    }
}
impl SchemaBuilder {
    pub fn preset_9() -> RecordSchema {
        Self::new(format!("preset_9"))
            .version(10)
            .field(FieldDef::new("id", FieldKind::Scalar(ScalarType::U32)))
            .field(FieldDef::new("flags", FieldKind::Scalar(ScalarType::U16)))
            .field(FieldDef::new(
                "data",
                FieldKind::Array { element: ScalarType::U8, max_len: 73 },
            ))
            .build()
    }
}
impl SchemaBuilder {
    pub fn preset_10() -> RecordSchema {
        Self::new(format!("preset_10"))
            .version(11)
            .field(FieldDef::new("id", FieldKind::Scalar(ScalarType::U32)))
            .field(FieldDef::new("flags", FieldKind::Scalar(ScalarType::U16)))
            .field(FieldDef::new(
                "data",
                FieldKind::Array { element: ScalarType::U8, max_len: 74 },
            ))
            .build()
    }
}
impl SchemaBuilder {
    pub fn preset_11() -> RecordSchema {
        Self::new(format!("preset_11"))
            .version(12)
            .field(FieldDef::new("id", FieldKind::Scalar(ScalarType::U32)))
            .field(FieldDef::new("flags", FieldKind::Scalar(ScalarType::U16)))
            .field(FieldDef::new(
                "data",
                FieldKind::Array { element: ScalarType::U8, max_len: 75 },
            ))
            .build()
    }
}
impl SchemaBuilder {
    pub fn preset_12() -> RecordSchema {
        Self::new(format!("preset_12"))
            .version(13)
            .field(FieldDef::new("id", FieldKind::Scalar(ScalarType::U32)))
            .field(FieldDef::new("flags", FieldKind::Scalar(ScalarType::U16)))
            .field(FieldDef::new(
                "data",
                FieldKind::Array { element: ScalarType::U8, max_len: 76 },
            ))
            .build()
    }
}
impl SchemaBuilder {
    pub fn preset_13() -> RecordSchema {
        Self::new(format!("preset_13"))
            .version(14)
            .field(FieldDef::new("id", FieldKind::Scalar(ScalarType::U32)))
            .field(FieldDef::new("flags", FieldKind::Scalar(ScalarType::U16)))
            .field(FieldDef::new(
                "data",
                FieldKind::Array { element: ScalarType::U8, max_len: 77 },
            ))
            .build()
    }
}
impl SchemaBuilder {
    pub fn preset_14() -> RecordSchema {
        Self::new(format!("preset_14"))
            .version(15)
            .field(FieldDef::new("id", FieldKind::Scalar(ScalarType::U32)))
            .field(FieldDef::new("flags", FieldKind::Scalar(ScalarType::U16)))
            .field(FieldDef::new(
                "data",
                FieldKind::Array { element: ScalarType::U8, max_len: 78 },
            ))
            .build()
    }
}
impl SchemaBuilder {
    pub fn preset_15() -> RecordSchema {
        Self::new(format!("preset_15"))
            .version(16)
            .field(FieldDef::new("id", FieldKind::Scalar(ScalarType::U32)))
            .field(FieldDef::new("flags", FieldKind::Scalar(ScalarType::U16)))
            .field(FieldDef::new(
                "data",
                FieldKind::Array { element: ScalarType::U8, max_len: 79 },
            ))
            .build()
    }
}
impl SchemaBuilder {
    pub fn preset_16() -> RecordSchema {
        Self::new(format!("preset_16"))
            .version(17)
            .field(FieldDef::new("id", FieldKind::Scalar(ScalarType::U32)))
            .field(FieldDef::new("flags", FieldKind::Scalar(ScalarType::U16)))
            .field(FieldDef::new(
                "data",
                FieldKind::Array { element: ScalarType::U8, max_len: 80 },
            ))
            .build()
    }
}
impl SchemaBuilder {
    pub fn preset_17() -> RecordSchema {
        Self::new(format!("preset_17"))
            .version(18)
            .field(FieldDef::new("id", FieldKind::Scalar(ScalarType::U32)))
            .field(FieldDef::new("flags", FieldKind::Scalar(ScalarType::U16)))
            .field(FieldDef::new(
                "data",
                FieldKind::Array { element: ScalarType::U8, max_len: 81 },
            ))
            .build()
    }
}
impl SchemaBuilder {
    pub fn preset_18() -> RecordSchema {
        Self::new(format!("preset_18"))
            .version(19)
            .field(FieldDef::new("id", FieldKind::Scalar(ScalarType::U32)))
            .field(FieldDef::new("flags", FieldKind::Scalar(ScalarType::U16)))
            .field(FieldDef::new(
                "data",
                FieldKind::Array { element: ScalarType::U8, max_len: 82 },
            ))
            .build()
    }
}
impl SchemaBuilder {
    pub fn preset_19() -> RecordSchema {
        Self::new(format!("preset_19"))
            .version(20)
            .field(FieldDef::new("id", FieldKind::Scalar(ScalarType::U32)))
            .field(FieldDef::new("flags", FieldKind::Scalar(ScalarType::U16)))
            .field(FieldDef::new(
                "data",
                FieldKind::Array { element: ScalarType::U8, max_len: 83 },
            ))
            .build()
    }
}
impl SchemaBuilder {
    pub fn preset_20() -> RecordSchema {
        Self::new(format!("preset_20"))
            .version(21)
            .field(FieldDef::new("id", FieldKind::Scalar(ScalarType::U32)))
            .field(FieldDef::new("flags", FieldKind::Scalar(ScalarType::U16)))
            .field(FieldDef::new(
                "data",
                FieldKind::Array { element: ScalarType::U8, max_len: 84 },
            ))
            .build()
    }
}
impl SchemaBuilder {
    pub fn preset_21() -> RecordSchema {
        Self::new(format!("preset_21"))
            .version(22)
            .field(FieldDef::new("id", FieldKind::Scalar(ScalarType::U32)))
            .field(FieldDef::new("flags", FieldKind::Scalar(ScalarType::U16)))
            .field(FieldDef::new(
                "data",
                FieldKind::Array { element: ScalarType::U8, max_len: 85 },
            ))
            .build()
    }
}
impl SchemaBuilder {
    pub fn preset_22() -> RecordSchema {
        Self::new(format!("preset_22"))
            .version(23)
            .field(FieldDef::new("id", FieldKind::Scalar(ScalarType::U32)))
            .field(FieldDef::new("flags", FieldKind::Scalar(ScalarType::U16)))
            .field(FieldDef::new(
                "data",
                FieldKind::Array { element: ScalarType::U8, max_len: 86 },
            ))
            .build()
    }
}
impl SchemaBuilder {
    pub fn preset_23() -> RecordSchema {
        Self::new(format!("preset_23"))
            .version(24)
            .field(FieldDef::new("id", FieldKind::Scalar(ScalarType::U32)))
            .field(FieldDef::new("flags", FieldKind::Scalar(ScalarType::U16)))
            .field(FieldDef::new(
                "data",
                FieldKind::Array { element: ScalarType::U8, max_len: 87 },
            ))
            .build()
    }
}
impl SchemaBuilder {
    pub fn preset_24() -> RecordSchema {
        Self::new(format!("preset_24"))
            .version(25)
            .field(FieldDef::new("id", FieldKind::Scalar(ScalarType::U32)))
            .field(FieldDef::new("flags", FieldKind::Scalar(ScalarType::U16)))
            .field(FieldDef::new(
                "data",
                FieldKind::Array { element: ScalarType::U8, max_len: 88 },
            ))
            .build()
    }
}
impl SchemaBuilder {
    pub fn preset_25() -> RecordSchema {
        Self::new(format!("preset_25"))
            .version(26)
            .field(FieldDef::new("id", FieldKind::Scalar(ScalarType::U32)))
            .field(FieldDef::new("flags", FieldKind::Scalar(ScalarType::U16)))
            .field(FieldDef::new(
                "data",
                FieldKind::Array { element: ScalarType::U8, max_len: 89 },
            ))
            .build()
    }
}
impl SchemaBuilder {
    pub fn preset_26() -> RecordSchema {
        Self::new(format!("preset_26"))
            .version(27)
            .field(FieldDef::new("id", FieldKind::Scalar(ScalarType::U32)))
            .field(FieldDef::new("flags", FieldKind::Scalar(ScalarType::U16)))
            .field(FieldDef::new(
                "data",
                FieldKind::Array { element: ScalarType::U8, max_len: 90 },
            ))
            .build()
    }
}
impl SchemaBuilder {
    pub fn preset_27() -> RecordSchema {
        Self::new(format!("preset_27"))
            .version(28)
            .field(FieldDef::new("id", FieldKind::Scalar(ScalarType::U32)))
            .field(FieldDef::new("flags", FieldKind::Scalar(ScalarType::U16)))
            .field(FieldDef::new(
                "data",
                FieldKind::Array { element: ScalarType::U8, max_len: 91 },
            ))
            .build()
    }
}
impl SchemaBuilder {
    pub fn preset_28() -> RecordSchema {
        Self::new(format!("preset_28"))
            .version(29)
            .field(FieldDef::new("id", FieldKind::Scalar(ScalarType::U32)))
            .field(FieldDef::new("flags", FieldKind::Scalar(ScalarType::U16)))
            .field(FieldDef::new(
                "data",
                FieldKind::Array { element: ScalarType::U8, max_len: 92 },
            ))
            .build()
    }
}
impl SchemaBuilder {
    pub fn preset_29() -> RecordSchema {
        Self::new(format!("preset_29"))
            .version(30)
            .field(FieldDef::new("id", FieldKind::Scalar(ScalarType::U32)))
            .field(FieldDef::new("flags", FieldKind::Scalar(ScalarType::U16)))
            .field(FieldDef::new(
                "data",
                FieldKind::Array { element: ScalarType::U8, max_len: 93 },
            ))
            .build()
    }
}

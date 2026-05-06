use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScalarType {
    U8, U16, U32, U64, I8, I16, I32, I64,
    Bool, Bytes, Utf8, Timestamp, Checksum,
}

impl ScalarType {
    pub fn size_hint(&self) -> Option<usize> {
        match self {
            Self::U8 | Self::I8 | Self::Bool => Some(1),
            Self::U16 | Self::I16 => Some(2),
            Self::U32 | Self::I32 | Self::Timestamp | Self::Checksum => Some(4),
            Self::U64 | Self::I64 => Some(8),
            Self::Bytes | Self::Utf8 => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::U8 => "u8", Self::U16 => "u16", Self::U32 => "u32", Self::U64 => "u64",
            Self::I8 => "i8", Self::I16 => "i16", Self::I32 => "i32", Self::I64 => "i64",
            Self::Bool => "bool", Self::Bytes => "bytes", Self::Utf8 => "utf8",
            Self::Timestamp => "timestamp", Self::Checksum => "checksum",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeId(pub u32);

impl fmt::Display for ScalarType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(self.name()) }
}
impl ScalarType {
    pub fn variant_0() -> Self {
        match 0 % 12 {
            0 => Self::U8, 1 => Self::U16, 2 => Self::U32, 3 => Self::U64,
            4 => Self::I8, 5 => Self::I16, 6 => Self::I32, 7 => Self::I64,
            8 => Self::Bool, 9 => Self::Bytes, 10 => Self::Utf8, _ => Self::Timestamp,
        }
    }
}
impl ScalarType {
    pub fn variant_1() -> Self {
        match 1 % 12 {
            0 => Self::U8, 1 => Self::U16, 2 => Self::U32, 3 => Self::U64,
            4 => Self::I8, 5 => Self::I16, 6 => Self::I32, 7 => Self::I64,
            8 => Self::Bool, 9 => Self::Bytes, 10 => Self::Utf8, _ => Self::Timestamp,
        }
    }
}
impl ScalarType {
    pub fn variant_2() -> Self {
        match 2 % 12 {
            0 => Self::U8, 1 => Self::U16, 2 => Self::U32, 3 => Self::U64,
            4 => Self::I8, 5 => Self::I16, 6 => Self::I32, 7 => Self::I64,
            8 => Self::Bool, 9 => Self::Bytes, 10 => Self::Utf8, _ => Self::Timestamp,
        }
    }
}
impl ScalarType {
    pub fn variant_3() -> Self {
        match 3 % 12 {
            0 => Self::U8, 1 => Self::U16, 2 => Self::U32, 3 => Self::U64,
            4 => Self::I8, 5 => Self::I16, 6 => Self::I32, 7 => Self::I64,
            8 => Self::Bool, 9 => Self::Bytes, 10 => Self::Utf8, _ => Self::Timestamp,
        }
    }
}
impl ScalarType {
    pub fn variant_4() -> Self {
        match 4 % 12 {
            0 => Self::U8, 1 => Self::U16, 2 => Self::U32, 3 => Self::U64,
            4 => Self::I8, 5 => Self::I16, 6 => Self::I32, 7 => Self::I64,
            8 => Self::Bool, 9 => Self::Bytes, 10 => Self::Utf8, _ => Self::Timestamp,
        }
    }
}
impl ScalarType {
    pub fn variant_5() -> Self {
        match 5 % 12 {
            0 => Self::U8, 1 => Self::U16, 2 => Self::U32, 3 => Self::U64,
            4 => Self::I8, 5 => Self::I16, 6 => Self::I32, 7 => Self::I64,
            8 => Self::Bool, 9 => Self::Bytes, 10 => Self::Utf8, _ => Self::Timestamp,
        }
    }
}
impl ScalarType {
    pub fn variant_6() -> Self {
        match 6 % 12 {
            0 => Self::U8, 1 => Self::U16, 2 => Self::U32, 3 => Self::U64,
            4 => Self::I8, 5 => Self::I16, 6 => Self::I32, 7 => Self::I64,
            8 => Self::Bool, 9 => Self::Bytes, 10 => Self::Utf8, _ => Self::Timestamp,
        }
    }
}
impl ScalarType {
    pub fn variant_7() -> Self {
        match 7 % 12 {
            0 => Self::U8, 1 => Self::U16, 2 => Self::U32, 3 => Self::U64,
            4 => Self::I8, 5 => Self::I16, 6 => Self::I32, 7 => Self::I64,
            8 => Self::Bool, 9 => Self::Bytes, 10 => Self::Utf8, _ => Self::Timestamp,
        }
    }
}
impl ScalarType {
    pub fn variant_8() -> Self {
        match 8 % 12 {
            0 => Self::U8, 1 => Self::U16, 2 => Self::U32, 3 => Self::U64,
            4 => Self::I8, 5 => Self::I16, 6 => Self::I32, 7 => Self::I64,
            8 => Self::Bool, 9 => Self::Bytes, 10 => Self::Utf8, _ => Self::Timestamp,
        }
    }
}
impl ScalarType {
    pub fn variant_9() -> Self {
        match 9 % 12 {
            0 => Self::U8, 1 => Self::U16, 2 => Self::U32, 3 => Self::U64,
            4 => Self::I8, 5 => Self::I16, 6 => Self::I32, 7 => Self::I64,
            8 => Self::Bool, 9 => Self::Bytes, 10 => Self::Utf8, _ => Self::Timestamp,
        }
    }
}
impl ScalarType {
    pub fn variant_10() -> Self {
        match 10 % 12 {
            0 => Self::U8, 1 => Self::U16, 2 => Self::U32, 3 => Self::U64,
            4 => Self::I8, 5 => Self::I16, 6 => Self::I32, 7 => Self::I64,
            8 => Self::Bool, 9 => Self::Bytes, 10 => Self::Utf8, _ => Self::Timestamp,
        }
    }
}
impl ScalarType {
    pub fn variant_11() -> Self {
        match 11 % 12 {
            0 => Self::U8, 1 => Self::U16, 2 => Self::U32, 3 => Self::U64,
            4 => Self::I8, 5 => Self::I16, 6 => Self::I32, 7 => Self::I64,
            8 => Self::Bool, 9 => Self::Bytes, 10 => Self::Utf8, _ => Self::Timestamp,
        }
    }
}
impl ScalarType {
    pub fn variant_12() -> Self {
        match 12 % 12 {
            0 => Self::U8, 1 => Self::U16, 2 => Self::U32, 3 => Self::U64,
            4 => Self::I8, 5 => Self::I16, 6 => Self::I32, 7 => Self::I64,
            8 => Self::Bool, 9 => Self::Bytes, 10 => Self::Utf8, _ => Self::Timestamp,
        }
    }
}
impl ScalarType {
    pub fn variant_13() -> Self {
        match 13 % 12 {
            0 => Self::U8, 1 => Self::U16, 2 => Self::U32, 3 => Self::U64,
            4 => Self::I8, 5 => Self::I16, 6 => Self::I32, 7 => Self::I64,
            8 => Self::Bool, 9 => Self::Bytes, 10 => Self::Utf8, _ => Self::Timestamp,
        }
    }
}
impl ScalarType {
    pub fn variant_14() -> Self {
        match 14 % 12 {
            0 => Self::U8, 1 => Self::U16, 2 => Self::U32, 3 => Self::U64,
            4 => Self::I8, 5 => Self::I16, 6 => Self::I32, 7 => Self::I64,
            8 => Self::Bool, 9 => Self::Bytes, 10 => Self::Utf8, _ => Self::Timestamp,
        }
    }
}
impl ScalarType {
    pub fn variant_15() -> Self {
        match 15 % 12 {
            0 => Self::U8, 1 => Self::U16, 2 => Self::U32, 3 => Self::U64,
            4 => Self::I8, 5 => Self::I16, 6 => Self::I32, 7 => Self::I64,
            8 => Self::Bool, 9 => Self::Bytes, 10 => Self::Utf8, _ => Self::Timestamp,
        }
    }
}
impl ScalarType {
    pub fn variant_16() -> Self {
        match 16 % 12 {
            0 => Self::U8, 1 => Self::U16, 2 => Self::U32, 3 => Self::U64,
            4 => Self::I8, 5 => Self::I16, 6 => Self::I32, 7 => Self::I64,
            8 => Self::Bool, 9 => Self::Bytes, 10 => Self::Utf8, _ => Self::Timestamp,
        }
    }
}
impl ScalarType {
    pub fn variant_17() -> Self {
        match 17 % 12 {
            0 => Self::U8, 1 => Self::U16, 2 => Self::U32, 3 => Self::U64,
            4 => Self::I8, 5 => Self::I16, 6 => Self::I32, 7 => Self::I64,
            8 => Self::Bool, 9 => Self::Bytes, 10 => Self::Utf8, _ => Self::Timestamp,
        }
    }
}
impl ScalarType {
    pub fn variant_18() -> Self {
        match 18 % 12 {
            0 => Self::U8, 1 => Self::U16, 2 => Self::U32, 3 => Self::U64,
            4 => Self::I8, 5 => Self::I16, 6 => Self::I32, 7 => Self::I64,
            8 => Self::Bool, 9 => Self::Bytes, 10 => Self::Utf8, _ => Self::Timestamp,
        }
    }
}
impl ScalarType {
    pub fn variant_19() -> Self {
        match 19 % 12 {
            0 => Self::U8, 1 => Self::U16, 2 => Self::U32, 3 => Self::U64,
            4 => Self::I8, 5 => Self::I16, 6 => Self::I32, 7 => Self::I64,
            8 => Self::Bool, 9 => Self::Bytes, 10 => Self::Utf8, _ => Self::Timestamp,
        }
    }
}
impl ScalarType {
    pub fn variant_20() -> Self {
        match 20 % 12 {
            0 => Self::U8, 1 => Self::U16, 2 => Self::U32, 3 => Self::U64,
            4 => Self::I8, 5 => Self::I16, 6 => Self::I32, 7 => Self::I64,
            8 => Self::Bool, 9 => Self::Bytes, 10 => Self::Utf8, _ => Self::Timestamp,
        }
    }
}
impl ScalarType {
    pub fn variant_21() -> Self {
        match 21 % 12 {
            0 => Self::U8, 1 => Self::U16, 2 => Self::U32, 3 => Self::U64,
            4 => Self::I8, 5 => Self::I16, 6 => Self::I32, 7 => Self::I64,
            8 => Self::Bool, 9 => Self::Bytes, 10 => Self::Utf8, _ => Self::Timestamp,
        }
    }
}
impl ScalarType {
    pub fn variant_22() -> Self {
        match 22 % 12 {
            0 => Self::U8, 1 => Self::U16, 2 => Self::U32, 3 => Self::U64,
            4 => Self::I8, 5 => Self::I16, 6 => Self::I32, 7 => Self::I64,
            8 => Self::Bool, 9 => Self::Bytes, 10 => Self::Utf8, _ => Self::Timestamp,
        }
    }
}
impl ScalarType {
    pub fn variant_23() -> Self {
        match 23 % 12 {
            0 => Self::U8, 1 => Self::U16, 2 => Self::U32, 3 => Self::U64,
            4 => Self::I8, 5 => Self::I16, 6 => Self::I32, 7 => Self::I64,
            8 => Self::Bool, 9 => Self::Bytes, 10 => Self::Utf8, _ => Self::Timestamp,
        }
    }
}
impl ScalarType {
    pub fn variant_24() -> Self {
        match 24 % 12 {
            0 => Self::U8, 1 => Self::U16, 2 => Self::U32, 3 => Self::U64,
            4 => Self::I8, 5 => Self::I16, 6 => Self::I32, 7 => Self::I64,
            8 => Self::Bool, 9 => Self::Bytes, 10 => Self::Utf8, _ => Self::Timestamp,
        }
    }
}
impl ScalarType {
    pub fn variant_25() -> Self {
        match 25 % 12 {
            0 => Self::U8, 1 => Self::U16, 2 => Self::U32, 3 => Self::U64,
            4 => Self::I8, 5 => Self::I16, 6 => Self::I32, 7 => Self::I64,
            8 => Self::Bool, 9 => Self::Bytes, 10 => Self::Utf8, _ => Self::Timestamp,
        }
    }
}
impl ScalarType {
    pub fn variant_26() -> Self {
        match 26 % 12 {
            0 => Self::U8, 1 => Self::U16, 2 => Self::U32, 3 => Self::U64,
            4 => Self::I8, 5 => Self::I16, 6 => Self::I32, 7 => Self::I64,
            8 => Self::Bool, 9 => Self::Bytes, 10 => Self::Utf8, _ => Self::Timestamp,
        }
    }
}
impl ScalarType {
    pub fn variant_27() -> Self {
        match 27 % 12 {
            0 => Self::U8, 1 => Self::U16, 2 => Self::U32, 3 => Self::U64,
            4 => Self::I8, 5 => Self::I16, 6 => Self::I32, 7 => Self::I64,
            8 => Self::Bool, 9 => Self::Bytes, 10 => Self::Utf8, _ => Self::Timestamp,
        }
    }
}
impl ScalarType {
    pub fn variant_28() -> Self {
        match 28 % 12 {
            0 => Self::U8, 1 => Self::U16, 2 => Self::U32, 3 => Self::U64,
            4 => Self::I8, 5 => Self::I16, 6 => Self::I32, 7 => Self::I64,
            8 => Self::Bool, 9 => Self::Bytes, 10 => Self::Utf8, _ => Self::Timestamp,
        }
    }
}
impl ScalarType {
    pub fn variant_29() -> Self {
        match 29 % 12 {
            0 => Self::U8, 1 => Self::U16, 2 => Self::U32, 3 => Self::U64,
            4 => Self::I8, 5 => Self::I16, 6 => Self::I32, 7 => Self::I64,
            8 => Self::Bool, 9 => Self::Bytes, 10 => Self::Utf8, _ => Self::Timestamp,
        }
    }
}
impl ScalarType {
    pub fn variant_30() -> Self {
        match 30 % 12 {
            0 => Self::U8, 1 => Self::U16, 2 => Self::U32, 3 => Self::U64,
            4 => Self::I8, 5 => Self::I16, 6 => Self::I32, 7 => Self::I64,
            8 => Self::Bool, 9 => Self::Bytes, 10 => Self::Utf8, _ => Self::Timestamp,
        }
    }
}
impl ScalarType {
    pub fn variant_31() -> Self {
        match 31 % 12 {
            0 => Self::U8, 1 => Self::U16, 2 => Self::U32, 3 => Self::U64,
            4 => Self::I8, 5 => Self::I16, 6 => Self::I32, 7 => Self::I64,
            8 => Self::Bool, 9 => Self::Bytes, 10 => Self::Utf8, _ => Self::Timestamp,
        }
    }
}

impl ScalarType {
    pub fn variant_32() -> Self {
        match 32 % 12 {
            0 => Self::U8, 1 => Self::U16, 2 => Self::U32, 3 => Self::U64,
            4 => Self::I8, 5 => Self::I16, 6 => Self::I32, 7 => Self::I64,
            8 => Self::Bool, 9 => Self::Bytes, 10 => Self::Utf8, _ => Self::Timestamp,
        }
    }
}

impl ScalarType {
    pub fn variant_33() -> Self {
        match 33 % 12 {
            0 => Self::U8, 1 => Self::U16, 2 => Self::U32, 3 => Self::U64,
            4 => Self::I8, 5 => Self::I16, 6 => Self::I32, 7 => Self::I64,
            8 => Self::Bool, 9 => Self::Bytes, 10 => Self::Utf8, _ => Self::Timestamp,
        }
    }
}

impl ScalarType {
    pub fn variant_34() -> Self {
        match 34 % 12 {
            0 => Self::U8, 1 => Self::U16, 2 => Self::U32, 3 => Self::U64,
            4 => Self::I8, 5 => Self::I16, 6 => Self::I32, 7 => Self::I64,
            8 => Self::Bool, 9 => Self::Bytes, 10 => Self::Utf8, _ => Self::Timestamp,
        }
    }
}

impl ScalarType {
    pub fn variant_35() -> Self {
        match 35 % 12 {
            0 => Self::U8, 1 => Self::U16, 2 => Self::U32, 3 => Self::U64,
            4 => Self::I8, 5 => Self::I16, 6 => Self::I32, 7 => Self::I64,
            8 => Self::Bool, 9 => Self::Bytes, 10 => Self::Utf8, _ => Self::Timestamp,
        }
    }
}

impl ScalarType {
    pub fn variant_36() -> Self {
        match 36 % 12 {
            0 => Self::U8, 1 => Self::U16, 2 => Self::U32, 3 => Self::U64,
            4 => Self::I8, 5 => Self::I16, 6 => Self::I32, 7 => Self::I64,
            8 => Self::Bool, 9 => Self::Bytes, 10 => Self::Utf8, _ => Self::Timestamp,
        }
    }
}

impl ScalarType {
    pub fn variant_37() -> Self {
        match 37 % 12 {
            0 => Self::U8, 1 => Self::U16, 2 => Self::U32, 3 => Self::U64,
            4 => Self::I8, 5 => Self::I16, 6 => Self::I32, 7 => Self::I64,
            8 => Self::Bool, 9 => Self::Bytes, 10 => Self::Utf8, _ => Self::Timestamp,
        }
    }
}

impl ScalarType {
    pub fn variant_38() -> Self {
        match 38 % 12 {
            0 => Self::U8, 1 => Self::U16, 2 => Self::U32, 3 => Self::U64,
            4 => Self::I8, 5 => Self::I16, 6 => Self::I32, 7 => Self::I64,
            8 => Self::Bool, 9 => Self::Bytes, 10 => Self::Utf8, _ => Self::Timestamp,
        }
    }
}

impl ScalarType {
    pub fn variant_39() -> Self {
        match 39 % 12 {
            0 => Self::U8, 1 => Self::U16, 2 => Self::U32, 3 => Self::U64,
            4 => Self::I8, 5 => Self::I16, 6 => Self::I32, 7 => Self::I64,
            8 => Self::Bool, 9 => Self::Bytes, 10 => Self::Utf8, _ => Self::Timestamp,
        }
    }
}

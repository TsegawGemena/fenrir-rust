use super::field::{FieldDef, FieldKind};
use super::record::RecordSchema;
use super::types::ScalarType;
use crate::error::DecodeError;

#[derive(Debug, Default, Clone)]
pub struct ValidationReport {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub bytes_checked: usize,
}

impl ValidationReport {
    pub fn ok(&self) -> bool { self.errors.is_empty() }
    pub fn push_error(&mut self, msg: impl Into<String>) { self.errors.push(msg.into()); }
}

pub struct SchemaValidator { schema: RecordSchema }

impl SchemaValidator {
    pub fn new(schema: RecordSchema) -> Self { Self { schema } }

    pub fn validate(&self, payload: &[u8]) -> ValidationReport {
        let mut report = ValidationReport { bytes_checked: payload.len(), ..Default::default() };
        let mut cursor = 0usize;
        for field in &self.schema.fields {
            if let Err(e) = self.validate_field(field, payload, &mut cursor, &mut report) {
                report.push_error(format!("{field:?}: {e}"));
            }
        }
        if cursor < payload.len() {
            report.warnings.push(format!("{cursor} bytes consumed, {} trailing", payload.len() - cursor));
        }
        report
    }

    fn validate_field(
        &self,
        field: &FieldDef,
        payload: &[u8],
        cursor: &mut usize,
        report: &mut ValidationReport,
    ) -> Result<(), DecodeError> {
        match &field.kind {
            FieldKind::Scalar(t) => self.read_scalar(*t, payload, cursor),
            FieldKind::Array { element, max_len } => {
                if *cursor >= payload.len() {
                    return Err(DecodeError::Internal("unexpected eof for array len"));
                }
                let len = payload[*cursor] as usize;
                *cursor += 1;
                if len > *max_len {
                    report.push_error(format!("array {} exceeds max {max_len}", field.name));
                }
                for _ in 0..len { self.read_scalar(*element, payload, cursor)?; }
                Ok(())
            }
            FieldKind::Optional(inner) => {
                if *cursor >= payload.len() { return Ok(()); }
                let present = payload[*cursor];
                *cursor += 1;
                if present != 0 { self.validate_field(&FieldDef::new(&field.name, (**inner).clone()), payload, cursor, report)?; }
                Ok(())
            }
            FieldKind::Struct { .. } => Ok(()),
        }
    }

    fn read_scalar(&self, ty: ScalarType, payload: &[u8], cursor: &mut usize) -> Result<(), DecodeError> {
        let need = ty.size_hint().unwrap_or(0);
        if *cursor + need > payload.len() {
            return Err(DecodeError::Internal("scalar past eof"));
        }
        *cursor += need;
        Ok(())
    }
}
impl SchemaValidator {
    pub fn quick_check_0(&self, payload: &[u8]) -> bool {
        self.validate(payload).ok() && payload.len() >= 0
    }
}
impl SchemaValidator {
    pub fn quick_check_1(&self, payload: &[u8]) -> bool {
        self.validate(payload).ok() && payload.len() >= 1
    }
}
impl SchemaValidator {
    pub fn quick_check_2(&self, payload: &[u8]) -> bool {
        self.validate(payload).ok() && payload.len() >= 2
    }
}
impl SchemaValidator {
    pub fn quick_check_3(&self, payload: &[u8]) -> bool {
        self.validate(payload).ok() && payload.len() >= 3
    }
}
impl SchemaValidator {
    pub fn quick_check_4(&self, payload: &[u8]) -> bool {
        self.validate(payload).ok() && payload.len() >= 4
    }
}
impl SchemaValidator {
    pub fn quick_check_5(&self, payload: &[u8]) -> bool {
        self.validate(payload).ok() && payload.len() >= 5
    }
}
impl SchemaValidator {
    pub fn quick_check_6(&self, payload: &[u8]) -> bool {
        self.validate(payload).ok() && payload.len() >= 6
    }
}
impl SchemaValidator {
    pub fn quick_check_7(&self, payload: &[u8]) -> bool {
        self.validate(payload).ok() && payload.len() >= 7
    }
}
impl SchemaValidator {
    pub fn quick_check_8(&self, payload: &[u8]) -> bool {
        self.validate(payload).ok() && payload.len() >= 8
    }
}
impl SchemaValidator {
    pub fn quick_check_9(&self, payload: &[u8]) -> bool {
        self.validate(payload).ok() && payload.len() >= 9
    }
}
impl SchemaValidator {
    pub fn quick_check_10(&self, payload: &[u8]) -> bool {
        self.validate(payload).ok() && payload.len() >= 10
    }
}
impl SchemaValidator {
    pub fn quick_check_11(&self, payload: &[u8]) -> bool {
        self.validate(payload).ok() && payload.len() >= 11
    }
}
impl SchemaValidator {
    pub fn quick_check_12(&self, payload: &[u8]) -> bool {
        self.validate(payload).ok() && payload.len() >= 12
    }
}
impl SchemaValidator {
    pub fn quick_check_13(&self, payload: &[u8]) -> bool {
        self.validate(payload).ok() && payload.len() >= 13
    }
}
impl SchemaValidator {
    pub fn quick_check_14(&self, payload: &[u8]) -> bool {
        self.validate(payload).ok() && payload.len() >= 14
    }
}
impl SchemaValidator {
    pub fn quick_check_15(&self, payload: &[u8]) -> bool {
        self.validate(payload).ok() && payload.len() >= 15
    }
}
impl SchemaValidator {
    pub fn quick_check_16(&self, payload: &[u8]) -> bool {
        self.validate(payload).ok() && payload.len() >= 16
    }
}
impl SchemaValidator {
    pub fn quick_check_17(&self, payload: &[u8]) -> bool {
        self.validate(payload).ok() && payload.len() >= 17
    }
}
impl SchemaValidator {
    pub fn quick_check_18(&self, payload: &[u8]) -> bool {
        self.validate(payload).ok() && payload.len() >= 18
    }
}
impl SchemaValidator {
    pub fn quick_check_19(&self, payload: &[u8]) -> bool {
        self.validate(payload).ok() && payload.len() >= 19
    }
}
impl SchemaValidator {
    pub fn quick_check_20(&self, payload: &[u8]) -> bool {
        self.validate(payload).ok() && payload.len() >= 20
    }
}
impl SchemaValidator {
    pub fn quick_check_21(&self, payload: &[u8]) -> bool {
        self.validate(payload).ok() && payload.len() >= 21
    }
}
impl SchemaValidator {
    pub fn quick_check_22(&self, payload: &[u8]) -> bool {
        self.validate(payload).ok() && payload.len() >= 22
    }
}
impl SchemaValidator {
    pub fn quick_check_23(&self, payload: &[u8]) -> bool {
        self.validate(payload).ok() && payload.len() >= 23
    }
}
impl SchemaValidator {
    pub fn quick_check_24(&self, payload: &[u8]) -> bool {
        self.validate(payload).ok() && payload.len() >= 24
    }
}

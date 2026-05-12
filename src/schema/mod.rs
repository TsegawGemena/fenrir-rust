
pub mod field;
pub mod record;
pub mod types;
pub mod validate;

pub use field::{FieldDef, FieldKind};
pub use record::{RecordSchema, SchemaBuilder};
pub use types::{ScalarType, TypeId};
pub use validate::{SchemaValidator, ValidationReport};

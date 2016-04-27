//! This module contains functions called by serde to customize serialization of ECSRequest types.

pub fn is_none<T>(field: &Option<T>) -> bool {
    field.is_none()
}

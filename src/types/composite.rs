use std::collections::HashMap;

use crate::types::Type;

pub enum CompositeType {
	UserDefined(HashMap<String, Type>),
	Array(Box<Type>)
}

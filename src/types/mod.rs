mod composite;
mod primitive;

pub use composite::CompositeType;
pub use primitive::PrimitiveType;

pub enum Type {
	Primitive(PrimitiveType),
	Composite(CompositeType)
}

use std::hash::{Hash, Hasher};
pub struct ObjectKey(String);
impl Hash for ObjectKey {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.0.hash(state)
	}
}

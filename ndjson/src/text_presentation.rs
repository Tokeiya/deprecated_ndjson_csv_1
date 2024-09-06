#[derive(Eq, PartialEq)]
pub enum Foo {}

pub trait TextPresentation {
	fn raw(&self) -> String {
		todo!()
	}

	fn trimmed(&self) -> String;

	fn build(&self, buffer: &mut String);

	fn trimmed_build(&self, buffer: &mut String);
}
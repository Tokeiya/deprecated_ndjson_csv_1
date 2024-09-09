#[cfg(test)]
pub mod test_helper {

	pub fn add_spaces(target: &str) -> String {
		format!("\t \r  \t \n   {target}   \r\n")
	}
}

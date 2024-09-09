#[cfg(test)]
pub mod test_helper {

	//todo:move to with_raw_text module.
	pub fn add_spaces(target: &str) -> String {
		format!("\t \r  \t \n   {target}   \r\n")
	}
}

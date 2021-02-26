#[cfg(test)]
mod test {
	#[test]
	fn simple_test() {
		logger::log(log::Level::Info, "Some information");
	}
}

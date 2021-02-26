pub mod logger {
	use log::Level;

	pub fn log(level: Level, message: &str) {
		println!("[{}] {}", level, message);
	}

	pub fn log_info(message: &str) {
		log(Level::Info, message);
	}

	pub fn log_warn(message: &str) {
		log(Level::Warn, message);
	}

	pub fn log_error(message: &str) {
		log(Level::Error, message);
	}

	pub fn log_debug(message: &str) {
		log(Level::Debug, message);
	}
}

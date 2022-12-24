use configparser::ini::Ini;

pub fn create_repo(path: String, force: bool) {
	let nssdir = format!(path, ".nss");

	let mut config = Ini::new();
	let cf = repo_file(path);

	config.read(String::from("[somesection]someintvalue = 5")
	let my_value = config.getint("somesection", "someintvalue").unwrap().unwrap();
}

pub fn repo_file(path: String) {

}


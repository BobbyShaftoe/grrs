/// Function to find matches in a string and write them to a writer, and print any errors that occurred.
pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            match writeln!(writer, "{}", line) {
                Ok(_) => (),
                Err(e) => println!("Error: {}", e),
            }
        }
    }
}

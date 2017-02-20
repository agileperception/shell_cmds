use std;

/// Logic for the dirname command
/// See the manpage for dirname/basename (they share a man page)
pub fn dirname<'a>(input : &'a str) -> &'a str {
    // Trim any trailing slashes
    let mut result = input.trim_right_matches('/');
    // Delete from the last slash to the end
    if let Some(index) = result.rfind('/') {
        result = &result[..index];
    }
    result
}

/// Usage string for the dirname command
pub fn usage() {
    println!("usage: dirname path");
    std::process::exit(1);
}

#[cfg(test)]
mod tests {
    use super::dirname;

    #[test]
    fn test_dirname() {
        assert_eq!(dirname("/path/to/file"), "/path/to");
    }
}

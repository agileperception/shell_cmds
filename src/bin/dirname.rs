/*-
 * Copyright (c) 1991, 1993, 1994
 *	The Regents of the University of California.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. All advertising materials mentioning features or use of this software
 *    must display the following acknowledgement:
 *	This product includes software developed by the University of
 *	California, Berkeley and its contributors.
 * 4. Neither the name of the University nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE REGENTS AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */

/// Logic for the dirname command
/// See the manpage for dirname/basename (they share a man page)
fn dirname<'a>(input : &'a str) -> String {
    // Special Case: Empty String
    if input.len() == 0 {
        return ".".to_string()
    }
    // Trim whitespace, then convert to a vector of characters
    let mut characters : Vec<char> = input.trim().chars().collect();

    // Remove trailing slashes (but not the starting slash)
    while match (&characters[1..]).last() {
        Some(x) => (*x == '/'),
        None => false,
    } {
        characters.pop();
    }

    // Special Case: No Slashes remaining after initial trailing slashes removed
    match characters.iter().position(|&x| x == '/') {
        Some(_) => {},
        None => {
            return ".".to_string();
        }
    }

    // Delete anything after the last slash, inclusive
    match (&characters[1..]).iter().rposition(|&x| x == '/') {
        Some(index) => {
            characters.resize(index + 1, '#');
        },
        None => {},
    }

    // Remove trailing slashes (but not the starting slash) AGAIN
    // This isn't mentioned in the manpage, but is the actual behavior.
    // Am I just preserving a bug???
    while match (&characters[1..]).last() {
        Some(x) => (*x == '/'),
        None => false,
    } {
        characters.pop();
    }

    characters.into_iter().collect()
}

/// Usage string for the dirname command
fn usage() {
    println!("usage: dirname path");
    std::process::exit(1);
}

/// MAIN!
fn main() {
    let args : Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        usage();
    }

    if (&args[1]).starts_with("-") {
        usage();
    }

    println!("{}", dirname(&args[1]));
}

/// TESTS
#[cfg(test)]
mod tests {
    use super::dirname;

    #[test]
    fn test_trailing_slashes() {
        assert_eq!(dirname("/path/to/dir/"), "/path/to");
        assert_eq!(dirname("/path/to/file.txt/"), "/path/to");
        assert_eq!(dirname("/path/to/dir//"), "/path/to");
        assert_eq!(dirname("/"), "/");
        assert_eq!(dirname("//"), "/");
    }

    #[test]
    fn test_dot() {
        // DIRNAME(3)
        // If path is [...] the empty string, or contains no '/' characters, dirname() returns
        // [...] the string ".", signifying the current directory.
        assert_eq!(dirname(""), ".");
        assert_eq!(dirname("a"), ".");
        assert_eq!(dirname("rust"), ".");
        assert_eq!(dirname("file.ext"), ".");
    }

    #[test]
    fn test_multiple_intersticial_slashes() {
        assert_eq!(dirname("a///b"), "a");
        assert_eq!(dirname("/a///b"), "/a");
        // preserving behavior...or just preserving a bug???
        assert_eq!(dirname("a///b/c"), "a///b");
        assert_eq!(dirname("/a///b/c"), "/a///b");
        assert_eq!(dirname("///a///b"), "///a");
    }

    #[test]
    fn test_normal() {
        assert_eq!(dirname("a/b/c"), "a/b");
        assert_eq!(dirname("/a/b/c"), "/a/b");
    }

    #[test]
    fn test_extensions() {
        assert_eq!(dirname("a/b/file.ext"), "a/b");
        assert_eq!(dirname("/a/b/file.ext"), "/a/b");
    }
}


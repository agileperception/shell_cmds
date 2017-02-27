/*-
 * Copyright (c) 1988, 1993, 1994
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

use std::{env, thread};
use std::time::Duration;
use std::io::Write;

fn usage_and_die() {
    writeln!(&mut std::io::stderr(), "usage: sleep seconds").unwrap();
    std::process::exit(1);
}

fn get_duration_from_str(duration_str : &str) -> Duration {
    let mut time_float : f64 = match duration_str.trim().parse() {
        Ok(x) => x,
        Err(_) => {
            usage_and_die();
            0.0
        }
    };
    if time_float < 0.0 {
        usage_and_die();
    } 
    if time_float > std::u64::MAX as f64 {
        time_float = std::u64::MAX as f64;
    }
    Duration::new(time_float.trunc() as u64, (time_float.fract() * 1000000000.0) as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_duration_from_str() {
        assert_eq!(Duration::new(0, 0), get_duration_from_str("0"));
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();

    // Mirroring Apple's rudimentary argument parsing logic
    let mut time_string = String::new();
    if args.len() == 2 {
        time_string = args[1].clone();
    } else if (args.len() == 3) && (args[1] == "--") {
        // Comment from Apple's version:
        // -----------------------------
        // POSIX issue:   "sleep -- 3" should be the same as "sleep 3",
        // normally getopt makes this kind of thing work, but sleep has
        // no options, so we do it "the easy way"
        time_string = args[2].clone();
    } else {
        usage_and_die();
    }

    let duration = get_duration_from_str(&time_string);

    thread::sleep(duration);
}

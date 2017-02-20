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

extern crate clap;
use clap::{Arg, App};

fn basename<'a>(input: &'a str, suffix: &str) -> &'a str {
    // Trim surrounding whitespace
    let mut result = input.trim();
    // Remove any trailing slashes from the right side
    result = result.trim_right_matches('/');
    // Remove everything up to the last forward slash, if there is one
    if let Some(index) = result.rfind('/') {
        result = &result[index+1..];
    }
    // Remove the suffix if we find one, unless that's all that is left
    if (suffix.len() > 0)
    && (result.len() > suffix.len())
    && result.ends_with(suffix) {
        result = &result[..result.len()-suffix.len()];
    }
    result
}

fn main() {
    // Ancient utilities are frustrating because their behavior with arguments makes no blasted
    // sense.  `basename` is one of these.  If it has exactly two positional arguments, then it
    // acts completely differently than in all other cases, unless "-a" was specified, in which
    // case it acts normally still.
    
    let matches = App::new("basename")
                         .usage("basename string [suffix]\n    basename [-a] [-s suffix] string [...]")
                         .arg(Arg::with_name("all")
                              .short("a"))
                         .arg(Arg::with_name("suffix")
                              .short("s")
                              .takes_value(true))
                         .arg(Arg::with_name("string")
                              .multiple(true))
                         .get_matches();
    let all = matches.is_present("all");
    let suffix = matches.value_of("suffix").unwrap_or("");

    if let Some(strings) = matches.values_of("string") {
        let args : Vec<&str> = strings.clone().collect();

        // Exactly two positional arguments
        if (args.len() == 2) && !all {
            println!("{}", basename(&args[0], &args[1]));
            return
        }

        // Every other case
        for string in strings {
            println!("{}", basename(string, suffix));
        }

    } else {
        println!("{}", matches.usage());
        std::process::exit(1);
    }

}

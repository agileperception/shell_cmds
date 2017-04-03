/*
 * Copyright (c) 1988, 1993
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

extern crate getopts;
extern crate libc;

use getopts::Options;
use libc::{signal, SIGINT, SIG_IGN};
use std::env;
use std::fs::OpenOptions;
use std::io::{Read, Write};


fn warn(msg : String) {
    std::io::stderr().write(format!("tee: {}\n", msg).as_bytes())
        .expect("Failed writing to stderr");
}

fn main() {
    let usage = "usage: tee [-ai] [file ...]";

    // See manpage tee.1 for option descriptions. They never occur in output, so we don't include
    // them here.
    let mut opts = Options::new();
    opts.optflag("a", "append", "");
    opts.optflag("i", "ignore_sigint", "");

    // Parse options
    let args: Vec<String> = env::args().collect();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            println!("tee: {}", e.to_string());
            println!("{}", usage);
            std::process::exit(1);
        }
    };

    // Ignore SIGINT?
    if matches.opt_present("ignore_sigint") {
        unsafe {
            signal(SIGINT, SIG_IGN);
        }
    }

    // Open each of the files and push them on a vector
    let mut filehandles : Vec<Box<Write>> = Vec::new();
    filehandles.push(Box::new(std::io::stdout()));
    let mut filenames = matches.free.clone();
    for filename in &filenames {
        let mut open_options = OpenOptions::new();
        open_options.create(true);
        if matches.opt_present("append") {
            open_options.append(true);
        } else {
            open_options.write(true).truncate(true);
        }
        match open_options.open(&filename) {
            Ok(fh) => filehandles.push(Box::new(fh)),
            Err(e) => warn(format!("{}: {}", filename, e)),
        };
    }
    filenames.insert(0, String::from("stdout"));

    // MAIN LOOP - Read from stdin, write to stdout and each file.

    // The size of the buffer in the C implementation.  Should we increase it?
    const BUFSIZE : usize = 8192;
    let mut buffer_in = vec![0; BUFSIZE];
    let mut stdin = std::io::stdin();
    loop {
        // Read
        let num_bytes = match stdin.read(&mut buffer_in) {
            Ok(n) => n,
            Err(e) => panic!("{}", e.to_string()),
        };
        // 0 bytes means we hit EOF
        if num_bytes == 0 {
            break;
        }
        // Write
        let buffer_out = buffer_in.clone();
        for (filename, mut filehandle) in filenames.iter().zip(&mut filehandles) {
            match filehandle.write(&buffer_out[..num_bytes]) {
                Ok(bytes_written) => {
                    if bytes_written != num_bytes {
                        warn(format!("Failed writing all bytes to {}.  Location may be full.",
                                     filename));
                    }
                },
                Err(e) => warn(format!("{}: {}", filename, e)),
            }
        }
    }
}


/*-
 * Copyright (c) 1987, 1993
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

// The code this was ported from also included this immediately before the C main() function:

/*
 * printenv
 *
 * Bill Joy, UCB
 * February, 1979
 */

fn main() {
    let args : Vec<String> = std::env::args().collect();

    // printenv doesn't take "options".  Trying to use any triggers usage.
    let mut options = args.clone();
    options.retain(|ref x| x.starts_with('-'));
    if !options.is_empty() {
        println!("usage: printenv [name]");
        std::process::exit(1);
    }

    // With no arguments, printenv just prints all the environment variables in KEY=VALUE style
    if args.len() == 1 {
        for (key, value) in std::env::vars() {
            println!("{}={}", key, value);
        }
        std::process::exit(0);
    }

    // With any arguments, we print out the value of the first one if found. Rest are ignored.
    if let Some(arg) = args.get(1) {
        for (key, value) in std::env::vars() {
            if &key == arg {
                println!("{}", value);
                std::process::exit(0);
            }
        }
    }
    std::process::exit(1);
}

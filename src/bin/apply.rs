/*-
 * Copyright (c) 1994
 *	The Regents of the University of California.  All rights reserved.
 *
 * This code is derived from software contributed to Berkeley by
 * Jan-Simon Pendry.
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

use std::str::FromStr;

fn errx(msg : &str) {
    if !msg.is_empty() {
        println!("{}", msg);
    }
    std::process::exit(1);
}

fn main() {
    let args = App::new("apply")
        .usage("apply [-a magic] [-d] [-0123456789] command arguments ...")
        .arg(Arg::with_name("magic_char")
             .short("a")
             .takes_value(true)
             .default_value("%"))
        .arg(Arg::with_name("debug")
             .short("d"))
        .arg(Arg::with_name("0").short("0"))
        .arg(Arg::with_name("1").short("1"))
        .arg(Arg::with_name("2").short("2"))
        .arg(Arg::with_name("3").short("3"))
        .arg(Arg::with_name("4").short("4"))
        .arg(Arg::with_name("5").short("5"))
        .arg(Arg::with_name("6").short("6"))
        .arg(Arg::with_name("7").short("7"))
        .arg(Arg::with_name("8").short("8"))
        .arg(Arg::with_name("9").short("9"))
        .arg(Arg::with_name("command")
             .required(true)
             .takes_value(true))
        .arg(Arg::with_name("command_arguments")
             .required(true)
             .multiple(true)
             .takes_value(true))
        .get_matches();

    let magic_char = args.value_of("magic_char").unwrap();
    if magic_char.len() != 1 {
        errx("illegal magic character specification");
    }

    let debug = args.is_present("debug");

    let command = args.value_of("command").unwrap();

    let command_arguments : Vec<&str> = args.values_of("command_arguments").unwrap().collect();

    // Set num_args_per_command based off of one of the -0123456789 arguments.
    let mut num_args_per_command : i8 = -1;
    for number_str in &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"] {
        if args.is_present(number_str) {
            if num_args_per_command != -1 {
                errx("only one -# argument may be specified");
            }
            num_args_per_command = i8::from_str(number_str).unwrap();
        }
    }

    // Find the highest numerical value seen in the command string
    let mut max_argnum_in_command : i8 = 0;
    for number_char in &["9", "8", "7", "6", "5", "4", "3", "2", "1"] {
        let mut search_str = String::with_capacity(2);
        search_str += magic_char;
        search_str += number_char;
        if command.find(&search_str).is_some() {
            max_argnum_in_command = i8::from_str(number_char).unwrap();
            break;
        }
    }

    let shell = match std::env::var("SHELL") {
        Ok(s) => s,
        // Note: The old BSD code uses the value of _PATH_BSHELL from /usr/include/paths.h, which
        // appears to have been /bin/sh for the last 20 years.
        Err(_) => "/bin/sh".to_string(),
    };

    let mut command_template = String::from_str("exec ").unwrap();
    if max_argnum_in_command == 0 {
        if num_args_per_command == -1 {
            num_args_per_command = 1;
        }
        command_template.push_str(command);
        for i in 1..(num_args_per_command + 1) {
            command_template.push(' ');
            command_template.push_str(magic_char);
            command_template.push_str(&(i.to_string()));
        }
        // XXX
    } else {
        command_template.push_str(command);
        num_args_per_command = max_argnum_in_command;
        
    }

    println!("shell is {}", shell);
    println!("debug is {}", debug);
    println!("magic_char is {:?}", magic_char);
    println!("num_args_per_command is {}", num_args_per_command);
    println!("max_argnum_in_command is {}", max_argnum_in_command);
    println!("command is {}", command);
    println!("command_arguments are {:?}", command_arguments);
    println!("command_template is {}", command_template)


}

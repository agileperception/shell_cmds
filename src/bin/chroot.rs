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
use std::env;
use libc::{c_int, chroot, gid_t, setgid, setgroups, setuid, uid_t};


fn usage() {
    println!("usage: chroot [-g group] [-G group,group,...] [-u user] newroot [command]");
}

fn get_group_id(group: String) -> gid_t {
    // Maybe it's a number
    if let Ok(group_id) = group.parse::<gid_t>() {
        return group_id;
    }
    // TODO: Parse by group name, fail if we can't

    return 0;
}

fn main() {
    let mut opts = Options::new();
    opts.optopt("u", "user", "user to switch to before running program", "USER");
    opts.optopt("g", "group", "group to switch to before running program", "GROUP");
    opts.optopt("G", "grouplist", "group list to switch to before running program", "GROUPLIST");
    let args: Vec<String> = env::args().collect();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(_) => {
            usage();
            std::process::exit(1);
        }
    };
    // Supplying newroot is required
    if matches.free.len() < 1 {
        usage();
        std::process::exit(1);
    }
    let newroot = matches.free.get(0).expect("Vec lied to us about its length.");

    // The BSD version used the value of NGROUPS_MAX from /usr/include/sys/syslimits.h
    // We should probably do that too instead of hard-coding it here
    const NGROUPS_MAX: i32 = 16;

    if matches.opt_present("user") {
        let user = matches.opt_str("user").expect("getopts lied to us about user");
        // TODO: do the stuff here...
    }
    if matches.opt_present("group") {
        let group = matches.opt_str("group").expect("getopts lied to us about group");
        let group_id = get_group_id(group);
        // TODO: Are we done here?
    }
    if matches.opt_present("grouplist") {
        let grouplist = matches.opt_str("grouplist").expect("getopts lied to us about grouplist");
        // TODO: do the stuff here...
    }

    // TODO: Perform the chroot

    // Run a user-supplied command
    if let Some(command) = matches.free.get(1) {
        // TODO: Actually run the command...
        println!("In root {} executing {}", newroot, command);
    }

    // Just run a shell.  Prefer the user's chosen shell.
    let shell = match std::env::var("SHELL") {
        Ok(s) => s,
        // Note: The old BSD code uses the value of _PATH_BSHELL from /usr/include/paths.h, which
        // appears to have been /bin/sh for time immemorial.  We should probably do that too,
        // instead of hard-coding it here.
        Err(_) => "/bin/sh".to_string(),
    };

    // TODO: Actually run the shell
    println!("In root {} running shell {}", newroot, shell);

}

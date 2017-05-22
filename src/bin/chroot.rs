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
use std::ffi::CString;
use libc::{c_int, chdir, chroot, execvp, execlp, getgrnam, getpwnam, gid_t, setgid, setgroups, setuid, strerror, uid_t};

#[link(name = "c")]
extern {
    static mut errno: c_int;
}

fn usage() {
    println!("usage: chroot [-g group] [-G group,group,...] [-u user] newroot [command]");
}

fn get_group_id<S: Into<String>>(group: S) -> gid_t {
    let group = group.into();
    // Maybe it's a number
    if let Ok(group_id) = group.parse::<gid_t>() {
        return group_id;
    }
    let cstring = CString::new(group).unwrap();
    unsafe {
        let group_struct = getgrnam(cstring.as_ptr());
        if group_struct.is_null() {
            println!("no such group {}", cstring.into_string().unwrap());
            std::process::exit(1);
        } else {
            return (*group_struct).gr_gid;
        }
    }
}

fn get_user_id<S: Into<String>>(user: S) -> uid_t {
    let user = user.into();
    // Maybe it's a number
    if let Ok(user_id) = user.parse::<uid_t>() {
        return user_id;
    }
    let cstring = CString::new(user).unwrap();
    unsafe {
        let user_struct = getpwnam(cstring.as_ptr());
        if user_struct.is_null() {
            println!("no such user {}", cstring.into_string().unwrap());
            std::process::exit(1);
        } else {
            return (*user_struct).pw_uid;
        }
    }
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
    let newroot = matches.free.get(0).unwrap();

    // Resolve the group argument into a gid
    let mut group_id : gid_t = 0;
    if matches.opt_present("group") {
        let group = matches.opt_str("group").unwrap();
        group_id = get_group_id(group);
        println!("{}", group_id);
    }
    // Resolve the grouplist argument into a vector of gids
    let mut grouplist_ids : Vec<gid_t> = Vec::new();
    if matches.opt_present("grouplist") {
        let grouplist = matches.opt_str("grouplist").unwrap();
        let grouplist_vec: Vec<&str> = grouplist.split(',').collect();
        for grouplist_entry in grouplist_vec {
            grouplist_ids.push(get_group_id(grouplist_entry));
        }
        println!("{:?}", grouplist_ids);
    }
    // Resolve the user argument into a uid
    let mut user_id : uid_t = 0;
    if matches.opt_present("user") {
        let user = matches.opt_str("user").unwrap();
        user_id = get_user_id(user);
        println!("{}", user_id);
    }

    // Change directory
    let newroot_c = CString::new(newroot.to_string()).unwrap();
    unsafe {
        if chdir(newroot_c.as_ptr()) < 0 {
            let err_msg = CString::from_raw(strerror(errno));
            println!("chroot: {}: {}", newroot, err_msg.into_string().unwrap());
            std::process::exit(1);
        }
    }
    // Perform chroot
    let dot_dir = CString::new(".").unwrap();
    unsafe {
        if chroot(dot_dir.as_ptr()) < 0 {
            let err_msg = CString::from_raw(strerror(errno));
            println!("chroot: {}: {}", newroot, err_msg.into_string().unwrap());
            std::process::exit(1);
        }
    }

    // Order of setgroups, setgid, and setuid calls are preserved from original -- because I don't
    // know if the order makes any difference.
    if matches.opt_present("grouplist") {
        unsafe {
            if setgroups(grouplist_ids.len() as i32, grouplist_ids.as_ptr()) < 0 {
                let err_msg = CString::from_raw(strerror(errno));
                println!("chroot: setgroups: {}", err_msg.into_string().unwrap());
                std::process::exit(1);
            }
        }
    }
    if matches.opt_present("group") {
        unsafe {
            if setgid(group_id) < 0 {
                let err_msg = CString::from_raw(strerror(errno));
                println!("chroot: setgid: {}", err_msg.into_string().unwrap());
                std::process::exit(1);
            }
        }
    }
    if matches.opt_present("user") {
        unsafe {
            if setuid(user_id) < 0 {
                let err_msg = CString::from_raw(strerror(errno));
                println!("chroot: setuid: {}", err_msg.into_string().unwrap());
                std::process::exit(1);
            }
        }
    }

    // Run a user-supplied command
    if let Some(command) = matches.free.get(1) {
        let command_c = CString::new(command.as_str()).unwrap();
        unsafe {
            execvp(command_c.as_ptr(), &command_c.as_ptr());
            let err_msg = CString::from_raw(strerror(errno));
            println!("{}: {}", command, err_msg.into_string().unwrap());
        }
        std::process::exit(1);
    }

    // No user-supplied command.  Run a shell instead.

    // Determine which shell we should use
    let shell = match std::env::var("SHELL") {
        Ok(s) => s,
        // Note: The old BSD code uses the value of _PATH_BSHELL from /usr/include/paths.h, which
        // appears to have been /bin/sh for time immemorial.  We should probably do that too,
        // instead of hard-coding it here.
        Err(_) => "/bin/sh".to_string(),
    };

    // Run the shell
    let shell_c = CString::new(shell.as_str()).unwrap();
    let dash_i_c = CString::new("-i").unwrap();
    unsafe {
        execlp(shell_c.as_ptr(), shell_c.as_ptr(), dash_i_c.as_ptr());
        let err_msg = CString::from_raw(strerror(errno));
        println!("{}: {}", shell, err_msg.into_string().unwrap());
    }
    std::process::exit(1);
}

Rust port of Apple's [shell_cmds](https://opensource.apple.com/source/shell_cmds/shell_cmds-198/)
=================================

Port Apple's shell commands from C to Rust.

Contributing
============

-  Want to help? Contact me by
   [creating an issue](https://github.com/agileperception/shell_cmds/issues/new)
   or join my [RustProgramming Discord Channel](https://discord.gg/pR7hBBe)

- Code to port: [shell_cmds version 198](https://opensource.apple.com/source/shell_cmds/shell_cmds-198/)

- Commands need a file in `src/bin/` with a `fn main()`.  For example `echo` has `src/bin/echo.rs`

- To compile+run your new command (for example) `echo arg1 arg2` you would do `cargo run --bin echo -- arg1 arg2`

- Use `std::env::args()` directly for the dirt-simple utilities.  Use
  [getopts](https://doc.rust-lang.org/getopts/getopts/index.html) for the fancier ones.

- Put the man pages (files ending in `.1`) in the `man/` directory.

- Put the companion shell scripts in the `sh/` directory.

- When there's a license header, copy it over verbatim into the `.rs` file for the binary. (Lawyer repellent.)

Porting Status
==============

* [x] alias - Just a man page pointer to `builtin.1`, which is `csh`'s manpage.
* [x] apply - Some serious pointer-loop reverse engineering on this one.
* [x] basename - Ancient utilities are frustrating because their behavior with
  arguments makes no blasted sense.  `basename` is one of these.  If it has
  exactly two arguments, then it acts completely differently.
* [ ] chroot - In Progress
* [ ] date
* [x] dirname - Shares a man page with basename.
* [x] echo - Got an education in rust Strings.
* [ ] env
* [ ] expr
* [x] false - Simple.
* [ ] find
* [ ] getopt
* [ ] hexdump
* [ ] hostname
* [ ] id
* [ ] jot
* [ ] kill
* [ ] killall
* [ ] lastcomm
* [ ] locate
* [ ] logname
* [ ] mktemp
* [ ] nice
* [ ] nohup
* [ ] path_helper
* [x] printenv - Done. Did you know printenv silently ignores any extra arguments?
* [ ] pwd
* [ ] renice
* [ ] script
* [ ] seq
* [ ] sh
* [ ] shlock
* [x] sleep - Instead of treating invalid input as 0 silently, we spit out the
  usage and die.
* [ ] su
* [ ] systime
* [x] tee - Done.  Good practice with `Vec`, `zip()`, `stdin/stdout/stderr`, and files.
* [ ] test
* [ ] time
* [x] true - Simple.
* [ ] uname
* [ ] users
* [ ] w
* [ ] what
* [ ] whereis
* [ ] which
* [ ] who
* [ ] xargs
* [x] yes - Did you know that `yes` takes an optional argument?

Bugs in Original
================

Below is a partial list of bugs discovered in C code of Apple's [shell_cmds version 198](https://opensource.apple.com/source/shell_cmds/shell_cmds-198/) -- which ships on macOS Sierra.

chroot.c
--------

- If more than 16 valid groups are provided to the `-G` option, then the
  `gidlist` buffer overflows and starts overwriting later data in the stack
  frame with resolved group ids.  That is not difficult to do since macOS ships
  with over 100 valid groups by default.  In Rust, we use a `Vec` to store the
  resolved group ids.  `Vec` is dynamically sized, so it won't overflow.




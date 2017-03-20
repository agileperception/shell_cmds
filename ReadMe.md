# Rust port of Apple's [shell_cmds](https://opensource.apple.com/source/shell_cmds/shell_cmds-198/)

This is my project to port a bunch of Apple's utilities from C to Rust to learn
and have fun.  If you would like to join in, please contact me by making an
issue or joining my
[RustProgramming Discord Channel](https://discord.gg/pR7hBBe)

# Contributing

- We're porting [shell_cmds version 198](https://opensource.apple.com/source/shell_cmds/shell_cmds-198/)

- Commands need a file in `src/bin/` with a `fn main()`.  For example `echo` has `src/bin/echo.rs`

- To compile+run your new command `echo arg1 arg2`: `cargo run --bin echo -- arg1 arg2`

- Use `std::env::args()` directly for the dirt-simple utilities.  Use
  [clap](https://crates.io/crates/clap) for the fancier ones (even though it
  means our argument parsing will look and behave nicer).

- Put the man pages (files ending in `.1`) in the `man/` directory.

- Put the companion shell scripts in the `sh/` directory.

- When there's a license header, copy it over verbatim. (Lawyer repellent.)

# Command Status

* [ ] alias
* [x] apply - Some serious pointer-loop reverse engineering on this one.
* [x] basename - Ancient utilities are frustrating because their behavior with
  arguments makes no blasted sense.  `basename` is one of these.  If it has
  exactly two arguments, then it acts completely differently.
* [ ] chroot
* [ ] date
* [x] dirname - Shares a man page with basename.
* [x] echo - Got an educiation in rust Strings.
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
* [ ] printenv
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
* [-] tee - Work has begun
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
* [x] yes - It works! Did you know that yes takes an optional argument?

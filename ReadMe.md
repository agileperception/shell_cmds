# Rust port of Apple's [shell_cmds](https://opensource.apple.com/source/shell_cmds/shell_cmds-198/)

Port Apple's shell commands from C to Rust.

# Contributing

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

# Command Status

* [ ] alias
* [x] apply - Some serious pointer-loop reverse engineering on this one.
* [x] basename - Ancient utilities are frustrating because their behavior with
  arguments makes no blasted sense.  `basename` is one of these.  If it has
  exactly two arguments, then it acts completely differently.
* [ ] chroot
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

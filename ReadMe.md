# Rust port of Apple's [shell_cmds](https://opensource.apple.com/source/shell_cmds/shell_cmds-198/)

This is my project to port a bunch of Apple's utilities from C to Rust to
learn.  And have fun.  And learn while I have fun.  Would you like to have some
fun too? If you think it might be fun, tell me and I'll grant you commit
access.  To contact me, either make an issue, or join me on my
[RustProgramming Discord Channel](https://discord.gg/pR7hBBe)

# Guidelines

- I'm targeting [shell_cmds version 198](https://opensource.apple.com/source/shell_cmds/shell_cmds-198/)
- Keep command names and flags exactly the same.
- The rust code should be as *idiomatic* as possible (aka do it the Rust way)
- Most of the original internal variable names are detestably awful. Rename
  them something meaningful, for heaven's sake.
- There's a bunch of ~25-year-old bugs.  Don't bother porting the bugs.
- No pull requests.  Just push to master.  You might have to pull first ;-)
- Use `std::env::args()` directly for the dirt-simple utilities.  Use
  [clap](https://crates.io/crates/clap) for the fancier ones (even though it
  means our argument parsing will look and behave nicer).
- Lets make unit tests.  Running tests is fun.
- Put the man pages (files ending in `.1`) in the `man/` directory.
- Put the companion shell scripts in the `sh/` directory.
- When there's a license header, copy it over verbatim.  Lawyer repellent.


# Contributing

- Commands need a file in `src/bin/` with a `fn main()`.  For example `echo` has `src/bin/echo.rs`

- To run your new command `echo`: `cargo run --bin echo`

- If there's some *actually* duplicated or reusable code somewhere, lets put it
in a properly named library module.  Like `src/echo.rs` or something relevant.

If there's a better way to do it, tell me about it.  Lets give it a shot.

# Command Status

* [ ] alias
* [ ] apply
* [x] basename - Ancient utilities are frustrating because their behavior with arguments makes no blasted sense.  `basename` is one of these.  If it has exactly two arguments, then it acts completely differently.
* [ ] chroot
* [ ] date
* [x] dirname - Shares a man page with basename.
* [x] echo - Strings are a pain in Rust.  Eventually, they'll get easier.
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
* [x] sleep - Instead of treating invalid input as 0 silently, we spit out the usage and die.
* [ ] su
* [ ] systime
* [ ] tee
* [ ] test
* [ ] time
* [x] true - It works flawlessly.  100% compatible.  AMAZING!
* [ ] uname
* [ ] users
* [ ] w
* [ ] what
* [ ] whereis
* [ ] which
* [ ] who
* [ ] xargs
* [x] yes - It works! I learned that yes takes an optional argument.

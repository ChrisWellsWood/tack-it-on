# tack-it-on
### Tack notes onto your project files and directories!

[![Latest Version]][crates.io]

[Latest Version]: https://img.shields.io/crates/v/tack-it-on.svg
[creates.io]: https://crates.io/crates/tack-it-on
## Overview

Ever been working on a project and wanted to write a quick note about something
you're doing? Maybe you want to keep track of a file you're working on or save
some information that you keep using repeatedly, but you don't want to clog up
your readme or add superfluous comments. `tack-it-on` solves this problem by
creating project centric notes in a hidden directory.

## Getting started 

Install the [Rust toolchain](https://www.rust-lang.org/en-US/install.html) and 
install with `cargo`:

```bash
cargo install tack-it-on
```

Once you've installed `tack-it-on`, you can tack notes onto any project:

```bash
mkdir my_project
cd my_project
tack init
```

You can add a general note like so:

```bash
tack note "I should add important_file.txt!"
```

You can see all notes in your project:

```bash
tack show
```

Out:

```
[25610142] 2017-10-08 23:21:48.390531318 +01:00
I should add important_file.txt!
```

(The ID for your note may be different)

You can add a note onto a specific file:

```bash
touch important_file.txt
tack note --on important_file.txt "This file is really important."
tack show
```

Out:

```
[25610142] 2017-10-08 23:21:48.390531318 +01:00
I should add important_file.txt!

[17355568] 2017-10-08 23:26:17.579074514 +01:00
On important_file.txt: This file is really important!
```

You can delete notes using the `rm` subcommand:

```bash
tack rm --id 17355568
```

You can truncate the ID as long as it's unique:

```bash
tack rm --id 2
```

## Development Roadmap

- [ ] More options for `show`.
- [ ] Add user information to notes.
- [ ] Add tagging system.
- [ ] Global notes.

If you have any ideas for new features, please make an issue requesting it, or
fork the repo, add it and make a pull request!

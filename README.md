# did

A tool to keep track of the activities done, inspired by this post: [did.txt file][blog].

## Usage

```sh
$ did "Writing the README"
```

This will produce a `~/did.txt` file with the logs appended.

## Installation

You need [cargo][cargo] to install and build the `did` command for your computer

```sh
cargo install did
```

### Options

**-t --ticket**

Prepends a ticket id to the message. Useful for Jira

[blog]: https://theptrk.com/2018/07/11/did-txt-file/
[cargo]: https://github.com/rust-lang/cargo

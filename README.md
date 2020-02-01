# SSHFS manager

Mounts and unmounts filesystems over SSH, using `~/.ssh/config` aliases and public key authentication.

Unfinished project, but works for some use cases. Written in Rust, though I have no
idea why I chose it over Python or Bash.

## Installation

First, install `sshfs`, e.g. via `apt update && apt install -y sshfs`.

Install Rust toolchain from [rust-lang.org](https://www.rust-lang.org/tools/install).
If you use rustup (recommended, no root needed), it will manage toolchain versions
for you, otherwise install `nightly-2019-08-24` or similar.

Then run:

```bash
git clone --depth 1 https://github.com/pzmarzly/sshfs-manager
cd sshfs-manager
cargo install --path .
cd ..
# optionally: rm -rf sshfs-manager
```

The `sshfs` binary will be added to `~/.cargo/bin`.

Tip: since it's a compiled binary, you can set SUID for it (unlike bash script, where
it'd be ignored).

## Usage

Create `~/.ssh/config` file and rename your key to `~/.ssh/id_rsa`. The config file
can be left empty, though you may want to use it as an address book, e.g.

```text
Host vps
    Port 6022
    User root
    Hostname 11.22.33.44

Host hosting
    Port 22
    User user123
    Hostname 22.33.44.55
```

Then run e.g.

```bash
sshfs vps
```

where `vps` is an alias in `~/.ssh/config` or an IP address. Run the same command again to unmount. You can have multiple shares mounted at the same time, as long as they are on different servers.

### Note

Back when I was writing this script, I wrote in the README that before using
it, you will need to uncomment `user_allow_other` in `/etc/fuse.conf`. But it works
fine with that line commented, unless I'm forgetting about something.

Perhaps I intended to make the script no longer require root privileges. I'm not
actively maintaining this repo, but I guess it would be easy to change mount points
to e.g. `/tmp/...`.

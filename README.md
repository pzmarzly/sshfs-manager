# SSHFS manager

Mounts and unmounts filesystems over SSH, using `~/.ssh/config` aliases and public key authentication.

## Usage

Before using, uncomment `user_allow_other` in `/etc/fuse.conf`. Then, create `~/.ssh/config` and (for now) rename your key to `~/.ssh/id_rsa`.

Then run:

```bash
sfs server1
```

where `server1` is an alias in `~/.ssh/config` or IP address. Run the same command again to unmount. You can have multiple shares mounted at the same time, as long as they are on different servers.

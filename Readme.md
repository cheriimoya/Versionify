# Versionify

Run this by copying and adjusting the `_.env` file to `.env` in the same directory.

Afterwards simply run `nix-shell` and `cargo r`.
A git repository should be created at the path you specified and there should be an empty initial commit and one additional commit per run.

TODOs:

- [ ] Don't create empty commits if nothing happened
- [ ] Logging
- [ ] Filter what is being saved
- [ ] Add rollback option to reset playlist to a given checkpoint
- [ ] Push repo to remote

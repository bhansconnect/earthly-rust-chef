This is just to show an issue with using [Earthly](https://earthly.dev/) and [Chef](https://www.lpalmieri.com/posts/fast-rust-docker-builds/) together.
The goal is pretty simple have one target that builds our dependencies and gets cached.
Then have another target that builds are actual libs and binaries.
That way, only when dependencies change will we need to run the much longer dependency step.

The crates essentially have random dependencies for the fun of it.

The current Earthfile works, but it is a hassle for big projects due to all of the manual
copying and making of files.

The `Earthfile - Suggested` more directly maps what is suggested to be done in a Dockerfile.
The issue is that it doesn't work. With any file changes it will rebuild all dependencies.
The goal would be for it to just recalculate `recipes.json`,
but then realize it is the same as the last version and use caching from then on.

To test when caching should work, modify any `.rs` file.\
To test when caching shouldn't work, modify a `Cargo.toml` file.\
Note that if you don't change the deps or add a new target, even `Cargo.toml` changes should cache.

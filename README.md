# PHP Engine CNB in Rust

This very basic buildpack implements a CNB in Rust to install a hard-coded version of PHP and launch a web process using PHP's built-in web server.

## Development

### Prerequisites

- Rust (use [rustup](https://rustup.rs/), or `brew install rustup && rustup-init`)
- musl target for rustup: `rustup target add x86_64-unknown-linux-musl`
    - needs [homebrew-musl-cross](https://github.com/FiloSottile/homebrew-musl-cross) (`brew install FiloSottile/musl-cross/musl-cross`) on macOS
- [cargo-make](https://github.com/sagiegurari/cargo-make): `cargo install cargo-make`
- [pack](https://buildpacks.io/docs/tools/pack/#install) (and Docker, obviously)

### Run tests

```ShellSession
$ cargo test
```

### Testing/building an app

```ShellSession
$ cargo make pack --profile "production"
$ pack build phpcnb-hello-world --path test/fixtures/hello-world -B heroku/buildpacks:18 --buildpack ./target -v
```

#### Does `php` work?

```ShellSession
$ docker run --rm -ti --entrypoint launcher phpcnb-hello-world php -v
```

#### Launch a web server and call it

```ShellSession
$ docker run --rm -d -p 8080:8080 -e PORT=8080 phpcnb-hello-world
$ curl http://localhost:8080
hello world
```

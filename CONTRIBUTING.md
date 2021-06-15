# How to Contribute to Sampitor

Thank you for your interest in contributing to Sampitor. This guide will assist
you in setting up a development environment, understanding the project tooling,
and learning the code conventions.

## Dependencies

Sampitor is written in the [Rust](https://www.rust-lang.org/) programming
language and uses [cargo-make](https://github.com/sagiegurari/cargo-make) as a
task runner. If they are not installed, see the
[Rust installation guide](https://www.rust-lang.org/tools/install) and
[cargo-make installation documentation](https://github.com/sagiegurari/cargo-make#installation).

On Linux platforms, the ALSA development libraries may need to be installed.
Check the table for required packages.

| Distribution | Package        |
| :----------- | :------------- |
| Alpine       | alsa-lib-dev   |
| Arch         | alsa-lib       |
| Debian       | libasound2-dev |
| Fedora       | alsa-lib-devel |

On macOS and Windows platforms, the native audio host development libraries are
pre-installed.

Sampitor uses [CPAL](https://github.com/RustAudio/cpal) for communicating with
audio hosts. If developing with a non-default operating system audio host, see
the [CPAL repository](https://github.com/RustAudio/cpal) for dependency
requirements.

## Getting Started

Once the dependencies are installed, the project can be downloaded and its test
suite executed.

```console
git clone https://github.com/wolfgangwazzlestrauss/sampitor
cd sampitor
cargo make all
```

The cargo-make test suite includes all checks that are enforced by the
continuous integration pipelines.

## Code Conventions

Sampitor follows the official
[Rust Style Guide](https://github.com/rust-dev-tools/fmt-rfcs/blob/master/guide/guide.md)
and the official
[Rust API Guidelines](https://rust-lang.github.io/api-guidelines/).
[Clippy](https://github.com/rust-lang/rust-clippy) and
[rustfmt](https://github.com/rust-lang/rustfmt) used to enforce these
guidelines.

Clippy is configured to be strict and may raise false positives. If Clippy
throws a false error, add a `#[allow(clippy::lint_name)]` and a comment
explaining why the lint should be disabled for the line or code block.

## Release

The release process is mostly automated by GitHub Actions. Any pushed Git tag
which follows a numerical [SemVer](https://semver.org/) scheme, such as
`2.0.11`, will trigger the CI release workflow. Unfortunately, GitHub Actions
does not yet provide M1 macOS virtual machines. For more information, see the
assoicated
[GitHub issue](https://github.com/actions/virtual-environments/issues/2187). As
a result, by the following steps must be manually executed for each release.

1. Trigger a GitHub Actions release with a Git tag push.
2. Download the project on a M1 macOS platform.
3. Compile the release binary.
4. Zip the binary and release files. See the
   [release workflow](<[github/workflows/release.yaml](https://github.com/wolfgangwazzlestrauss/sampitor/blob/master/.github/workflows/release.yaml)>)
   for details.
5. Edit the GitHub release by uploading the Zip archive.

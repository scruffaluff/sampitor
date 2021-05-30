# How to Contribute to Sampitor

Thank you for your interest in contributing to Sampitor. This guide will assist
you in setting up a development environment, understanding the project tooling,
and learning the coding guidelines.

## Dependencies

Sampitor is written in the [Rust](https://www.rust-lang.org/tools/install)
programming language and uses
[Cargo-Make](https://github.com/sagiegurari/cargo-make#installation) as a task
runner.

On Linux platforms, the ALSA development libraries may need to be installed.
Check the table for required packages.

| Distribution | Package        |
| :----------- | :------------- |
| Alpine       | alsa-lib-dev   |
| Arch         | alsa-lib       |
| Debian       | libasound2-dev |
| Fedora       | alsa-lib-devel |

On macOS and Windows platforms, the native audio host development libraries are
pre-installed. Sampitor uses CPAL for talking to audio hosts. For working with
other audio hosts, see their [GitHub page](https://github.com/RustAudio/cpal).

## Getting Started

To download the project and run the test suite, execute

```console
git clone https://github.com/wolfgangwazzlestrauss/sampitor
cd sampitor
cargo make all
```

## Code Conventions

Sampitor follows the official
[Rust Style Guide](https://github.com/rust-dev-tools/fmt-rfcs/blob/master/guide/guide.md)
and the official
[Rust API Guidelines](https://rust-lang.github.io/api-guidelines/).
[Clippy](https://github.com/rust-lang/rust-clippy) is used to enforce these
guidelines.

## Releases

The release process is mostly automated by GitHub Actions. Any pushed Git tag
which follows a numerical [SemVer](https://semver.org/) scheme, such as
`2.0.11`, will trigger the CI release workflow. Unfortunately, GitHub Actions
does not yet provide M1 macOS virtual machines. For more information, see the
assoicated
[GitHub issue](https://github.com/actions/virtual-environments/issues/2187). As
a result, the M1 macOS binary archive must be manually uploaded for each release
by the following steps.

1. Trigger a GitHub Actions release with a Git tag push.
2. Download the project on a M1 macOS platform.
3. Compile the release binary.
4. Zip the binary and release files. See the
   [release workflow](<[github/workflows/release.yaml](https://github.com/wolfgangwazzlestrauss/sampitor/blob/master/.github/workflows/release.yaml)>)
   for details.
5. Edit the GitHub release by uploading the Zip archive.

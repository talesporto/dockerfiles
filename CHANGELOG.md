# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]

## [v0.2.0-basic-interpreter]

Reached [milestone v0.2.0](https://github.com/ngeor/dockerfiles/milestone/3) of
basic interpreter, which means that the interpreter can run a
[program that prints out the Fibonacci numbers](https://github.com/ngeor/dockerfiles/blob/v0.2.0-basic-interpreter/basic/basic-interpreter-rust/fixtures/FIB.BAS).

## [v0.5.0-basic]

### Changed

Renamed `gwbasic` image and folder to `basic`, as it supports not only GW-Basic
but also QBasic.

## [v0.1.0-dockerv]

### Added

- Added `dockerv` command to make life easier working with Docker Toolbox on
  Windows.

## [v0.4.0-gwbasic]

### Added

- Full support for QBasic, including performance measurement
- Improved performance of DOSBox by configuring CPU and MIDI settings

### Changed

- Environment variables that control the launcher are prefixed with `BLR_`
- Whitelisting only specific environment variables for the BAS file, because too
  many environment variables cause the program to fail.
- Launcher script reads the `REDIRECT_` environment variables that mod_rewrite
  sets.
- Merged all QBasic REST API programs in one QBasic program for maintainability.

## [v0.3.0-gwbasic]

### Changed

- Rewrote the launcher in Rust, which brings a small performance benefit.

## [v0.2.0-gwbasic]

### Changed

- Externalized the configuration of Apache into `.htaccess` file. Changed the
  way the cgi-bin script gets the BAS file to run (was environment variable, now
  it's in the query string).

## [v0.1.0-gwbasic]

### Added

- New Ruby launcher script, supporting GWBasic and QBasic
- work in progress with Rust launcher
- Performance experiments for shell launcher and ruby launcher
- Tool for running performance tests
- Added `Makefile` for running common development tasks

### Changed

- BAS files must now have their traditional CRLF line endings
- Changed folder structure for Docker images. `/basic/bin` for the
  `GWBASIC.EXE`, `/basic/src` for the BAS files, `/usr/local/bin` for the
  launcher script.
- BAS files are expected to read input at a file specified by the environment
  variable `STDIN`. For example:
  `OPEN ENVIRON$("STDIN") FOR INPUT ACCESS READ AS #1`.
- The Apache configuration **must** use `mod_rewrite` to point to the launcher
  script and specify the BAS file with an environment variable named `BAS` in
  the rewrite rule. For example:
  `RewriteRule "^/api/todo$" "/cgi-bin/run-dos-box.rb" [E=BAS:LIST.BAS,PT,L]`
- No longer backing `GWBASIC.EXE` inside the image. It needs to be provided via
  a volume (`/basic/bin`).

### Removed

- Dropped support for BAS scripts with a shebang line
- Removed batch and shell launchers, keeping only ruby launcher

## [2020-02-22]

### Added

- Build script,
  [automatic publishing of descriptions to Docker Hub](https://ngeor.com/2019/12/26/docker-hub-automation.html)
- New GW-Basic dockerfile, as appeared in my
  [related blogpost](https://ngeor.com/2020/02/22/gwbasic-in-docker.html)

[unreleased]: https://github.com/ngeor/dockerfiles/compare/v0.2.0-basic-interpreter...HEAD
[v0.2.0-basic-interpreter]: https://github.com/ngeor/dockerfiles/compare/v0.5.0-basic...v0.2.0-basic-interpreter
[v0.5.0-basic]: https://github.com/ngeor/dockerfiles/compare/v0.1.0-dockerv...v0.5.0-basic
[v0.1.0-dockerv]: https://github.com/ngeor/dockerfiles/compare/v0.4.0-gwbasic...v0.1.0-dockerv
[v0.4.0-gwbasic]: https://github.com/ngeor/dockerfiles/compare/v0.3.0-gwbasic...v0.4.0-gwbasic
[v0.3.0-gwbasic]: https://github.com/ngeor/dockerfiles/compare/v0.2.0-gwbasic...v0.3.0-gwbasic
[v0.2.0-gwbasic]: https://github.com/ngeor/dockerfiles/compare/v0.1.0-gwbasic...v0.2.0-gwbasic
[v0.1.0-gwbasic]: https://github.com/ngeor/dockerfiles/compare/2020-02-22...v0.1.0-gwbasic
[2020-02-22]: https://github.com/ngeor/dockerfiles/releases/tag/2020-02-22

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]

### Added

- `gwbasic`: New Ruby launcher script, supporting GWBasic and QBasic
- `gwbasic`: work in progress with Rust launcher
- `gwbasic`: Performance experiments for shell launcher and ruby launcher
- `gwbasic`: Tool for running performance tests
- `gwbasic`: Added `Makefile` for running common development tasks

### Changed

- `gwbasic`: BAS files must now have their traditional CRLF line endings
- `gwbasic`: Changed folder structure for Docker images. `/basic/bin` for the
  `GWBASIC.EXE`, `/basic/src` for the BAS files, `/usr/local/bin` for the
  launcher script.
- `gwbasic`: BAS files are expected to read input at a file specified by the
  environment variable `STDIN`. For example:
  `OPEN ENVIRON$("STDIN") FOR INPUT ACCESS READ AS #1`.
- `gwbasic`: The Apache configuration **must** use `mod_rewrite` to point to the
  launcher script and specify the BAS file with an environment variable named
  `BAS` in the rewrite rule. For example:
  `RewriteRule "^/api/todo$" "/cgi-bin/run-dos-box.rb" [E=BAS:LIST.BAS,PT,L]`

### Removed

- `gwbasic`: Dropped support for BAS scripts with a shebang line

## [2020-02-22]

### Added

- Build script,
  [automatic publishing of descriptions to Docker Hub](https://ngeor.com/2019/12/26/docker-hub-automation.html)
- New GW-Basic dockerfile, as appeared in my
  [related blogpost](https://ngeor.com/2020/02/22/gwbasic-in-docker.html)

[unreleased]: https://github.com/ngeor/dockerfiles/compare/2020-02-22...HEAD
[2020-02-22]: https://github.com/ngeor/dockerfiles/releases/tag/2020-02-22

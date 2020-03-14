# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]

## [v0.2.0-gwbasic]

### Changed

- `gwbasic`: Externalized the configuration of Apache into `.htaccess` file.
  Changed the way the cgi-bin script gets the BAS file to run (was environment
  variable, now it's in the query string).

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

[unreleased]: https://github.com/ngeor/dockerfiles/compare/v0.2.0-gwbasic...HEAD
[v0.2.0-gwbasic]: https://github.com/ngeor/dockerfiles/compare/v0.1.0-gwbasic...v0.2.0-gwbasic
[v0.1.0-gwbasic]: https://github.com/ngeor/dockerfiles/compare/2020-02-22...v0.1.0-gwbasic
[2020-02-22]: https://github.com/ngeor/dockerfiles/releases/tag/2020-02-22

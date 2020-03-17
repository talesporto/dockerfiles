# dockerv

> dockerv is a wrapper for docker with better volume support

## Prerequisites

- Windows 10 Home
- Docker Toolbox for Windows
- `docker.exe` is in the `PATH`

## Prerequisites for (Git) Bash

- `MSYS_NO_PATHCONV=1` (see also
  [related post](https://ngeor.com/2019/12/25/windows-docker-and-paths.html))

## Overview

`dockerv.exe` is a lightweight wrapper that calls `docker.exe` passing all
arguments to it unmodified, except the `-v` or `--volumes` arguments. When it
encounters a volume in the form of `host:guest`, it ensures that:

- the host path is absolute
- the host path is a Unix path as expected by the Docker daemon running inside
  Docker Toolbox's VirtualBox

All other arguments are unmodified, meaning you can use it as a drop-in
replacement for `docker.exe`.

## Examples

### Windows path to Unix path

`dockerv run -v C:\Users\me\Documents:/docs`

is translated to:

`docker run -v /c/Users/me/Documents:/docs`

### Using `${PWD}` in PowerShell

Running in directory `C:\Users\me\code`:

`dockerv run -v ${PWD}\src:/usr/src`

is translated to:

`docker run -v /c/Users/me/code/src:/usr/src`

### Using relative paths in PowerShell

Running in directory `C:\Users\me\tests`:

`dockerv run -v ..\foo:/foo`

is translated to:

`docker run -v /c/Users/me/foo:/foo`

### Using relative paths in (Git) Bash

Running in directory `/c/Users/me/files`:

`dockerv run -v .:/files`

is translated to:

`dockerv run -v /c/Users/me/files:/files`

## Installing

The latest binary is in the releases. To build from source, use Rust and run
`cargo install -path .`.

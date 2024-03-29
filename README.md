# Nael

> [!WARNING]  
> Nael is not yet considered stable and is subject to breaking changes. If you're using it in CI or scripts you should pin to a specific version to avoid breakage. This includes both the `nael_core` crate and the `nael` binary.

*You're looking at the README for `nael` the command-line tool. If you're looking for the`nael_core` library, please go [here](./crates/core/).*

A [Dalamud](https://github.com/goatcorp/Dalamud) version manager that makes working with different releases (branches) of Dalamud simple, inspired by [nvm](https://github.com/nvm-sh/nvm).

## Features

- **Quick installs and updates** from release distribution sources.
- **Instant switching** between multiple locally-installed branches.
- **Update checks** from a remote source to only run an update when out of date.
- **Compliant** with all major operating system storage standards.
- **No configuration** files, everything is completely filesystem based.
- **Implemented safely** with a project-agnostic crate.
- **Cross-platform** and able to run on most major operating systems.

Check the [examples](#examples) section to see some examples of usage. 

## Installing

> [!IMPORTANT]  
> **WINDOWS USERS** will have to enable [Developer Mode](https://learn.microsoft.com/en-us/windows/apps/get-started/developer-mode-features-and-debugging) in order to use nael outside of an administrator context due to Windows preventing the creation of symlinks without it enabled.

### Cargo (best platform support; recommended)

The best way to install is by using Cargo to fetch it from crates.io and compile it for your machine. For some architectures (e.g. aarch64) this is currently the only way to install nael.

Once you have [Cargo setup locally](https://www.rust-lang.org/tools/install), simply run the following to download and compile for your system:

```
cargo install nael
```

### Prebuilt binaries

Pre-built binaries are available for all targets that `cargo-dist` can support and are built using the latest versions of Rust. To download one for your system, go to the [GitHub releases page](https://github.com/Blooym/nael/releases/latest) and download the right release asset for your architecture and operating system add place it somewhere in your system `$PATH`.

### Other package managers

Nael is not available from any package manager at this time.

## Setting up DALAMUD_HOME with nael

> [!NOTE]  
> Testing is required here. PRs are welcome to help improve this section!

`DALAMUD_HOME` is the [community-accepted](https://github.com/goatcorp/SamplePlugin/blob/c1dacec1e1f56ac798a9ffd5703f6101b8aa054e/SamplePlugin/Dalamud.Plugin.Bootstrap.targets) environment variable for setting a custom Dalamud path.

Setting an environment variable depends on the shell you're using. For most POSIX-compliant shells you should be able to just add the following to your shell configuration:

```sh
export DALAMUD_HOME=$(nael symlink-path)
```

If you cannot, or do not want to, call nael to get the path when setting the environment variable you can run `nael symlink-path` and place the command output in your environment configuration instead. The symlink returned from this command will always point to the active version if one is set *(note: if no active version is set, this symlink will not exist or will lead to a dead path.)*.

### Integrating with MSBuild

Add the following to your `.csproj` or `.targets` file, replacing any existing definitions of `DalamudLibPath` property.

```xml
<PropertyGroup>
  <DalamudLibPath Condition="$([MSBuild]::IsOSPlatform('Windows'))">$(appdata)\XIVLauncher\addon\Hooks\dev\</DalamudLibPath>
  <DalamudLibPath Condition="$([MSBuild]::IsOSPlatform('Linux'))">$(HOME)/.xlcore/dalamud/Hooks/dev/</DalamudLibPath>
  <DalamudLibPath Condition="$([MSBuild]::IsOSPlatform('OSX'))">$(HOME)/Library/Application Support/XIV on Mac/dalamud/Hooks/dev/</DalamudLibPath>
  <DalamudLibPath Condition="$(DALAMUD_HOME) != ''">$(DALAMUD_HOME)/</DalamudLibPath>
</PropertyGroup>
```

You will now be able to use the `DALAMUD_HOME` environment variable to override the default DalamudLibPath and use nael to manage your Dalamud version instead - if `DALAMUD_HOME` isn't set the per-platform paths will be used instead.

## Examples

**Show all command line options:**
```
> nael help
A Dalamud version manager that makes working with different releases of Dalamud simple.

Usage: nael <COMMAND>

Commands:
  install       Install a Dalamud release from the specified branch
  update        Update a local branch to the latest version
  remove        Remove a branch from this system
  list          List all installed branches
  use           Switch the currently active branch
  active        Get information about the active branch
  symlink-path  Get the path to the symlink that always points to the active branch
  info          Show information about the specified branch
  help          Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

**Installing a Dalamud release from the `release` branch:**
```
> nael install release
Successfully installed branch 'release' with version '9.0.0.17'.
```

**Updating the `release` branch**
```
> nael update release
Branch is already up to date.
```

**Setting the active branch to `release`:**
```
> nael use release
Successfully set branch 'release' as active.
```

**Listing all installed branches**
```
> nael list
Installed branches:
 - stg
 - v9
 * release
```

**Getting information about the active version:**
```
> nael active
release
> nael active --format path
/home/example/.local/share/nael/dalamud-branches/release
```

**Getting local information about the `release` branch:**   
Optionally, see remote information instead with the `--remote` flag.
```
> nael info release
Local version information for branch release:
- Version: 9.0.0.17
- Git Sha: unknown
- Revision: unknown
- Key: N/A
- Supported GameVer: 2023.03.24.0000.0000
- Runtime version: 7.0.0
- Runtime required: true
```

## License

This project is dual-licensed under both the MIT License and the Apache License (Version 2.0). See [LICENSE-MIT](./LICENSE-MIT) and [LICENSE-APACHE](./LICENSE-APACHE) for more details.

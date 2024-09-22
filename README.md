# splendor_scripts
Scripting utilities to facilitate setup and launch of Splendor AI competition on client devices

Currently tested on Windows and Linux systems!
## Prerequisites

This script needs 
- Rust (cargo) >= 1.50
- Python >= 3.9

## Coming Soon

- [x] Add support for creating new projects
- [x] Add support for running projects against each other
- [x] Add support for saving configurations and editing them
- [ ] Improve output for running projects
- [x] Reintroduce visualization for games
- [x] Improve build times by removing extraneous files and dependencies
- [x] Add support for updating projects / checking for the latest version of stourney
- [ ] Add replay logging and visualising

## Creating a new project

To create a new project, run the following command:

```bash
cargo install stourney
stourney new <project_name>
```

Which will initialize a project in the given directory

## Configuring a project

To show the current configuration of a project, run the following command:

```bash
stourney config show
```

To edit the configuration of a project, run the following command:

```bash
stourney config edit
```

## Running projects 

To run projects against each other, run the following command:
be sure to have set up the the competitors using the `config` commands!

```bash
stourney run
```

## Disclaimer

This repository is not affiliated, associated, authorized, endorsed by, or in any way officially connected with Splendor, Space Cowboys or any of its subsidiaries or its affiliates. The link to their website can be found [here](https://www.spacecowboys.fr/splendor-en).

The name Splendor as well as related names, marks, emblems and images are registered trademarks of their respective owners.

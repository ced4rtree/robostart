# Robostart: A Standalone CLI WPILib Project Creator

Robostart is a CLI utility designed to create a
[WPILib](https://github.com/wpilibsuite/allwpilib) project for teams competing
in the FIRST Robotics Competition.

WPILib currently has a standalone robot project creator, and one built into
their distribution of VSCode. However, the standalone project creator is slated
to be removed for the 2027 season. Thus, Robostart was born.

## Usage

Robostart can be invoked via the command line as such:

```bash
robostart
```

Spawning `robostart` with no arguments will launch an interactive prompt
dialogue to create the project you would like.

You can also inform Robostart of many of these dialogue options with flags.

```bash
robostart -h
```

Output:

```
Usage: robostart [OPTIONS]

Options:
  -l, --language <LANGUAGE>
          Language to initialize [possible values: java, cpp]
  -p, --project-type <PROJECT_TYPE>
          Type of project to initalize [possible values: example, template]
  -w, --wpilib-version <WPILIB_VERSION>
          What version to download
  -o, --output-prefix <OUTPUT_PREFIX>
          The parent directory for the new project
  -n, --name <NAME>
          Name of the new project
  -t, --team-number <TEAM_NUMBER>
          Your team number
  -h, --help
          Print help
  -V, --version
          Print version
```

## Building

Robostart is built with rust, and so the use of `cargo` to build the project is
fairly simple.

```bash
cargo build
```

Alternatively, one can run a debug version of Robostart with cargo as well.

```bash
cargo run -- # whatever flags you want to pass go here
```

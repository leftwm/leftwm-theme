# leftwm-theme
A tool to help manage themes for LeftWM

Themes are defined by theme.toml file. Theme file consists of tow main components: global values which includes theme name, it's dependencies and optionally environment variables. The second component is task. Each task performs the setup and teardown for each of the dependencies specified. Below is a minimal example of `theme.toml`:

```toml
[global]
name = "mytheme"
dependencies = ["polybar", "dunst"]  # Used to check if required packages are installed in the system

# task.program_name.up sets up the program. The following section will spawn `polybar mybar`
[task.polybar.up]
args = ["mybar"]

# defaults to pkill program name if no down script is provided.
[task.polybar.down]
command = "killall"
args = ["-q", "polybar"]

# if program_name in task.program_name.{up,down} does not match any dependency listed, a command value must be provided.
# paths are relative to the current directory of theme.toml file.
[task.notification.up]
command = "/usr/bin/dunst"
args = ["--config", "configs/dunstrc"]
```

## Installation


## Usage

To install a theme, simply do:
```bash
leftwm-theme install theme_url
```
where `theme_url` is a git url with `theme.toml` file in the root directory.
If you wish to install local theme, use _--path_ flag instead.

Themes can be applied via:
```bash
leftwm-theme apply theme_name
```

To undo the previous application of the theme, run:
```bash
leftwm-theme undo
```

As you have guessed, uninstalling a theme is just
```bash
leftwm-theme uninstall theme_name
```

Status of currently active theme as well as installed theme can be viewed by:
```bash
leftwm-theme status
```


## Roadmap

- [ ] Cli application with argument and basic theme.toml parsing 
- [ ] Validation of theme.toml
- [ ] Implement tasks(up, down) along with task dependencies.
- [ ] Git integration
- [ ] Provision for name aliases for dependencies in different distros.


## License

This project is licensed under the BSD 3-clause license.

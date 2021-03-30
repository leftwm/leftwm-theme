<div align="center">
  <h1><code>leftwm-theme</code></h1>

  <p>
    <strong>A theme manager for
    <a href="https://github.com/leftwm/leftwm/">LeftWM</a></strong>
  </p>
  <p>
    <a href="https://github.com/leftwm/leftwm-theme/actions?query=workflow%3ACI"><img src="https://github.com/leftwm/leftwm-theme/workflows/CI/badge.svg" alt="build status" /></a>
    <!--<img src="https://img.shields.io/badge/rustc-1.37+-green.svg" alt="min rustc" />-->
    <!--<a href="https://docs.rs/leftwm/0.2.6/leftwm/"><img src="https://docs.rs/leftwm-theme/badge.svg" alt="Documentation" /></a>-->
  </p>
</div>



## Installation
Currently, LeftWM-theme is only available by Git and AUR. 

To install by AUR (Arch-based distributions only), use package `leftwm-theme-git`.

To install manually, clone this repository:
```bash
git clone https://github.com/leftwm/leftwm-theme
```
Then enter the repository: 
```
cd leftwm-theme
```
Then build with Cargo:
```bash
cargo build --release
```
You can then install LeftWM-theme:
```bash
# for production installations (does not update when recompiled)
sudo install -s -Dm755 ./target/release/leftwm-theme -t /usr/bin
#-- or --
# for developer installations (updates when recompiled)
sudo ln -s "$(pwd)"/target/release/leftwm-theme /usr/bin/leftwm-theme
```

## Usage
### First use
The first time you start up LeftWM-theme, it will generate a file called `themes.toml` in your `~/.config/leftwm/` folder from the themes located in the [Community Themes](https://github.com/leftwm/leftwm-community-themes) repo. To do so, run:
```bash
leftwm-theme update
```
<!---
[](To populate your local repository with your previously installed themes, use the following:
```bash
leftwm-theme autofind
```
**Note: as of 02/26/2021 autofind is not yet fully implemented**)
 -->
### Install a theme
LeftWM-theme differentiates between _installing_ a theme and _applying_ a theme. Installing a theme is akin to downloading it; behind the scenes LeftWM-theme runs `git clone {theme}`. No dependency checks are performed at installation time, but instead at application time. To install a theme, for example the fabulous Orange Forest theme, run (quotation marks needed for names with spaces):
```bash
leftwm-theme install "Orange Forest"
```
**Note: LeftWM-theme is CaSe SeNsItIvE, so be careful!**

### Apply a theme
LeftWM-theme will check for dependencies, LeftWM-version, and the like during the application process.
Now that you've installed Orange Forest (or whatever theme you like), to set it as your current theme, run:
```bash
leftwm-theme apply "Orange Forest"
```
**Note: LeftWM should automatically restart with the new theme**

### List installed themes
To list all installed themes that LeftWM-theme knows about, run:
```bash
leftwm-theme list
```

### See current theme
Although multiple commands list the installed theme, using the following can provide additional context:
```bash
leftwm-theme status
```

### Update theme list
To update your copy of the themes, use the following:
```bash
leftwm-theme update
```
**Note: this does not also update the themes, just the repository listings! To update themes see upgrade**

### Updating themes
To update themes, use the following:
```bash
leftwm-theme upgrade
```
**Note: this command also updates repositories**

### Adding a repository
Leftwm-theme allows multiple `known.toml` repositories to be used. To add another repository, it must have a `known.toml` file which you can add to `themes.toml` in your LeftWM config folder.   

**Note: It is wise to backup your `themes.toml` file PRIOR to adding a new repostitory**

To add a repository, add the following to the BOTTOM of your `themes.toml` file, located at `~/.config/leftwm/themes.toml`:
```toml
# To add additional repos, you MUST specify a url, a UNIQUE ALPHANUMERIC name, and an empty array of themes
# e.g.:
# [[repos]]
# url = "https://raw.githubusercontant.com/mautamu/leftwm-community-themes/master/known.toml"
# name = "mautamu"
# themes = []
[[repos]]
url = ""
name = ""
themes = []
```
**Note: be sure that the url points to a file called known.toml, such as https://raw.githubusercontent.com/leftwm/leftwm-community-themes/master/known.toml**

Then fill in the url with the url of that repo and add a descriptive name that consists of only letters and numbers [A-z0-9]. To load themes from the repository, use the following:
```bash
leftwm-theme -vvv update
``` 
**Note: the -vvv flag is not necessary, but will provide additional output in case your new repo goes wrong**
## Troubleshooting
### Themes.toml is nearly empty, and/or LeftWM won't update my themes:
Try removing themes.toml and running the `update` command, add any repositories that were removed, and then run `autofind` to repopulate your installed themes.
### I can't get a theme to install
Double check your name. Although `update` may say `mautam/theme`, you just need to type `theme`, not `mautam/theme`. Pay attention to capital letters and spelling.


## Roadmap:
### Version 0.1.0
- [x] Allow users to install themes
- [x] Allow users to remove themes
- [x] Allow a theme to be applied as current
	- [x] Check dependencies for a theme
	- [x] Allow dependency override with -n
  	- [ ] Offer suggestions for dependency installation
	- [ ] Check whether a theme's `theme.toml` file is valid
- [x] Allow themes to specify compatible LeftWM versions
- [ ] Find themes located in ~/.config/leftwm/themes/ automatically
- [x] Allow users to add more theme repositories
	- [x] Allow users to choose from which repository to install themes
- [x] Allow users to create new themes 
	- [x] Provide basic themes for users to fork into their own
	- [ ] Generate appropriate `known.toml` pull requests and `theme.toml` files
	- [ ] Make sure themes don't include `/` or other OS-specific marks. **Partially complete**
- [x] Allow users to update their repository theme lists with `update` as in apt-get form
- [x] Allow users to update their themes with `upgrade` command, as in apt-get form
	- [x] Allow users to skip repo update
	- [ ] Perform dependency checks prior to updating the current theme
- [x] Allow users to search for themes by name
### Version 0.2.0
- [ ] Extend `theme.toml` to allow for up/down specifications within `theme.toml`
- [ ] Integrate `themes.toml` and `known.toml` better
- [ ] Reduce the number of dependencies
	- [ ] Replace Reqwest with a crate with fewer dependencies
	- [ ] Examine other areas of overlapping features
- [ ] Provision for name aliases for dependencies in different distros
- [ ] Improve documentation
- [ ] Better, more consistent error handling
- [x] Remove `nightly` Rust requirement by replacing `?` on Options
- [ ] Add a testing suite





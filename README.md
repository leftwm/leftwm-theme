# leftwm-theme
A tool to help manage themes for LeftWM



## Adding more repositories to your `themes.toml` file
Leftwm-theme allows multiple `known.toml` repositories to be used. To add another repository, it must have a `known.toml` file which you can add to `themes.toml` in your LeftWM config folder.   

```toml
# To add additional repos, you MUST specify a url, a UNIQUE name, and an empty array of themes
# e.g.:
# [[repos]]
# url = "https://raw.githubusercontant.com/mautamu/leftwm-community-themes/master/known.toml"
# name = "mautamu"
# themes = []
```

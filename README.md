
## Installation (has 2 steps)

Install the crate binaries using Cargo:

```sh
$ cargo install --git https://github.com/allovelle/macos-xbar-commands
#    Installs: ~/.cargo/bin/macos-resize-dock
#    Installs: ~/.cargo/bin/macos-toggle-dark-mode
#    Installs: ~/.cargo/bin/macos-toggle-dock
#    Installs: ~/.cargo/bin/macos-toggle-natural-scrolling
```

Each of the Xbar utilities has their own install/uninstall process unique to the
binary. They each provide `install` and `uninstall` commands that refer to Xbar
and not Cargo. This means that uninstalling the crate using Cargo does not
remove it for Xbar as well as the reverse. Additionally, only the desired Xbar
utilities can be enabled using selective installation.

Install the Xbar utilities:

```sh
$ macos-resize-dock install
$ macos-toggle-dark-mode install
$ macos-toggle-dock install
$ macos-toggle-natural-scrolling install
```

## Uninstallation (has 2 steps)

Uninstalling the crate with Cargo does not remove the associated Xbar scripts.
First uninstall the Xbar scripts and then proceed with the Cargo binary removal.

Uninstall the Xbar utilities:

```sh
$ macos-resize-dock uninstall
$ macos-toggle-dark-mode uninstall
$ macos-toggle-dock uninstall
$ macos-toggle-natural-scrolling uninstall
```

Uninstall the crate binaries using Cargo:

```sh
$ cargo uninstall macos-xbar-commands
#    Uninstalls: ~/.cargo/bin/macos-resize-dock
#    Uninstalls: ~/.cargo/bin/macos-toggle-dark-mode
#    Uninstalls: ~/.cargo/bin/macos-toggle-dock
#    Uninstalls: ~/.cargo/bin/macos-toggle-natural-scrolling
```


# macos-toggle-dock
> CLI utility to toggle dock hiding along with an installable Xbar plugin.

> Toggling the dock only works on MacOS ⚠️

![Screenshot](Screenshot.png)

Installation:

```bash
$ cargo install --git https://github.com/allovelle/macos-toggle-dock
```

For convenience, use this [Xbar](https://xbarapp.com) script to put a clickable icon in the menu
bar:

```bash
#!/usr/bin/env bash

export PATH="$HOME/.cargo/bin:$PATH"

echo ':peach:'  # More Emojis: https://gist.github.com/rxaviers/7360908
echo '---'
echo "Toggle Dock | shell='$(which macos-toggle-dock)'"
```

Put the Xbar script in `~/Library/Application\ Support/xbar/plugins` and don't
forget to make it executable with `chmod +x toggle-hide-dock.sh`.

For convenience, this crate adds a CLI command to do this automatically:

```bash
$ macos-toggle-dock install
$ macos-toggle-dock uninstall
```

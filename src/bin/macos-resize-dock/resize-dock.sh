#!/usr/bin/env bash
# <xbar.title>Slightly Resize MacOS Dock</xbar.title>
# <xbar.desc>Switch between small, medium, and large dock sizes</xbar.desc>
# <xbar.author>Allovelle (she/they) ⭐️</xbar.author>
# <xbar.author.github>allovelle</xbar.author.github>
# <xbar.version>0.1.0</xbar.version>
# <xbar.dependencies>bash</xbar.dependencies>
# <xbar.image>https://github.com/allovelle/macos-xbar-commands/raw/main/src/bin/toggle-dock/Screenshot.png</xbar.image>


# TODO: Choose between these sizes in the plugin browser & menu
# TODO: defaults read com.apple.dock tilesize    <--- was 48.
# TODO: ^^^^ this can be a slider?
# Env var versions of vars are all strings:
# echo "Hello, ${VAR_NAME}"
# Sidecar json (better for Rust): types are the expected ones:
# THE_PLUGIN_NAME.5s.sh.vars.json
# {
# 	"VAR_FILE": "./001-THE_PLUGIN_NAME.5s.sh",
# 	"VAR_LINES": 15       <<<<<
# }


# <xbar.var>string(VAR_NAME="Mat Ryer"): Your name.</xbar.var>
# <xbar.var>number(VAR_COUNTER=1): A counter.</xbar.var>
# <xbar.var>boolean(VAR_VERBOSE=true): Verbose or not?</xbar.var>
# <xbar.var>select(VAR_STYLE="normal"): Which style to use. [small, normal,


export PATH="$HOME/.cargo/bin:$PATH"

echo ':cherries:'  # More Emojis: https://gist.github.com/rxaviers/7360908
echo '---'
echo "Resize Dock | shell='$(which macos-resize-dock)'"

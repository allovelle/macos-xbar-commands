#!/usr/bin/env bash
# <xbar.title>Toggle MacOS Dark Mode</xbar.title>
# <xbar.desc>Switch between Light, Dark, and Auto appearance modes.</xbar.desc>
# <xbar.author>Allovelle (she/they) ⭐️</xbar.author>
# <xbar.author.github>allovelle</xbar.author.github>
# <xbar.version>0.1.0</xbar.version>
# <xbar.dependencies>bash</xbar.dependencies>
# <xbar.image>https://github.com/allovelle/macos-xbar-commands/raw/main/src/bin/toggle-dock/Screenshot.png</xbar.image>

export PATH="$HOME/.cargo/bin:$PATH"

# Detect current mode
IS_DARK=$(defaults read -g AppleInterfaceStyle 2>/dev/null)
AUTO=$(defaults read -g AppleInterfaceStyleSwitchesAutomatically 2>/dev/null)

# More Emojis: https://gist.github.com/rxaviers/7360908
if [[ "$AUTO" == "1" ]]; then
    ICON="🌓"
elif [[ "$IS_DARK" == "Dark" ]]; then
    ICON="🌑"
else
    ICON="🌕"
fi

echo "$ICON"
echo "---"

echo "🌕 Light Mode | shell='$(which macos-toggle-dark-mode) light-mode'"
echo "🌓 Auto Mode | shell='$(which macos-toggle-dark-mode) auto-mode'"
echo "🌑 Dark Mode | shell='$(which macos-toggle-dark-mode) dark-mode'"

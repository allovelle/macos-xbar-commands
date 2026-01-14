#!/usr/bin/env bash
# <xbar.title>Toggle MacOS Dark Mode</xbar.title>
# <xbar.desc>Switch between Light, Dark, and Auto appearance modes.</xbar.desc>
# <xbar.author>Allovelle (she/they) ⭐️</xbar.author>
# <xbar.author.github>allovelle</xbar.author.github>
# <xbar.version>0.1.0</xbar.version>
# <xbar.dependencies>bash</xbar.dependencies>
# <xbar.image>https://github.com/allovelle/macos-xbar-commands/raw/main/src/bin/macos-toggle-dock/Screenshot.png</xbar.image>

export PATH="$HOME/.cargo/bin:$PATH"

# More Emojis: https://gist.github.com/rxaviers/7360908
if [[ "$XBARDarkMode" == "true" ]]; then
    # 🌑
    echo ":new_moon:"
else
    # 🌕
    echo ":full_moon:"
fi

echo "---"

echo "🌕 Light Mode | shell='$(which macos-toggle-dark-mode)' param1=light-mode terminal=false refresh=true"
echo "🌑 Dark Mode | shell='$(which macos-toggle-dark-mode)' param1=dark-mode terminal=false refresh=true"

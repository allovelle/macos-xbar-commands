#!/bin/bash
# <xbar.title>Toggle MacOS Dark Mode</xbar.title>
# <xbar.desc>Switch between Light, Dark, and Auto appearance modes.</xbar.desc>
# <xbar.author>Allovelle (she/they) ⭐️</xbar.author>
# <xbar.author.github>allovelle</xbar.author.github>
# <xbar.version>0.1.0</xbar.version>
# <xbar.dependencies>bash</xbar.dependencies>
# <xbar.image>https://github.com/allovelle/macos-xbar-commands/raw/main/src/bin/toggle-dock/Screenshot.png</xbar.image>

# Detect current mode
IS_DARK=$(defaults read -g AppleInterfaceStyle 2>/dev/null)
AUTO=$(defaults read -g AppleInterfaceStyleSwitchesAutomatically 2>/dev/null)

if [[ "$AUTO" == "1" ]]; then
    ICON="🌓"
elif [[ "$IS_DARK" == "Dark" ]]; then
    ICON="🌑"
else
    ICON="🌕"
fi

echo "$ICON"
echo "---"

# Light Mode
echo "Light Mode | shell='osascript -e \"tell application \\\"System Events\\\" to tell appearance preferences to set dark mode to false\"' terminal=true"

# Dark Mode
echo "Dark Mode | bash='osascript -e \"tell application \\\"System Events\\\" to tell appearance preferences to set dark mode to true\"' terminal=false"

# Auto Mode
echo "Auto Mode | bash='bash -c \"defaults write -g AppleInterfaceStyleSwitchesAutomatically -bool true; killall SystemUIServer\"' terminal=false"

# Disable Auto
echo "Disable Auto | bash='bash -c \"defaults write -g AppleInterfaceStyleSwitchesAutomatically -bool false; killall SystemUIServer\"' terminal=false"

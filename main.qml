import QtQuick
import Quickshell
import qs.components.misc

QtObject {
    id: root
    
    // Register the shortcut for the plugin
    property CustomShortcut shortcut: CustomShortcut {
        name: "wallpaperSelector"
        key: "Meta+Shift+W"
        description: "Open Wallpaper Selector"
        onPressed: {
            Quickshell.execDetached([Quickshell.env("HOME") + "/.local/bin/caelestia-kde-plugins-wallpaper-selector"]);
        }
    }
}

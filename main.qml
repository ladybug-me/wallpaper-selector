pragma ComponentBehavior: Bound

import QtQuick
import Quickshell
import qs.services.api
import qs.services
import "qml"

import "qml" as PluginStyle
import "qml/wallpaper" as WallpaperQml

Scope {
    id: root

    property bool showing: false
    property string activeScreen: ""

    property int settingsOpenCount: 0
    signal closeSettingsRequested()

    function registerShortcut() {
        CaelestiaApi.shortcuts.register("wallpaper_selector", "Toggle Wallpaper Selector", Config.shortcut, () => {
            if (!root.showing) {
                if (typeof CaelestiaApi.windows.kwin !== "undefined" && CaelestiaApi.windows.kwin) {
                    root.activeScreen = CaelestiaApi.windows.kwin.cursorOutputName() || "";
                } else if (typeof Hypr !== "undefined" && Hypr) {
                    root.activeScreen = Hypr.focusedMonitor ? Hypr.focusedMonitor.name : "";
                }
                root.showing = true;
            } else {
                if (root.settingsOpenCount > 0) {
                    root.closeSettingsRequested()
                } else {
                    root.showing = false;
                }
            }
        });
    }

    Component.onCompleted: registerShortcut()

    Connections {
        target: Config
        function onShortcutChanged() {
            registerShortcut()
        }
    }

    Variants {
        model: Quickshell.screens

        WallpaperQml.WallpaperSelector {
            required property var modelData
            screen: modelData
            showing: root.showing
            isMainScreen: modelData.name === root.activeScreen
            colors: PluginStyle.Colors {}
            
            Connections {
                target: root
                function onShowingChanged() {
                    if (showing !== root.showing) {
                        showing = root.showing;
                    }
                }
            }
            
            onShowingChanged: {
                if (!showing && root.showing) {
                    root.showing = false;
                }
            }
        }
    }
}

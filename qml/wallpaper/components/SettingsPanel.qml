import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Caelestia
import Caelestia.Config as ShellConfig
import qs.components.controls
import qs.components.containers
import qs.components
import qs.modules.nexus.common as Common
import "../.." // For Config

Item {
    id: root
    property var colors
    property bool showing: false
    
    signal closeClicked()
    
    anchors.fill: parent
    visible: showing || opacity > 0
    opacity: showing ? 1 : 0
    enabled: showing
    Behavior on opacity { NumberAnimation { duration: 200; easing.type: Easing.InOutQuad } }
    z: 2000
    focus: showing
    
    onShowingChanged: {
        if (showing) {
            root.forceActiveFocus()
        }
    }

    Keys.onPressed: function(event) {
        if (event.key === Qt.Key_Escape) {
            root.showing = false
            event.accepted = true
        }
    }
    
    function openCaptureDialog(name, currentKey, targetItem) {
        shortcutDialogLoader.active = true
        shortcutDialogLoader.item.shortcutName = name
        shortcutDialogLoader.item.currentKey = currentKey
        shortcutDialogLoader.item.targetItem = targetItem
        shortcutDialogLoader.item.open()
    }
    
    Loader {
        id: shortcutDialogLoader
        active: false
        sourceComponent: Common.KeyCaptureDialog {
            onConfirm: function(name, newKey) {
                Config.saveKey("components.appWallpaper.shortcut", newKey)
            }
            onClear: function(name) {
                Config.saveKey("components.appWallpaper.shortcut", Config.defaultConfig.components.appWallpaper.shortcut)
            }
            onUnblocked: function() {
                shortcutDialogLoader.active = false
            }
        }
    }
    
    // Dim background - captures all clicks, hovers, and wheel events
    MouseArea {
        anchors.fill: parent
        hoverEnabled: true
        preventStealing: true
        onClicked: root.showing = false
        onWheel: function(wheel) {
            wheel.accepted = true
        }
        
        Rectangle {
            anchors.fill: parent
            color: "black"
            opacity: 0.5
        }
    }
    
    // Main Modal
    Rectangle {
        id: modalRect
        width: 600
        height: Math.min(parent.height - 100, 750)
        anchors.centerIn: parent
        color: colors ? Qt.rgba(colors.surfaceContainer.r, colors.surfaceContainer.g, colors.surfaceContainer.b, 0.95) : "transparent"
        radius: ShellConfig.Tokens.rounding.extraLarge
        clip: true

        MouseArea {
            anchors.fill: parent
            onClicked: {}
        }
        
        // Scale and slide animation
        scale: root.showing ? 1.0 : 0.95
        Behavior on scale { NumberAnimation { duration: 300; easing.type: Easing.OutBack } }
        
        // Close Button (Sticky)
        Text {
            z: 11
            anchors.top: parent.top
            anchors.right: parent.right
            anchors.topMargin: ShellConfig.Tokens.padding.largeIncreased
            anchors.rightMargin: ShellConfig.Tokens.padding.largeIncreased
            text: "\uF00D" // nf-fa-times
            font.family: "Symbols Nerd Font"
            font.pixelSize: 24
            color: colors ? (closeMouse.containsMouse ? colors.primary : colors.surfaceText) : "white"
            
            MouseArea {
                id: closeMouse
                anchors.fill: parent
                hoverEnabled: true
                cursorShape: Qt.PointingHandCursor
                onClicked: root.showing = false
            }
            Behavior on color { ColorAnimation { duration: 150 } }
        }
        
        // Content
        VerticalFadeFlickable {
            id: flickable
            anchors.fill: parent
            clip: true
            contentWidth: width
            contentHeight: contentLayout.implicitHeight + ShellConfig.Tokens.padding.largeIncreased * 2
            
            ScrollBar.vertical: ScrollBar {
                active: flickable.contentHeight > flickable.height
                contentItem: Rectangle {
                    implicitWidth: 6
                    radius: width / 2
                    color: colors ? colors.primary : "white"
                    opacity: parent.active ? 0.8 : 0.0
                    Behavior on opacity { NumberAnimation { duration: 200 } }
                }
            }
            
            ColumnLayout {
                id: contentLayout
                width: modalRect.width - ShellConfig.Tokens.padding.largeIncreased * 2
                x: ShellConfig.Tokens.padding.largeIncreased
                y: ShellConfig.Tokens.padding.largeIncreased
                spacing: 0
                
                RowLayout {
                    Layout.fillWidth: true
                    
                    Text {
                        text: "Settings"
                        font: ShellConfig.Tokens.font.title.large
                        color: colors ? colors.surfaceText : "white"
                    }
                    
                    Item { Layout.fillWidth: true }
                    
                    TextButton {
                        text: "RESET"
                        type: TextButton.Filled
                        Layout.rightMargin: 30
                        onClicked: {
                            Config.resetToDefault()
                        }
                    }
                }
                
                // --- General ---
                Common.SectionHeader { text: "General"; first: false }
                
                Common.SelectRow {
                    label: "Display Mode"
                    subtext: "Layout style of the wallpaper browser"
                    first: true
                    last: false
                    property var modeIds: ["slice", "hex", "wall"]
                    active: menuItems[Math.max(0, modeIds.indexOf(Config.displayMode))]
                    
                    menuItems: [
                        MenuItem { text: "Slice" },
                        MenuItem { text: "Hex" },
                        MenuItem { text: "Grid" }
                    ]
                    
                    onSelected: function(item) {
                        var idx = menuItems.indexOf(item)
                        if (idx >= 0) {
                            Config.saveKey("components.appWallpaper.displayMode", modeIds[idx])
                        }
                    }
                }
                
                Common.ShortcutRow {
                    label: "Toggle Shortcut"
                    actionName: "wallpaper_selector"
                    keybind: Config.shortcut
                    isOverridden: Config.shortcut !== Config.defaultConfig.components.appWallpaper.shortcut
                    first: false
                    last: true
                    onAddClicked: function(target) {
                        root.openCaptureDialog("Toggle Wallpaper Selector", Config.shortcut, target)
                    }
                    onKeybindEdited: function(newKeybind) {
                        Config.saveKey("components.appWallpaper.shortcut", newKeybind)
                    }
                    onResetClicked: function() {
                        Config.saveKey("components.appWallpaper.shortcut", Config.defaultConfig.components.appWallpaper.shortcut)
                    }
                }
                
                // --- Slice Settings ---
                Common.SectionHeader { text: "Slice Mode" }
                
                SettingsStepper {
                    label: "Inactive Width"
                    subtext: "Width of background items"
                    first: true
                    actualFrom: 50; actualTo: 400
                    actualValue: Config.sliceWidth
                    onUpdated: val => Config.saveKey("components.appWallpaper.sliceWidth", val)
                }
                SettingsStepper {
                    label: "Expanded Width"
                    subtext: "Width of active item"
                    actualFrom: 400; actualTo: 1600
                    actualValue: Config.expandedWidth
                    onUpdated: val => Config.saveKey("components.appWallpaper.expandedWidth", val)
                }
                SettingsStepper {
                    label: "Height"
                    subtext: "Height of all items"
                    actualFrom: 200; actualTo: 1000
                    actualValue: Config.sliceHeight
                    onUpdated: val => Config.saveKey("components.appWallpaper.sliceHeight", val)
                }
                SettingsStepper {
                    label: "Skew Offset"
                    subtext: "Slant angle of items"
                    actualFrom: -150; actualTo: 150
                    actualValue: Config.skewOffset
                    onUpdated: val => Config.saveKey("components.appWallpaper.skewOffset", val)
                }
                SettingsStepper {
                    label: "Spacing"
                    subtext: "Gap between items"
                    last: true
                    actualFrom: -100; actualTo: 100
                    actualValue: Config.sliceSpacing
                    onUpdated: val => Config.saveKey("components.appWallpaper.sliceSpacing", val)
                }
                
                // --- Hex Settings ---
                Common.SectionHeader { text: "Hex Mode" }
                
                SettingsStepper {
                    label: "Hex Radius"
                    subtext: "Size of hexagons"
                    first: true
                    actualFrom: 50; actualTo: 300
                    actualValue: Config.hexRadius
                    onUpdated: val => Config.saveKey("components.appWallpaper.hexRadius", val)
                }
                SettingsStepper {
                    label: "No. of Rows"
                    subtext: "Rows in hex grid"
                    actualFrom: 1; actualTo: 10
                    actualValue: Config.hexRows
                    onUpdated: val => Config.saveKey("components.appWallpaper.hexRows", Math.round(val))
                }
                SettingsStepper {
                    label: "No. of Columns"
                    subtext: "Columns in hex grid"
                    actualFrom: 1; actualTo: 20
                    actualValue: Config.hexCols
                    onUpdated: val => Config.saveKey("components.appWallpaper.hexCols", Math.round(val))
                }
                SettingsStepper {
                    label: "Arc Intensity"
                    subtext: "Hex layout curvature"
                    last: true
                    actualFrom: 0.0; actualTo: 3.0
                    actualStep: 0.1
                    actualValue: Config.hexArcIntensity
                    onUpdated: val => Config.saveKey("components.appWallpaper.hexArcIntensity", val)
                }
                
                // --- Grid Settings ---
                Common.SectionHeader { text: "Grid Mode" }
                
                SettingsStepper {
                    label: "No. of Columns"
                    subtext: "Number of columns"
                    first: true
                    actualFrom: 1; actualTo: 15
                    actualValue: Config.gridColumns
                    onUpdated: val => Config.saveKey("components.appWallpaper.gridColumns", Math.round(val))
                }
                SettingsStepper {
                    label: "No. of Rows"
                    subtext: "Number of rows"
                    actualFrom: 1; actualTo: 10
                    actualValue: Config.gridRows
                    onUpdated: val => Config.saveKey("components.appWallpaper.gridRows", Math.round(val))
                }
                SettingsStepper {
                    label: "Thumbnail Width"
                    subtext: "Item width"
                    actualFrom: 100; actualTo: 800
                    actualValue: Config.gridThumbWidth
                    onUpdated: val => Config.saveKey("components.appWallpaper.gridThumbWidth", val)
                }
                SettingsStepper {
                    label: "Thumbnail Height"
                    subtext: "Item height"
                    last: true
                    actualFrom: 50; actualTo: 600
                    actualValue: Config.gridThumbHeight
                    onUpdated: val => Config.saveKey("components.appWallpaper.gridThumbHeight", val)
                }
                
                Item { Layout.preferredHeight: ShellConfig.Tokens.padding.largeIncreased } // Bottom padding
            }
        }
    }
    
    component SettingsStepper : Common.StepperRow {
        property real actualFrom: 0
        property real actualTo: 100
        property real actualValue: 0
        property real actualStep: 1
        
        readonly property bool absolute: label.startsWith("No. of")
        
        signal updated(real val)
        
        from: absolute ? actualFrom : 0
        to: absolute ? actualTo : 100
        stepSize: absolute ? actualStep : 1
        
        value: {
            if (absolute) return actualValue
            if (actualTo === actualFrom) return 0
            var ratio = (actualValue - actualFrom) / (actualTo - actualFrom)
            return Math.round(Math.max(0.0, Math.min(1.0, ratio)) * 100)
        }
        
        onMoved: function(v) {
            if (absolute) {
                updated(v)
            } else {
                var val = actualFrom + (v / 100.0) * (actualTo - actualFrom)
                if (actualStep === 1 || actualStep === Math.floor(actualStep)) {
                    val = Math.round(val)
                }
                updated(val)
            }
        }
    }
}

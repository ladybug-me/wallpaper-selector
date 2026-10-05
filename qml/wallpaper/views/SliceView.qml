import QtQuick
import qs.services.api
import "../../"
import ".."

ListView {
    id: root

    property var colors
    property int visibleCount: 5
    property int expandedWidth: 800
    property int sliceWidth: 100
    property int sliceSpacing: 10
    property int skewOffset: 40

    signal cycleNext(int step)
    signal cyclePrev(int step)
    signal wallpaperSelected(string path)
    signal escapePressed()
    signal appendSearchText(string text)
    signal backspaceSearchText()
    signal interactionStarted()

    width: expandedWidth + (visibleCount - 1) * (sliceWidth + sliceSpacing)
    Behavior on width { NumberAnimation { duration: Style.animExpand; easing.type: Easing.OutCubic } }

    orientation: ListView.Horizontal
    clip: false
    spacing: sliceSpacing
    keyNavigationEnabled: false

    flickDeceleration: 1500
    maximumFlickVelocity: 3000
    boundsBehavior: Flickable.StopAtBounds
    cacheBuffer: expandedWidth

    property bool _initialSnap: true
    onVisibleChanged: {
        if (visible) {
            _initialSnap = true
            highlightMoveDuration = 0
            _snapRestoreTimer.restart()
        }
    }

    Timer {
        id: _snapRestoreTimer
        interval: 50
        onTriggered: {
            root.highlightMoveDuration = Style.animExpand
            root._initialSnap = false
        }
    }

    highlightFollowsCurrentItem: true
    highlightMoveDuration: Style.animExpand
    highlight: Item {}
    preferredHighlightBegin: (width - expandedWidth) / 2
    preferredHighlightEnd: (width + expandedWidth) / 2
    highlightRangeMode: ListView.StrictlyEnforceRange
    header: Item { width: (root.width - root.expandedWidth) / 2; height: 1 }
    footer: Item { width: (root.width - root.expandedWidth) / 2; height: 1 }

    add: Transition {
        NumberAnimation { property: "opacity"; from: 0; to: 1; duration: Style.animEnter; easing.type: Easing.OutCubic }
        NumberAnimation { property: "scale"; from: 0.85; to: 1; duration: Style.animEnter; easing.type: Easing.OutCubic }
    }
    remove: Transition {
        NumberAnimation { property: "opacity"; to: 0; duration: Style.animNormal; easing.type: Easing.InCubic }
    }
    displaced: Transition {
        enabled: !root._initialSnap
        NumberAnimation { properties: "x,y"; duration: Style.animMedium; easing.type: Easing.OutCubic }
    }
    move: Transition {
        enabled: !root._initialSnap
        NumberAnimation { properties: "x,y"; duration: Style.animMedium; easing.type: Easing.OutCubic }
    }

    property double lastWheelTime: 0
    WheelHandler {
        acceptedDevices: PointerDevice.Mouse | PointerDevice.TouchPad
        onWheel: function(event) {
            if (typeof appWallpaper !== "undefined" && typeof appWallpaper.hideCursor === "function") {
                appWallpaper.hideCursor(100)
            }
            root.interactionStarted()
            var now = Date.now()
            if (now - root.lastWheelTime < 100) {
                event.accepted = true
                return
            }
            root.lastWheelTime = now
            if (event.angleDelta.y > 0 || event.angleDelta.x > 0) {
                root.cyclePrev(1)
            } else if (event.angleDelta.y < 0 || event.angleDelta.x < 0) {
                root.cycleNext(1)
            }
            event.accepted = true
        }
    }

    Keys.onPressed: event => {
        if (event.key === Qt.Key_Escape) {
            root.escapePressed()
            event.accepted = true
            return
        }

        if (event.text && event.text.length > 0 && !event.modifiers) {
            var c = event.text.charCodeAt(0)
            if (c >= 32 && c < 127) {
                root.appendSearchText(event.text)
                event.accepted = true
                return
            }
        }

        if (event.key === Qt.Key_Backspace) {
            root.backspaceSearchText()
            event.accepted = true
            return
        }

        if (event.key === Qt.Key_Return || event.key === Qt.Key_Enter) {
            if (root.currentIndex >= 0 && root.model.values && root.currentIndex < root.model.values.length) {
                var app = root.model.values ? root.model.values[root.currentIndex] : null
                root.wallpaperSelected(app.path)
            }
            event.accepted = true
            return
        }
    }

    delegate: SliceDelegate {
        colors: root.colors
        expandedWidth: root.expandedWidth
        sliceWidth: root.sliceWidth
        skewOffset: root.skewOffset
        onActivated: function(item) {
            if (item) {
                root.wallpaperSelected(item.path)
            }
        }
    }
}

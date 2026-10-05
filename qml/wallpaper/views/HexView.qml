import QtQuick
import qs.services.api
import "../../"
import ".."

ListView {
    id: root

    property var colors
    property var wallpaperData: []

    signal cycleNext(int step)
    signal cyclePrev(int step)
    signal wallpaperSelected(string path)
    signal escapePressed()
    signal appendSearchText(string text)
    signal backspaceSearchText()
    signal interactionStarted()

    property int _selectedCol: currentIndex
    property int _selectedRow: 0

    orientation: ListView.Horizontal
    clip: true
    property int _rows: Config.hexRows
    property real _r: Config.hexRadius
    property real _gridSpacing: 6
    property real _hexW: _r * 2
    property real _hexH: Math.ceil(_r * 1.73205)
    property real _stepX: 1.5 * _r + _gridSpacing
    property real _stepY: _hexH + _gridSpacing
    property real _gridContentH: (_rows - 1) * _stepY + _hexH + _hexH / 2
    property real _yOffset: Math.max(0, (height - _gridContentH) / 2)
    property real _visibleBand: (Config.hexCols - 1) * _stepX + _hexW
    property real _fadeZone: (width - _visibleBand) / 2

    boundsBehavior: Flickable.StopAtBounds
    flickDeceleration: 1500
    maximumFlickVelocity: 3000
    cacheBuffer: _stepX * 2

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

    spacing: 0
    keyNavigationEnabled: false
    highlightFollowsCurrentItem: true
    highlightMoveDuration: Style.animExpand
    highlight: Item {}
    preferredHighlightBegin: (width - _hexW) / 2
    preferredHighlightEnd: (width + _hexW) / 2
    highlightRangeMode: ListView.StrictlyEnforceRange

    header: Item { width: (root.width - root._hexW) / 2 }
    footer: Item { width: (root.width - root._hexW) / 2 }

    add: Transition {
        NumberAnimation { property: "opacity"; from: 0; to: 1; duration: Style.animEnter; easing.type: Easing.OutCubic }
        NumberAnimation { property: "scale"; from: 0.9; to: 1; duration: Style.animEnter; easing.type: Easing.OutCubic }
    }
    remove: Transition {
        NumberAnimation { property: "opacity"; to: 0; duration: Style.animNormal; easing.type: Easing.InCubic }
    }
    displaced: Transition {
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
            if (root.currentIndex >= 0 && root.wallpaperData && root.currentIndex < root.wallpaperData.length) {
                var app = root.wallpaperData[root.currentIndex]
                if (app) root.wallpaperSelected(app.path)
            }
            event.accepted = true
            return
        }
    }

    delegate: Item {
        id: hexCol
        width: root._stepX
        height: root.height
        clip: false
        property int colIdx: index

        readonly property real _colCenter: (x - root.contentX) + width * 0.5
        readonly property bool _insideView: _colCenter > -root._hexW && _colCenter < root.width + root._hexW
        readonly property bool _nearEdge: _colCenter < root._fadeZone || _colCenter > (root.width - root._fadeZone)
        readonly property bool _nearLeft: _colCenter < root.width / 2
        readonly property bool _visible: _insideView && !_nearEdge
        property real _colScale: _visible ? 1 : 0
        Behavior on _colScale { enabled: !root._initialSnap; NumberAnimation { duration: Style.animExpand; easing.type: Easing.OutCubic } }

        property real _arcFactor: Config.hexArc ? Config.hexArcIntensity : 0
        Behavior on _arcFactor { NumberAnimation { duration: Style.animExpand; easing.type: Easing.OutCubic } }

        readonly property real _arcOffset: {
            if (_arcFactor === 0) return 0
            var viewCenterX = root.width / 2
            var normalized = (_colCenter - viewCenterX) / Math.max(1, viewCenterX)
            return -normalized * normalized * root._r * _arcFactor
        }

        Repeater {
            id: hexCellRepeater
            property var _items: {
                var arr = []
                var start = hexCol.colIdx * root._rows
                var end = Math.min(start + root._rows, root.wallpaperData ? root.wallpaperData.length : 0)
                for (var i = start; i < end; i++) {
                    var r = root.wallpaperData ? root.wallpaperData[i] : null
                    if (r) arr.push({ row: r, rowIdx: i - start, flatIdx: i })
                }
                return arr
            }
            model: _items

            HexDelegate {
                required property var modelData
                readonly property int rowIdx: modelData.rowIdx
                readonly property int flatIdx: modelData.flatIdx

                hexRadius: root._r
                colors: root.colors
                
                itemData: modelData.row
                isSelected: hexCol.colIdx === root._selectedCol && rowIdx === root._selectedRow

                x: 0
                y: root._yOffset + rowIdx * root._stepY + (hexCol.colIdx % 2 !== 0 ? root._hexH / 2 : 0) + hexCol._arcOffset

                scale: hexCol._colScale
                transformOrigin: hexCol._nearLeft ? Item.Left : Item.Right
                opacity: hexCol._colScale < 0.01 ? 0 : 1

                onHoverSelected: {
                    if (root._selectedCol !== hexCol.colIdx || root._selectedRow !== rowIdx) {
                        root.interactionStarted()
                        root._selectedCol = hexCol.colIdx
                        root._selectedRow = rowIdx
                    }
                }
                onActivated: function(item) {
                    if (item) {
                        root.wallpaperSelected(item.path)
                    }
                }
            }
        }
    }
}

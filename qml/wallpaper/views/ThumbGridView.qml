import qs.utils
import QtQuick
import qs.services.api
import qs.services
import qs.components.images
import "../../"

GridView {
    id: root

    property var colors
    property int _gridCellGap: 8

    signal cycleNext(int step)
    signal cyclePrev(int step)
    signal wallpaperSelected(string path)
    signal escapePressed()
    signal appendSearchText(string text)
    signal backspaceSearchText()
    signal interactionStarted()

    cellWidth: Config.gridThumbWidth + root._gridCellGap
    cellHeight: Config.gridThumbHeight + root._gridCellGap
    clip: true
    keyNavigationEnabled: false
    cacheBuffer: 300
    boundsBehavior: Flickable.StopAtBounds

    onVisibleChanged: {
        if (visible && currentIndex >= 0) {
            positionViewAtIndex(currentIndex, GridView.Center)
            _ensureVisible(currentIndex)
        }
    }


    property real _scrollTarget: 0
    onContentYChanged: {
        if (!_gridScrollAnim.running) _scrollTarget = contentY
    }

    NumberAnimation {
        id: _gridScrollAnim
        target: root
        property: "contentY"
        duration: 400
        easing.type: Easing.OutCubic
    }

    function _snapScroll(delta) {
        if (!_gridScrollAnim.running) _scrollTarget = contentY
        var step = cellHeight
        _scrollTarget += (delta > 0 ? -step : step)
        var maxY = Math.max(0, contentHeight - height)
        _scrollTarget = Math.max(0, Math.min(_scrollTarget, maxY))
        _gridScrollAnim.stop()
        _gridScrollAnim.from = contentY
        _gridScrollAnim.to = _scrollTarget
        _gridScrollAnim.start()
    }

    function _snapScrollTo(target) {
        var maxY = Math.max(0, contentHeight - height)
        _scrollTarget = Math.max(0, Math.min(target, maxY))
        _gridScrollAnim.stop()
        _gridScrollAnim.from = contentY
        _gridScrollAnim.to = _scrollTarget
        _gridScrollAnim.start()
    }

    function _ensureVisible(idx) {
        var cols = Math.max(1, Math.floor(width / cellWidth))
        var row = Math.floor(idx / cols)
        var rowTop = row * cellHeight
        var rowBottom = rowTop + cellHeight
        if (rowTop < contentY) _snapScrollTo(rowTop)
        else if (rowBottom > contentY + height) _snapScrollTo(rowBottom - height)
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
            if (root.currentItem) {
                root.wallpaperSelected(root.currentItem._path)
            }
            event.accepted = true
            return
        }
    }

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

    delegate: Rectangle {
        id: delegateItem
        required property var modelData
        required property int index

        readonly property bool isCurrent: GridView.isCurrentItem || (root.currentIndex === index)
        readonly property bool isHovered: _gridMouse.containsMouse
        
        width: Config.gridThumbWidth
        height: Config.gridThumbHeight
        radius: 6
        color: Qt.rgba(root.colors.surfaceContainer.r, root.colors.surfaceContainer.g, root.colors.surfaceContainer.b, 0.85)
        clip: true

        readonly property bool _preferGlyph: false
        readonly property string _path: modelData.path

        CachingImage {
            anchors.fill: parent
            path: Images.isVideo(modelData.name) ? CaelestiaApi.visuals.wallpaper.thumbFor(modelData.path) : modelData.path
            horizontalAlignment: Image.AlignHCenter
            verticalAlignment: Image.AlignVCenter
            smooth: true
            sourceSize.width: parent.width
            sourceSize.height: parent.height
            visible: status === Image.Ready
        }

        Text {
            anchors.centerIn: parent
            visible: Images.isVideo(modelData.name)
            text: "\ue04b"
            font.family: Style.fontFamilyIcons
            font.pixelSize: Math.min(parent.width, parent.height) * 0.45
            color: Qt.rgba(root.colors.primary.r, root.colors.primary.g, root.colors.primary.b, 0.85)
        }

        Rectangle {
            anchors.fill: parent
            color: Qt.rgba(0, 0, 0, isCurrent ? 0 : (isHovered ? 0.15 : 0.4))
            Behavior on color { ColorAnimation { duration: 150 } }
        }

        Text {
            anchors.bottom: parent.bottom
            anchors.bottomMargin: 6
            anchors.horizontalCenter: parent.horizontalCenter
            text: (modelData.name || "").toUpperCase()
            font.family: Style.fontFamily
            font.pixelSize: 10
            font.weight: Font.Bold
            color: "#fff"
        }

        // Colored selection border (layered on top)
        Rectangle {
            id: thumbBorder
            anchors.fill: parent
            radius: 6
            color: "transparent"
            z: 10
            border.width: isCurrent ? 3 : (isHovered ? 2 : 1)
            border.color: isCurrent
                ? root.colors.primary
                : (isHovered
                    ? Qt.rgba(root.colors.primary.r, root.colors.primary.g, root.colors.primary.b, 0.4)
                    : Qt.rgba(0, 0, 0, 0.5))
            Behavior on border.color { ColorAnimation { duration: 150 } }
            Behavior on border.width { NumberAnimation { duration: 150 } }
        }

        MouseArea {
            id: _gridMouse
            anchors.fill: parent
            hoverEnabled: true
            cursorShape: Qt.PointingHandCursor
            onPositionChanged: {
                if (appWallpaper.blockHover) return
                if (root.currentIndex !== index) {
                    root.interactionStarted()
                    root.currentIndex = index
                }
            }
            onClicked: {
                root.wallpaperSelected(modelData.path)
            }
        }
    }
    
    
}

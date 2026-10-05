import QtQuick
import QtQuick.Shapes
import QtQuick.Effects
import Quickshell
import qs.utils
import qs.services.api
import qs.components.images
import ".."

Item {
    id: hexItem

    property var colors
    property var service
    property int hexRadius: 140
    property var itemData
    property bool isSelected: false
    property bool isHovered: hexMouse.containsMouse

    property real parallaxX: 0
    property real parallaxY: 0

    signal hoverSelected()
    signal activated(var data)

    width: hexRadius * 2
    height: Math.ceil(hexRadius * 1.73205)

    readonly property real _r: hexRadius
    readonly property real _cx: _r
    readonly property real _cy: height / 2
    readonly property real _cos30: 0.866025
    readonly property real _sin30: 0.5

    readonly property string _label: itemData ? itemData.name : ""

    Item {
        id: hexMask
        width: hexItem.width; height: hexItem.height
        visible: false
        layer.enabled: true
        Shape {
            anchors.fill: parent
            antialiasing: true
            preferredRendererType: Shape.CurveRenderer
            ShapePath {
                fillColor: "white"
                strokeColor: "transparent"
                startX: hexItem._cx + hexItem._r;                          startY: hexItem._cy
                PathLine { x: hexItem._cx + hexItem._r * hexItem._sin30;  y: hexItem._cy - hexItem._r * hexItem._cos30 }
                PathLine { x: hexItem._cx - hexItem._r * hexItem._sin30;  y: hexItem._cy - hexItem._r * hexItem._cos30 }
                PathLine { x: hexItem._cx - hexItem._r;                   y: hexItem._cy }
                PathLine { x: hexItem._cx - hexItem._r * hexItem._sin30;  y: hexItem._cy + hexItem._r * hexItem._cos30 }
                PathLine { x: hexItem._cx + hexItem._r * hexItem._sin30;  y: hexItem._cy + hexItem._r * hexItem._cos30 }
                PathLine { x: hexItem._cx + hexItem._r;                   y: hexItem._cy }
            }
        }
    }

    Item {
        id: imageContainer
        anchors.fill: parent

        Rectangle {
            id: hexPlaceholder
            anchors.centerIn: parent
            width: hexItem.width * 1.3
            height: hexItem.height * 1.3
            color: Style.fallbackAccent
            opacity: bgImage.status === Image.Ready ? 0 : 0.08
            Behavior on opacity { NumberAnimation { duration: Style.animNormal; easing.type: Easing.OutCubic } }
            visible: opacity > 0
        }

        CachingImage {
            id: bgImage
            width: hexItem.width * 1.3
            height: hexItem.height * 1.3
            x: (hexItem.width - width) / 2 + hexItem.parallaxX
            y: (hexItem.height - height) / 2 + hexItem.parallaxY
            path: Images.isVideo(itemData ? itemData.name : "") ? CaelestiaApi.visuals.wallpaper.thumbFor(itemData ? itemData.path : "") : (itemData ? itemData.path : "")
            smooth: true
            opacity: status === Image.Ready ? 1 : 0
            Behavior on opacity { NumberAnimation { duration: Style.animNormal; easing.type: Easing.OutCubic } }
        }

        Text {
            anchors.centerIn: parent
            text: Images.isVideo(itemData ? itemData.name : "") ? "\ue04b" : "\uf03e"
            font.family: Style.fontFamilyIcons
            font.pixelSize: Math.max(24, hexItem.height * 0.4)
            color: Qt.rgba(hexItem.colors.primary.r, hexItem.colors.primary.g, hexItem.colors.primary.b, 0.85)
            visible: bgImage.status !== Image.Ready
        }

        layer.enabled: true
        layer.smooth: true
        layer.effect: MultiEffect {
            maskEnabled: true
            maskSource: hexMask
            maskThresholdMin: 0.3
            maskSpreadAtMin: 0.3
        }
    }

    Shape {
        id: hexBorder
        anchors.fill: parent
        antialiasing: true
        preferredRendererType: Shape.CurveRenderer
        ShapePath {
            fillColor: "transparent"
            strokeColor: hexItem.isSelected
                ? (hexItem.colors.primary)
                : Qt.rgba(0, 0, 0, 0.5)
            Behavior on strokeColor { ColorAnimation { duration: Style.animFast } }
            strokeWidth: hexItem.isSelected ? 3 : 1.5
            startX: hexItem._cx + hexItem._r;                          startY: hexItem._cy
            PathLine { x: hexItem._cx + hexItem._r * hexItem._sin30;  y: hexItem._cy - hexItem._r * hexItem._cos30 }
            PathLine { x: hexItem._cx - hexItem._r * hexItem._sin30;  y: hexItem._cy - hexItem._r * hexItem._cos30 }
            PathLine { x: hexItem._cx - hexItem._r;                   y: hexItem._cy }
            PathLine { x: hexItem._cx - hexItem._r * hexItem._sin30;  y: hexItem._cy + hexItem._r * hexItem._cos30 }
            PathLine { x: hexItem._cx + hexItem._r * hexItem._sin30;  y: hexItem._cy + hexItem._r * hexItem._cos30 }
            PathLine { x: hexItem._cx + hexItem._r;                   y: hexItem._cy }
        }
    }

    Rectangle {
        anchors.horizontalCenter: parent.horizontalCenter
        anchors.bottom: parent.bottom
        anchors.bottomMargin: hexItem._r * 0.18
        width: nameLabel.implicitWidth + 14
        height: 18
        radius: 9
        color: Qt.rgba(hexItem.colors.surfaceContainer.r, hexItem.colors.surfaceContainer.g, hexItem.colors.surfaceContainer.b, 0.85)
        border.width: 1
        border.color: Qt.rgba(hexItem.colors.primary.r, hexItem.colors.primary.g, hexItem.colors.primary.b, 0.4)
        z: 5
        visible: hexItem._label.length > 0

        Text {
            id: nameLabel
            anchors.centerIn: parent
            text: hexItem._label
            font.family: Style.fontFamily; font.pixelSize: 10; font.weight: Font.Bold; font.letterSpacing: 0.5
            color: hexItem.colors.surfaceText
        }
    }

    MouseArea {
        id: hexMouse
        anchors.fill: parent
        hoverEnabled: true
        acceptedButtons: Qt.LeftButton
        cursorShape: Qt.PointingHandCursor
        function contains(point) {
            var dx = Math.abs(point.x - hexItem._cx)
            var dy = Math.abs(point.y - hexItem._cy)
            return dy <= hexItem._cos30 * hexItem._r && dx <= hexItem._r - dy * 0.57735
        }
        onPositionChanged: function(mouse) {
            if (appWallpaper.blockHover) return
            if (contains(Qt.point(mouse.x, mouse.y))) hexItem.hoverSelected()
        }
        onClicked: function(mouse) {
            if (mouse.button === Qt.LeftButton && hexItem.itemData) {
                hexItem.activated(hexItem.itemData)
            }
        }
    }
}

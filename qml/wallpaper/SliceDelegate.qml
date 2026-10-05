import QtQuick
import QtQuick.Shapes
import QtQuick.Effects
import qs.utils
import qs.services.api
import qs.components.images
import ".."

Item {
    id: delegateItem

    required property int index
    required property var modelData

    property var colors
    property int expandedWidth: 924
    property int sliceWidth: 135
    property int skewOffset: 35
    property var service

    property bool isCurrent: ListView.isCurrentItem
    property bool isHovered: itemMouseArea.containsMouse
    readonly property var _listView: ListView.view

    signal activated(var data)

    
    readonly property real _skAbs: Math.abs(skewOffset)
    readonly property real _topLeft: skewOffset >= 0 ? _skAbs : 0
    readonly property real _topRight: skewOffset >= 0 ? width : width - _skAbs
    readonly property real _botRight: skewOffset >= 0 ? width - _skAbs : width
    readonly property real _botLeft: skewOffset >= 0 ? 0 : _skAbs
    readonly property real _slantSx: _botLeft - _topLeft
    readonly property real _slantLen: Math.max(0.001, Math.sqrt(_slantSx * _slantSx + height * height))
    readonly property real _flatW: Math.max(0.001, _topRight - _topLeft)
    property real animatedCornerRadius: Config.sliceCornerRadius
    Behavior on animatedCornerRadius { NumberAnimation { duration: Style.animExpand; easing.type: Easing.OutCubic } }
    readonly property real _rEff: Math.max(0, Math.min(animatedCornerRadius, _flatW / 2 - 1, _slantLen / 2 - 1))
    readonly property real _rUx: _rEff * _slantSx / _slantLen
    readonly property real _rUy: _rEff * height / _slantLen
    readonly property real _tlInX: _topLeft + _rUx
    readonly property real _tlInY: _rUy
    readonly property real _tlOutX: _topLeft + _rEff
    readonly property real _trInX: _topRight - _rEff
    readonly property real _trOutX: _topRight + _rUx
    readonly property real _trOutY: _rUy
    readonly property real _brInX: _botRight - _rUx
    readonly property real _brInY: height - _rUy
    readonly property real _brOutX: _botRight - _rEff
    readonly property real _blInX: _botLeft + _rEff
    readonly property real _blOutX: _botLeft - _rUx
    readonly property real _blOutY: height - _rUy

    property bool suppressWidthAnim: false

    width: isCurrent ? expandedWidth : sliceWidth
    height: _listView ? _listView.height : 0

    z: isCurrent ? 100 : (isHovered ? 90 : 50 - Math.min(Math.abs(index - (_listView ? _listView.currentIndex : 0)), 50))

    readonly property real _viewCenterX: _listView ? (_listView.contentX + _listView.width / 2) : 0
    readonly property real _itemCenterX: x + width / 2
    readonly property real _halfView: _listView ? _listView.width / 2 : 1
    readonly property real _fullZone: Math.min(0.6, (expandedWidth / 2 + 2 * (sliceWidth + (_listView ? _listView.spacing : 0))) / _halfView)
    readonly property real _normDist: Math.abs(_itemCenterX - _viewCenterX) / _halfView
    opacity: _normDist <= _fullZone ? 1.0 : Math.max(0, 1.0 - (_normDist - _fullZone) / (1.2 - _fullZone))
    readonly property bool _nearViewport: opacity > 0.01

    Behavior on width {
        enabled: !suppressWidthAnim && !(_listView && _listView._initialSnap)
        NumberAnimation { duration: Style.animExpand; easing.type: Easing.OutCubic }
    }

    containmentMask: Item {
        id: hitMask
        function contains(point) {
            var w = delegateItem.width
            var h = delegateItem.height
            if (h <= 0 || w <= 0) return false
            var t = point.y / h
            var margin = delegateItem.isCurrent ? 0 : 25 // 25px offset deadzone
            var leftX = delegateItem._topLeft * (1.0 - t) + delegateItem._botLeft * t + margin
            var rightX = delegateItem._topRight * (1.0 - t) + delegateItem._botRight * t - margin
            if (rightX < leftX) return false // guard against extremely thin items
            return point.x >= leftX && point.x <= rightX && point.y >= 0 && point.y <= h
        }
    }

    Item {
        id: sharedMask
        width: delegateItem.width
        height: delegateItem.height
        visible: false
        layer.enabled: delegateItem._nearViewport
        layer.smooth: true
        Shape {
            anchors.fill: parent
            antialiasing: true
            preferredRendererType: Shape.CurveRenderer
            ShapePath {
                fillColor: "white"
                strokeColor: "transparent"
                startX: delegateItem._tlOutX
                startY: 0
                PathLine { x: delegateItem._trInX; y: 0 }
                PathQuad { x: delegateItem._trOutX; y: delegateItem._trOutY; controlX: delegateItem._topRight; controlY: 0 }
                PathLine { x: delegateItem._brInX; y: delegateItem._brInY }
                PathQuad { x: delegateItem._brOutX; y: delegateItem.height; controlX: delegateItem._botRight; controlY: delegateItem.height }
                PathLine { x: delegateItem._blInX; y: delegateItem.height }
                PathQuad { x: delegateItem._blOutX; y: delegateItem._blOutY; controlX: delegateItem._botLeft; controlY: delegateItem.height }
                PathLine { x: delegateItem._tlInX; y: delegateItem._tlInY }
                PathQuad { x: delegateItem._tlOutX; y: 0; controlX: delegateItem._topLeft; controlY: 0 }
            }
        }
    }

    
    Shape {
        id: shadowShape
        z: -1
        x: delegateItem.isCurrent ? 4 : 2
        y: delegateItem.isCurrent ? 10 : 5
        width: delegateItem.width
        height: delegateItem.height
        opacity: delegateItem.isCurrent ? 0.5 : 0.3
        Behavior on x { NumberAnimation { duration: Style.animNormal } }
        Behavior on y { NumberAnimation { duration: Style.animNormal } }
        Behavior on opacity { NumberAnimation { duration: Style.animNormal } }
        ShapePath {
            fillColor: "#000000"
            strokeColor: "transparent"
            startX: delegateItem._tlOutX
            startY: 0
            PathLine { x: delegateItem._trInX; y: 0 }
            PathQuad { x: delegateItem._trOutX; y: delegateItem._trOutY; controlX: delegateItem._topRight; controlY: 0 }
            PathLine { x: delegateItem._brInX; y: delegateItem._brInY }
            PathQuad { x: delegateItem._brOutX; y: delegateItem.height; controlX: delegateItem._botRight; controlY: delegateItem.height }
            PathLine { x: delegateItem._blInX; y: delegateItem.height }
            PathQuad { x: delegateItem._blOutX; y: delegateItem._blOutY; controlX: delegateItem._botLeft; controlY: delegateItem.height }
            PathLine { x: delegateItem._tlInX; y: delegateItem._tlInY }
            PathQuad { x: delegateItem._tlOutX; y: 0; controlX: delegateItem._topLeft; controlY: 0 }
        }
    }

    
    Item {
        id: imageContainer
        anchors.fill: parent

        CachingImage {
            id: bgImage
            anchors.fill: parent
            path: Images.isVideo(delegateItem.modelData.name) ? CaelestiaApi.visuals.wallpaper.thumbFor(delegateItem.modelData.path) : delegateItem.modelData.path
            smooth: true
            sourceSize.width:  Math.ceil(delegateItem.expandedWidth)
            sourceSize.height: Math.ceil(delegateItem.height)
            visible: source.toString() !== "file://" && status === Image.Ready
        }

        Rectangle {
            anchors.fill: parent
            visible: !bgImage.visible
            color: Qt.rgba(delegateItem.colors.surfaceVariant.r, delegateItem.colors.surfaceVariant.g, delegateItem.colors.surfaceVariant.b, 0.8)
        }

        Text {
            id: glyphIcon
            anchors.centerIn: parent
            text: Images.isVideo(delegateItem.modelData.name) ? "\ue04b" : "\uf03e"
            font.family: Style.fontFamilyIcons
            font.pixelSize: 48
            color: Qt.rgba(delegateItem.colors.primary.r, delegateItem.colors.primary.g, delegateItem.colors.primary.b, 0.7)
            visible: !bgImage.visible
        }

        Rectangle {
            anchors.fill: parent
            color: Qt.rgba(0, 0, 0, delegateItem.isCurrent ? 0 : (delegateItem.isHovered ? 0.15 : 0.4))
            Behavior on color { ColorAnimation { duration: Style.animNormal } }
        }

        layer.enabled: delegateItem._nearViewport
        layer.smooth: true
        layer.effect: MultiEffect {
            maskEnabled: true
            maskSource: sharedMask
            maskThresholdMin: 0.3
            maskSpreadAtMin: 0.3
        }
    }

    
    Shape {
        id: glowBorder
        anchors.fill: parent
        antialiasing: true
        preferredRendererType: Shape.CurveRenderer
        opacity: 1.0
        ShapePath {
            fillColor: "transparent"
            strokeColor: delegateItem.isCurrent
                ? (delegateItem.colors.primary)
                : (delegateItem.isHovered
                    ? Qt.rgba(delegateItem.colors.primary.r, delegateItem.colors.primary.g, delegateItem.colors.primary.b, 0.4)
                    : Qt.rgba(0, 0, 0, 0.6))
            Behavior on strokeColor { ColorAnimation { duration: Style.animNormal } }
            strokeWidth: delegateItem.isCurrent ? 3 : 1
            startX: delegateItem._tlOutX
            startY: 0
            PathLine { x: delegateItem._trInX; y: 0 }
            PathQuad { x: delegateItem._trOutX; y: delegateItem._trOutY; controlX: delegateItem._topRight; controlY: 0 }
            PathLine { x: delegateItem._brInX; y: delegateItem._brInY }
            PathQuad { x: delegateItem._brOutX; y: delegateItem.height; controlX: delegateItem._botRight; controlY: delegateItem.height }
            PathLine { x: delegateItem._blInX; y: delegateItem.height }
            PathQuad { x: delegateItem._blOutX; y: delegateItem._blOutY; controlX: delegateItem._botLeft; controlY: delegateItem.height }
            PathLine { x: delegateItem._tlInX; y: delegateItem._tlInY }
            PathQuad { x: delegateItem._tlOutX; y: 0; controlX: delegateItem._topLeft; controlY: 0 }
        }
    }

    
    Item {
        anchors.bottom: parent.bottom
        anchors.bottomMargin: 16
        anchors.horizontalCenter: parent.horizontalCenter
        width: nameLabel.implicitWidth + 24
        height: 22
        z: 10
        visible: delegateItem.isCurrent || delegateItem.isHovered
        opacity: visible ? 1 : 0
        Behavior on opacity { NumberAnimation { duration: Style.animFast } }

        Rectangle {
            anchors.fill: parent
            radius: 11
            color: Qt.rgba(delegateItem.colors.surfaceContainer.r, delegateItem.colors.surfaceContainer.g, delegateItem.colors.surfaceContainer.b, 0.85)
            border.width: 1
            border.color: Qt.rgba(delegateItem.colors.primary.r, delegateItem.colors.primary.g, delegateItem.colors.primary.b, 0.4)
        }

        Text {
            id: nameLabel
            anchors.centerIn: parent
            text: (delegateItem.modelData.name || "").toUpperCase()
            font.family: Style.fontFamily
            font.pixelSize: 11
            font.weight: Font.Bold
            font.letterSpacing: 0.5
            color: delegateItem.colors.surfaceText
        }
    }

    MouseArea {
        id: itemMouseArea
        anchors.fill: parent
        hoverEnabled: true
        cursorShape: Qt.PointingHandCursor
        acceptedButtons: Qt.LeftButton
        
        onPositionChanged: function(mouse) {
            if (appWallpaper.blockHover) return
            if (delegateItem._listView && delegateItem._listView.currentIndex !== delegateItem.index) {
                if (hitMask.contains(Qt.point(mouse.x, mouse.y))) {
                    delegateItem._listView.interactionStarted()
                    delegateItem._listView.currentIndex = delegateItem.index
                }
            }
        }
        onClicked: function(mouse) {
            if (!hitMask.contains(Qt.point(mouse.x, mouse.y))) return
            if (delegateItem.isCurrent) {
                delegateItem.activated(delegateItem.modelData)
            } else if (delegateItem._listView) {
                delegateItem._listView.currentIndex = delegateItem.index
            }
        }
    }
}

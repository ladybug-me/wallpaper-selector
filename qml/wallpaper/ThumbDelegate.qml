import QtQuick
import qs.services.api
import qs.services
import qs.components.images
import ".."

Rectangle {
    id: delegateItem
    required property var modelData
    required property int index
    
    property var colors

    width: Config.gridThumbWidth
    height: Config.gridThumbHeight
    radius: 6
    color: Qt.rgba(colors.surfaceContainer.r, colors.surfaceContainer.g, colors.surfaceContainer.b, 0.85)
    border.width: _gridMouse.containsMouse ? 2 : 0
    border.color: colors.primary
    clip: true

    readonly property bool _preferGlyph: false
    readonly property string _path: modelData.path

    CachingImage {
        anchors.fill: parent
        path: Images.isVideo(modelData.name) ? CaelestiaApi.visuals.wallpaper.thumbFor(modelData.path) : modelData.path
        horizontalAlignment: Image.AlignHCenter
        verticalAlignment: Image.AlignVCenter
        smooth: true
    }

    Text {
        anchors.centerIn: parent
        visible: Images.isVideo(modelData.name)
        text: "\ue04b"
        font.family: Style.fontFamilyIcons
        font.pixelSize: Math.min(parent.width, parent.height) * 0.45
        color: Qt.rgba(colors.primary.r, colors.primary.g, colors.primary.b, 0.85)
    }

    Rectangle {
        anchors.fill: parent
        color: Qt.rgba(0, 0, 0, _gridMouse.containsMouse ? 0.15 : 0.4)
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

    MouseArea {
        id: _gridMouse
        anchors.fill: parent
        hoverEnabled: true
        cursorShape: Qt.PointingHandCursor
        onPositionChanged: {
            if (appWallpaper.blockHover) return
            if (delegateItem.GridView.view.currentIndex !== index) {
                delegateItem.GridView.view.interactionStarted()
                delegateItem.GridView.view.currentIndex = index
            }
        }
        onClicked: {
            delegateItem.GridView.view.wallpaperSelected(modelData.path)
        }
    }
}

import QtQuick
import Caelestia.Config as ShellConfig
import qs.components.controls
import qs.services.api

Row {
    id: root
    spacing: ShellConfig.Tokens.spacing.small

    IconTextButton {
        text: qsTr("Images")
        icon: "image"
        type: CaelestiaApi.visuals.wallpaper.currentMediaFilter === "Image" ? TextButton.Filled : TextButton.Tonal
        onClicked: CaelestiaApi.visuals.wallpaper.currentMediaFilter = (CaelestiaApi.visuals.wallpaper.currentMediaFilter === "Image") ? "All" : "Image"
    }
    IconTextButton {
        text: qsTr("Animated")
        icon: "animation"
        type: CaelestiaApi.visuals.wallpaper.currentMediaFilter === "Animated" ? TextButton.Filled : TextButton.Tonal
        onClicked: CaelestiaApi.visuals.wallpaper.currentMediaFilter = (CaelestiaApi.visuals.wallpaper.currentMediaFilter === "Animated") ? "All" : "Animated"
    }
    IconTextButton {
        text: qsTr("Videos")
        icon: "videocam"
        type: CaelestiaApi.visuals.wallpaper.currentMediaFilter === "Video" ? TextButton.Filled : TextButton.Tonal
        onClicked: CaelestiaApi.visuals.wallpaper.currentMediaFilter = (CaelestiaApi.visuals.wallpaper.currentMediaFilter === "Video") ? "All" : "Video"
    }
}

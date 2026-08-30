import QtQuick
import Quickshell
import qs.services as ShellServices

QtObject {
    id: service

    // Filters (we map some to the shell if applicable, otherwise they are dummy to prevent UI errors)
    property int selectedColorFilter: -1
    property string selectedTypeFilter: ""
    property string selectedFolder: ""
    property var availableFolders: ShellServices.Wallpapers.categories || []

    // Dummy properties for removed bespoke features
    property var selectedTags: []
    property int selectedTagIndex: -1
    property var popularTags: []
    property bool tagsMatchAny: false
    property bool weatherFilterActive: false
    property bool favouriteFilterActive: false
    property bool cacheLoading: false
    property int cacheProgress: 0
    property int cacheTotal: 0
    property bool ollamaActive: false
    property int ollamaTotalThumbs: 0
    property int ollamaTaggedCount: 0
    property int ollamaColoredCount: 0
    property string ollamaEta: ""
    property string ollamaLogLine: ""
    property string lastApplyError: ""
    property bool showing: false

    signal modelUpdated()
    signal requestFilterUpdate()
    signal wallpaperApplied()
    signal wallpaperApplyFailed(string message)

    property bool filterTransitioning: false
    property bool _skipCrossfade: false

    property ListModel filteredModel: ListModel {}

    property var _listBinding: Connections {
        target: ShellServices.Wallpapers
        function onFilteredListChanged() {
            service.updateFilteredModel()
        }
    }

    function commitFilteredModel() {
        modelUpdated()
    }

    function startCacheCheck() {
        // Init model on first show
        updateFilteredModel()
    }

    function refreshFromDb() {
        updateFilteredModel()
    }

    function applyStatic(path, outputs) {
        ShellServices.Wallpapers.setWallpaper(path)
        wallpaperApplied()
    }

    function applyVideo(path, outputs, audioMap, volumeMap) {
        ShellServices.Wallpapers.setWallpaper(path)
        wallpaperApplied()
    }

    function applyWE(id, outputs, audioMap, volumeMap) {
        console.warn("Wallpaper Engine not natively supported by Caelestia Wallpapers service")
    }

    function updateFilteredModel() {
        if (selectedTypeFilter === "static") {
            ShellServices.Wallpapers.currentMediaFilter = "Image"
        } else if (selectedTypeFilter === "video") {
            ShellServices.Wallpapers.currentMediaFilter = "Video"
        } else {
            ShellServices.Wallpapers.currentMediaFilter = "All"
        }

        var sourceArray = ShellServices.Wallpapers.filteredList || []

        filteredModel.clear()

        for (var i = 0; i < sourceArray.length; i++) {
            var item = sourceArray[i]
            var folder = ShellServices.Wallpapers.getCategoryFor(item)

            if (selectedFolder !== "" && selectedFolder !== "*" && selectedFolder !== "Main" && folder !== selectedFolder) {
                continue
            }

            var isVid = item.relativePath.match(/\.(mp4|mkv|webm)$/i)

            filteredModel.append({
                name: item.fileName,
                type: isVid ? "video" : "static",
                path: item.path,
                thumb: ShellServices.Wallpapers.getThumbnailPath(item.path),
                weId: "",
                videoFile: isVid ? item.path : "",
                mtime: 0,
                hue: 99,
                saturation: 0,
                richness: 0,
                applyCount: 0,
                placeholder: false
            })
        }

        requestFilterUpdate()
    }

    onSelectedTypeFilterChanged: updateFilteredModel()
    onSelectedFolderChanged: updateFilteredModel()

    function beginTagsEdit() {}
    function endTagsEdit() {}
    function commitTagsEdit(path, tags) {}
}


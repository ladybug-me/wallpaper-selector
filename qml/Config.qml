pragma Singleton
import QtQuick
import Quickshell
import Quickshell.Io

QtObject {
    id: config

    function _resolve(path) { return path ? path.replace("~", homeDir) : "" }


    readonly property string homeDir: Quickshell.env("HOME")
    readonly property string configDir: (Quickshell.env("XDG_CONFIG_HOME") || (homeDir + "/.config")) + "/caelestia/plugins/wallpaper-selector"
    readonly property string installDir: configDir
    readonly property string runtimeDir: (Quickshell.env("XDG_RUNTIME_DIR") || "/tmp") + "/caelestia/plugins/wallpaper-selector"
    readonly property string scriptsDir: _resolve(_data.paths?.scripts) || (installDir + "/scripts")
    readonly property string cacheDir: _resolve(_data.paths?.cache) || (Quickshell.env("XDG_CACHE_HOME") || (homeDir + "/.cache")) + "/caelestia/plugins/wallpaper-selector"

    property var _data: ({})

    property var _configFile: FileView {
        path: configDir + "/.config"
        preload: true
        watchChanges: true
        onLoaded: config._reparse()
        onFileChanged: { reload(); config._reparse() }
    }

    function _mergeConfig(target, source) {
        var changed = false;
        for (var key in source) {
            if (source.hasOwnProperty(key)) {
                if (typeof source[key] === 'object' && source[key] !== null && !Array.isArray(source[key])) {
                    if (typeof target[key] !== 'object' || target[key] === null || Array.isArray(target[key])) {
                        target[key] = {};
                        changed = true;
                    }
                    if (_mergeConfig(target[key], source[key])) {
                        changed = true;
                    }
                } else if (target[key] === undefined) {
                    target[key] = source[key];
                    changed = true;
                }
            }
        }
        return changed;
    }

    function _reparse() {
        var raw = _configFile.text() || ""
        if (!raw) return
        try { 
            var parsed = JSON.parse(raw)
            var changed = _mergeConfig(parsed, defaultConfig)
            config._data = parsed
            if (changed) {
                _configWriter.setText(JSON.stringify(parsed, null, 2) + "\n")
            }
        } catch (e) {}
    }


    readonly property string compositor: _data.compositor ?? "niri"


    readonly property string mainMonitor: _data.monitor ?? ""
    readonly property string terminal: _data.terminal ?? "kitty"


    readonly property string steamDir: _resolve(_data.paths?.steam)


    readonly property string splashDir: _resolve(_data.paths?.splash) || (homeDir + "/appsplash")


    readonly property real uiScale: Math.max(1.0, Math.min(2.0, _data.general?.uiScale ?? 1.0))


    property var _wallpaper: _data.components?.appWallpaper ?? {}
    readonly property string shortcut: _wallpaper.shortcut ?? "Meta+Shift+W"
    readonly property string displayMode: _wallpaper.displayMode ?? "slice"
    readonly property int sliceWidth: _wallpaper.sliceWidth ?? 135
    readonly property int expandedWidth: _wallpaper.expandedWidth ?? 924
    readonly property int sliceHeight: _wallpaper.sliceHeight ?? 520
    readonly property int skewOffset: _wallpaper.skewOffset ?? 35
    readonly property int sliceSpacing: _wallpaper.sliceSpacing ?? -22
    readonly property int visibleCount: _wallpaper.visibleCount ?? 12
    readonly property bool sliceRoundCorners: _wallpaper.roundCorners === true
    readonly property int sliceCornerRadius: sliceRoundCorners ? (_wallpaper.cornerRadius ?? 16) : 0


    readonly property var customPresets: _wallpaper.customPresets ?? ({})


    readonly property int hexRadius:        _wallpaper.hexRadius        ?? 140
    readonly property int hexRows:          _wallpaper.hexRows          ?? 3
    readonly property int hexCols:          _wallpaper.hexCols          ?? 7
    readonly property int hexScrollStep:    _wallpaper.hexScrollStep    ?? 1
    readonly property bool hexArc:          _wallpaper.hexArc           !== false
    readonly property real hexArcIntensity: _wallpaper.hexArcIntensity  ?? 1.2


    readonly property int gridColumns:      _wallpaper.gridColumns      ?? 6
    readonly property int gridRows:         _wallpaper.gridRows         ?? 3
    readonly property int gridThumbWidth:   _wallpaper.gridThumbWidth   ?? 300
    readonly property int gridThumbHeight:  _wallpaper.gridThumbHeight  ?? 169

    readonly property var _wallpaperFilterDefaults: [
      { key: "all",     icon: "\u{F0136}", label: "All",   type: "all",      value: "" },
      { key: "desktop", icon: "\u{F003B}", label: "Apps",  type: "source",   value: "desktop" },
      { key: "game",    icon: "\u{F0297}", label: "Games", type: "category", value: "Game" },
      { key: "steam",   icon: "\u{F04D3}", label: "Steam", type: "source",   value: "steam" }
    ]
    readonly property var wallpaperFilters: Array.isArray(_wallpaper.filters) && _wallpaper.filters.length > 0 ? _wallpaper.filters : _wallpaperFilterDefaults

    function saveKey(path, value) {
        _configWriter.reload()
        var data
        try { data = JSON.parse(_configWriter.text()) } catch(e) { data = {} }
        var parts = path.split(".")
        var obj = data
        for (var i = 0; i < parts.length - 1; i++) {
            if (typeof obj[parts[i]] !== "object" || obj[parts[i]] === null)
                obj[parts[i]] = {}
            obj = obj[parts[i]]
        }
        obj[parts[parts.length - 1]] = value
        _configWriter.setText(JSON.stringify(data, null, 2) + "\n")
    }

    property var _configWriter: FileView {
        path: configDir + "/.config"
        preload: true
    }

    readonly property var defaultConfig: ({
        compositor: "niri",
        monitor: "",
        terminal: "kitty",
        paths: {
            scripts: "",
            cache: "",
            steam: "",
            splash: ""
        },
        general: {
            uiScale: 1.0
        },
        components: {
            appWallpaper: {
                shortcut: "Meta+Shift+W",
                displayMode: "slice",
                sliceWidth: 135,
                expandedWidth: 924,
                sliceHeight: 520,
                skewOffset: 35,
                sliceSpacing: -22,
                visibleCount: 12,
                roundCorners: false,
                cornerRadius: 16,
                customPresets: {},
                hexRadius: 140,
                hexRows: 3,
                hexCols: 7,
                hexScrollStep: 1,
                hexArc: true,
                hexArcIntensity: 1.2,
                gridColumns: 6,
                gridRows: 3,
                gridThumbWidth: 300,
                gridThumbHeight: 169,
                filters: [
                    { key: "all",     icon: "\uF0136", label: "All",   type: "all",      value: "" },
                    { key: "desktop", icon: "\uF003B", label: "Apps",  type: "source",   value: "desktop" },
                    { key: "game",    icon: "\uF0297", label: "Games", type: "category", value: "Game" },
                    { key: "steam",   icon: "\uF04D3", label: "Steam", type: "source",   value: "steam" }
                ]
            }
        }
    })

    function resetToDefault() {
        var jsonStr = JSON.stringify(defaultConfig, null, 2)
        _configWriter.setText(jsonStr + "\n")
        
        // Temporarily reset properties until reparse catches up
        try { config._data = JSON.parse(jsonStr) } catch (e) {}
    }

    Component.onCompleted: {
        var jsonStr = JSON.stringify(defaultConfig, null, 2)
        Quickshell.execDetached(["bash", "-c", "mkdir -p '" + configDir + "' && if [ ! -s '" + configDir + "/.config' ]; then cat << 'EOF' > '" + configDir + "/.config'\n" + jsonStr + "\nEOF\nfi"])
    }
}

/*
    This file is part of Lotus.

    Copyright (c) 2025 Max Rodriguez <me@maxrdz.com>

    Lotus is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Lotus is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use lotus_plugin::*;

static PLUGIN_METADATA: PluginMetadata = PluginMetadata {
    id: "org.lotus-shell.LotusVanillaPlugin",
    icon: include_bytes!("../../data/icons/hicolor/scalable/apps/icon.png"),
    version: SemanticVersion(0, 1, 0),
    plugin_api_version: SemanticVersion(0, 1, 0),
    dependencies: &[],
    permissions: &[PluginPermissions::Gui, PluginPermissions::Theme],
};

struct LotusVanilla;

impl LotusPlugin for LotusVanilla {
    fn get_plugin_api_version() -> SemanticVersion {
        PLUGIN_METADATA.plugin_api_version.clone()
    }

    fn get_plugin_metadata() -> &'static PluginMetadata {
        &PLUGIN_METADATA
    }

    fn update_locale(_lang: String) -> bool {
        todo!()
    }
}

generate_plugin_ffi!(LotusVanilla);

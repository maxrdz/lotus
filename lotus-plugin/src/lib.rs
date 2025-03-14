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

/// Based on the semantic versioning standard.
#[repr(C)]
pub struct SemanticVersion(pub u8, pub u8, pub u8);

/// Types of permissions that a plugin can request from
/// the Lotus shell upon loading of the plugin.
///
/// Guarantees the end user that a given plugin will
/// only modify certain aspects of the shell. Should be
/// displayed to the user before installing a plugin.
#[repr(C)]
pub enum PluginPermissions {
    /// Allow GUI element modification requests
    Gui,
    /// Allow theming requests
    Theme,
}

/// Lotus shell plugin metadata. Requested by the shell
/// as soon as it loads a shared object plugin file. This
/// metadata is requested and read by the shell before it
/// proceeds to load the plugin.
///
/// Allows the shell to perform checks to verify the
/// compatibility of this plugin, dependencies it may
/// require, etc.
///
/// This struct is meant to be initialized in static
/// memory on the plugin dynamic library.
pub struct PluginMetadata {
    /// Reverse DNS style
    pub id: &'static str,
    pub name: &'static str,
    /// Plugin icon stored as a raw byte array.
    /// Icon should be a PNG image with 128x128 pixel dimensions.
    pub icon: &'static [u8],
    pub version: SemanticVersion,
    /// A short description of what this plugin does
    pub description: &'static str,
    /// Lotus Plugin API version your plugin supports
    pub plugin_api_version: SemanticVersion,
    /// Required dependency plugins by plugin ID.
    /// An empty vec means no dependencies are required.
    pub dependencies: &'static [&'static str],
    /// Shell plugin API permissions required by this plugin
    pub permissions: &'static [PluginPermissions],
}

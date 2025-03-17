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

pub use libc;

#[cfg(feature = "plugin")]
pub mod ffi;
#[cfg(feature = "loader")]
pub mod loader;

/// Based on the semantic versioning standard.
#[derive(Clone, Copy)]
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
    /// Plugin icon stored as a raw byte array.
    /// Icon should be a PNG image with 128x128 pixel dimensions.
    pub icon: &'static [u8],
    pub version: SemanticVersion,
    /// Lotus Plugin API version your plugin supports
    pub plugin_api_version: SemanticVersion,
    /// Required dependency plugins by plugin ID.
    /// An empty vec means no dependencies are required.
    pub dependencies: &'static [&'static str],
    /// Shell plugin API permissions required by this plugin
    pub permissions: &'static [PluginPermissions],
}

/// Safe Rust function interface that is 1:1 with the C FFI that is generated
/// by the [`generate_plugin_ffi`] macro.
pub trait LotusPlugin
where
    Self: 'static,
{
    /// Returns the Lotus plugin API version supported by this plugin.
    ///
    /// This is the very first function call made by the shell as soon as
    /// the plugin dynamic library is loaded to memory. This version match
    /// ensures that all symbols requested by the shell will be found.
    ///
    fn get_plugin_api_version() -> SemanticVersion;

    /// Gets a pointer to this plugin's metadata struct.
    ///
    /// Returns a pointer with a static lifetime to the struct that is initialized
    /// in static memory.
    ///
    fn get_plugin_metadata() -> &'static PluginMetadata;

    /// Called from the shell upon a change in the current language translation.
    ///
    /// This allows the plugin to react to this locale change event and update the
    /// translations for all its widgets.
    ///
    fn update_locale(lang: String) -> bool;
}

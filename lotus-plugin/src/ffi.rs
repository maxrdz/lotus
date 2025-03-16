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

//! Contains the macro export for generating the FFI for a plugin
//! dynamic library using the C ABI.

/// Generates the C FFI for this plugin.
///
/// This means mapping the generated FFI to the plugin struct's implementation
/// of the [`super::LotusPlugin`] trait.
///
#[macro_export]
macro_rules! generate_plugin_ffi {
    ($plugin:ident) => {
        /// Returns the Lotus plugin API version supported by this plugin.
        ///
        /// This is the very first function call made by the shell as soon as
        /// the plugin dynamic library is loaded to memory. This version match
        /// ensures that all symbols requested by the shell will be found.
        ///
        #[unsafe(no_mangle)]
        pub extern "C" fn lotus_get_plugin_api_version() -> SemanticVersion {
            $plugin::get_plugin_api_version()
        }

        /// Gets a pointer to this plugin's metadata struct.
        ///
        /// Returns a pointer with a static lifetime to the struct that is initialized
        /// in static memory.
        ///
        #[unsafe(no_mangle)]
        pub extern "C" fn lotus_get_plugin_metadata() -> &'static PluginMetadata {
            $plugin::get_plugin_metadata()
        }

        /// Called from the shell upon a change in the current language translation.
        ///
        /// This allows the plugin to react to this locale change event and update the
        /// translations for all its widgets.
        ///
        /// # Safety
        ///
        /// - `lang` C string cannot violate the safety rules defined in the docstring
        /// for [`std::ffi::CStr::from_ptr`].
        ///
        /// # Panic
        ///
        /// If the [`std::ffi::CStr`] made from the [`*const std::ffi::c_char`] cannot
        /// be converted into a [`String`], this function will panic.
        ///
        #[unsafe(no_mangle)]
        pub extern "C" fn lotus_update_locale(lang: *const std::ffi::c_char) -> std::ffi::c_int {
            let c_str: &std::ffi::CStr = unsafe { std::ffi::CStr::from_ptr(lang) };

            match $plugin::update_locale(c_str.to_str().unwrap().to_owned()) {
                true => 1,
                false => 0,
            }
        }
    };
}

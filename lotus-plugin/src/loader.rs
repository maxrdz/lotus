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

//! Source code for loading Lotus shell plugins via dynamic libraries.

use super::*;
use libloading::{Library, Symbol};

pub struct PluginFFI<'s> {
    pub get_plugin_api_version: Symbol<'s, unsafe extern "C" fn() -> SemanticVersion>,
    pub get_plugin_metadata: Symbol<'s, unsafe extern "C" fn() -> &'static PluginMetadata>,
    pub update_locale: Symbol<'s, unsafe extern "C" fn(*const libc::c_char) -> u8>,
}

/// Represents a loaded dynamic library for providing a shell plugin.
///
/// **Unloading from memory** is not guaranteed to be safe by the compiler, as
/// the inner [`Library`] needs to have a `'static` lifetime, and freeing it
/// after leaking it would violate this promise. See [`DynamicPlugin::unload`].
pub struct DynamicPlugin {
    so: &'static libloading::Library,
    pub symbols: PluginFFI<'static>,
}

impl DynamicPlugin {
    /// Loads a dynamic library, stores it on the heap, then leaks it to give it
    /// a `'static` lifetime.
    pub fn load_so(path: &str) -> Result<DynamicPlugin, libloading::Error> {
        let so: &'static Library = Box::leak(Box::new(unsafe { Library::new(path)? }));

        Ok(Self {
            so,
            symbols: PluginFFI {
                get_plugin_api_version: unsafe { so.get(b"lotus_get_plugin_api_version")? },
                get_plugin_metadata: unsafe { so.get(b"lotus_get_plugin_metadata")? },
                update_locale: unsafe { so.get(b"lotus_update_locale")? },
            },
        })
    }

    /// Frees this plugin's dynamic library from memory.
    ///
    /// # Safety
    ///
    /// **All** `'static` references to the [`Library`] instance must be
    /// dropped **before** reconstructing the [`Box`] from a raw pointer of the
    /// [`Box<Library>`] that was leaked on the heap and dropping it to free it.
    ///
    /// Reconstructing a [`Box`] from a raw pointer violates the `'static`
    /// lifetime we promised to the compiler, so we have to manually ensure
    /// all references to that [`Library`] are kept within this struct, so
    /// this function can simply drop `Self` before dropping [`Box<Library>`].
    ///
    pub unsafe fn unload(self) {
        let so_ptr: *mut Library = self.so as *const Library as *mut Library;
        drop(self);
        drop(unsafe { Box::from_raw(so_ptr) });
    }
}

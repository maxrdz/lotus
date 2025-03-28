..
   This file is part of the Lotus Shell reference manual.

   Copyright (c) 2025 Max Rodriguez <me@maxrdz.com>

   Permission is granted to copy, distribute and/or modify this document
   under the terms of the GNU Free Documentation License, Version 1.3
   or any later version published by the Free Software Foundation;
   with no Invariant Sections, no Front-Cover Texts, and no Back-Cover Texts.
   A copy of the license is included in the section entitled "GNU
   Free Documentation License".

.. _main-page:

Lotus Shell Reference Manual
============================

.. admonition:: Copying Conditions

   The Lotus Shell documentation is released under the
   :ref:`GNU Free Documentation License <license>`.

.. warning::

   The Lotus Shell is still under heavy development and is **not**
   yet ready for use in a mobile device.

Welcome to the Lotus shell plugin documentation. Lotus is a free and
open source touchscreen graphical shell for Linux mobile devices.

Project Mission
---------------

To deliver a lightweight, modular, touchscreen-friendly shell for GNU/Linux
mobile devices that provides the **bare minimum set of features** to use your
mobile device while providing a modular system for extending the shell. The
stock shell should consume the **least possible amount of system resources**.
(CPU time, memory, etc.) The user experience should be **modern, respectful,
and elegant**.

Project Requirements
--------------------

- Support the **Wayland** display server protocol.
- Make use of **Freedesktop** (XDG) standards.
- Support for **internationalization** and **accessibility**.
- Shell should be **modular** in the sense that it can be extended by third party software.
- Stock shell should have **zero integration with any existing desktop environment ecosystem**.
- Support **plugins** that can be loaded at runtime as dynamic libraries.
- Provide a **plugin API** that exposes a stable FFI (C ABI) via **cdylib** crates.
- **Rollback mechanism** for hot reloading plugins at runtime without leaving orphaned changes.

Lotus uses Git_ for version control and Meson_ as the build system.

.. _Rust: https://www.rust-lang.org/
.. _Git: https://git-scm.com/
.. _Meson: https://mesonbuild.com/

The manual is divided up into sections, which are listed below.
You can at any time use the sidebar on the left side to navigate
between the different sections and their contained pages.

.. toctree::
   :maxdepth: 2

   license
   conduct
   glossary

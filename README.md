<div align="center">
    <img src="./data/icons/hicolor/scalable/apps/icon.svg">
    <h1>Lotus</h1>
    <p>
        The <b>L</b>ightweight <b>O</b>pen Source <b>T</b>ouchscreen
        <b>U</b>ser <b>S</b>hell
    </p>
    <a href="https://gitlab.com/maxrdz/lotus/-/pipelines/latest"><img src="https://gitlab.com/maxrdz/lotus/badges/master/pipeline.svg" alt="Pipeline" /></a>
</div>

## Project Mission

To deliver a lightweight, modular, touchscreen-friendly shell for GNU/Linux
mobile devices that provides the **bare minimum set of features** to use your
mobile device (launch apps, simple dashboard for clock and battery info, etc.)
while providing a modular system for extending the shell. The stock shell
should consume the **least possible amount of system resources**. (CPU time,
memory, etc.)

The user experience should be **modern, respectful, and elegant**. This
includes **internationalization** support. The stock shell should have **zero
integration with any existing desktop environment ecosystem**. (GNOME, KDE,
COSMIC, etc.) The mobile shell should make use of Freedesktop (XDG) standards.

## Building

Lotus uses Git for version control and Meson+Ninja as the build system.
The quickest way to build for release is to do the following:

### Getting the source

```sh
git clone https://gitlab.com/maxrdz/lotus.git
cd lotus/
```

### Dependencies

- [ModemManager](https://gitlab.freedesktop.org/mobile-broadband/ModemManager/)
- [feedbackd](https://source.puri.sm/Librem5/feedbackd)

To build Lotus, run the following in the repo root directory:

```sh
meson setup build
meson compile -C build
```

You can append the `-Dprofile=debug` argument to build for debug.

## Installing

To install a build of Lotus, run:

```sh
meson install -C build
```

## Cross Compiling

PLEASE install cross-rs via:

```sh
cargo install cross --git https://github.com/cross-rs/cross
```

The cross project is in a weird state where it doesn't have much motivation
and/or time to cut a release, so you need to pull from the main branch to
get a lot of bug fixes since the 'latest' release as of December 2024.

Then, you can run:

```sh
meson setup build -Dtarget=aarch64-unknown-linux-gnu
meson compile -C build
```

## Copyright and Software License

Copyright (c) 2025 Max Rodriguez <me@maxrdz.com>

"Lotus" can be found at https://gitlab.com/maxrdz/lotus

"Lotus" is distributed under the terms of the GNU General Public
License, either version 3.0 or, at your option, any later
version WITHOUT ANY WARRANTY. You can read the full copy of
the software license in the COPYING file.

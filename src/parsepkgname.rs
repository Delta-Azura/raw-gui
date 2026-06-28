// RAW-GUI is a simple GUI interface for raw written in Rust using ICED.
//    Copyright (C) 2026  Alexis/Delta-Azura

//    This program is free software; you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation; either version 2 of the License, or
//    (at your option) any later version.

//    This program is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.

//    You should have received a copy of the GNU General Public License along
//    with this program; if not, write to the Free Software Foundation, Inc.,
//    51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

use anyhow::Context;

pub fn parsepkgname(index: String) -> Result<String> {
    let name = index.split_once("/Pkgfile").map(|(name, _)| name).context("Failed to use index line")?.rsplit_once("/").map(|(_, name)| name).context("Failed to get package name")?;
    return Ok(name.to_string());
}
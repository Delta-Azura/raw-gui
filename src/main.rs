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

use iced::widget::{button, column, container, row, scrollable, text, Button};
use iced::{Background, Border, Color, Element, Length, Theme};
use std::fs;
use std::process::Command;
use std::path::Path;
use std::env;
use iced::widget::text_input;
use iced::Task;
use raw::changelog;
mod parsepkgname;
use parsepkgname::parsepkgname;

#[derive(Debug, Clone)]
pub enum Message {
    Install, 
    Select(String),
    Uninstall, 
    Upgrade,
    Search(String),
    UpgradeDone,
    Loaded(Result<Data, String>),
}


struct App {
    packages: Vec<String>,
    selected: Option<String>,
    pkglistwhole: Vec<String>, 
    search: String, 
    status: String, 
    statusapp: String,
    toupgrade: Vec<String>,
    pkgnum: String,
}



impl Default for App {
    fn default() -> Self {
        let toupgrade = changelog().unwrap();
        let index = fs::read_to_string("/var/cache/index.raw");
        let pkglist = index.lines().collect();
        let pkglistwhole = Vec::new();
        for i in pkglist.iter() {
            let name = parsepkgname(i)unwrap_or_else("Failed to get package name")?;
            pkglistwhole.push(name);
        }
        let packages: fs::read_dir("/var/lib/pkg/DB/")
            .unwrap()
            .filter_map(|e| Some(e.ok()?.file_name().read_to_string_lossy().to_string())).collect();
        let pkgnum = packages.len();
        Self {
            pkgnum,
            packages,
            pkglistwhole,
            search: Vec::new(),
            selected: None, 
            status: String::new(),
            statusapp: String::new(),
            toupgrade,
        }
    }
    
}

fn main() -> iced::Result {
    iced::run(App::update, App::view)
    println!("Hello, world!");
}

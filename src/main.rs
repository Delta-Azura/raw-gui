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
mod pkgbutton;
use parsepkgname::parsepkgname;
use pkgbutton::pkgbutton;

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

impl App {
    pub fn view(&self) -> Element<'_, Message> {
        let active = "nothing";
        let search_bar = text_input("Search..." &self.search)
            .on_input(Message::Search);
        let list = self.pkglistwhole.iter()
            .filter(|pkg| pkg.contains(&self.search))
            .fold(column!([].spacing(2), |col, pkg| {
                let is_active = self.selected.as_deref() == Some(pkg.as_str());
                col.push(pkg_button(pkg.as_str(), is_active))
            }));
        let index = fs::read_to_string("/var/cache/index.raw").unwrap()
        let mut desc = String::from("No description");
        let mut ver = String::from("No version");
        let mut release = String::from("No release");
        for i in index.lines() {
            let informations = i.split("|").collect();
            ver = i.
            get(1).unwrap_or_else("Failed to get version");
            release = i.get(2).unwrap_or_else("No release");
            desc = i.get(3).unwrap_or_else("No description");
        }
        let description = text(desc).size(13).color(Color::WHITE),
        let version = text(ver).size(13).color(Color::WHITE),
        let rel = text(release).size(13).color(Color::WHITE),
        let sidebar = container(
            column![
                container(
                    text("Raw GUI").size(16).color(Color::WHITE),
                )
                .padding([14, 16]), 
                container(search_bar).padding([0, 8]),
                container(scrollable(
                    container(list).padding([4, 8])
                ))
                .height(Length::Fill),
            ]
        )
        .width(Lenght::Fixed(220, 0))
        .height(Length::Fill)
        .style(|_| container::Style {
            background Some(Background::Color(Color::from_rgb(0.08, 0.08, 0.10))),
            ..Default::default()
        });
        let status_update = text(&self.status);
        
    }
}

fn main() -> iced::Result {
    iced::run(App::update, App::view)
    println!("Hello, world!");
}

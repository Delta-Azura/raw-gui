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
use std::env;
use iced::widget::text_input;
use iced::Task;
use raw::changelog;
mod parsepkgname;
mod pkgbutton;
use parsepkgname::parsepkgname;
use crate::pkgbutton::pkg_button;
use iced::widget::{button, text, Button};
use anyhow::Context;

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
        let pkglist = index.unwrap().lines().collect();
        let pkglistwhole = Vec::new();
        for i in pkglist.iter() {
            let name = parsepkgname(i).unwrap_or_else("Failed to get package name")?;
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
        let search_bar = text_input("Search...", &self.search)
            .on_input(Message::Search);

        let list = self.pkglistwhole.iter()
            .filter(|pkg| pkg.contains(&self.search))
            .fold(column![].spacing(2), |col, pkg| {
                let is_active = self.selected.as_deref() == Some(pkg.as_str());
                col.push(pkg_button(pkg.as_str(), is_active))
            });

        let index = fs::read_to_string("/var/cache/index.raw").unwrap();
        let mut desc = String::from("No description");
        let mut ver = String::from("No version");
        let mut release = String::from("No release");

        for line in index.lines() {
            let parts: Vec<&str> = line.split('|').collect();
            ver = parts.get(1).unwrap_or(&"No version").to_string();
            release = parts.get(2).unwrap_or(&"No release").to_string();
            desc = parts.get(3).unwrap_or(&"No description").to_string();
        }

        let description = text(desc).size(13).color(Color::WHITE);
        let version = text(ver).size(13).color(Color::WHITE);
        let rel = text(release).size(13).color(Color::WHITE);

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
        .width(Length::Fixed(220.0))
        .height(Length::Fill)
        .style(|_| container::Style {
            background: Some(Background::Color(Color::from_rgb(0.08, 0.08, 0.10))),
            ..Default::default()
        });

        let status_update = text(&self.status);

        let upgrade_btn: iced::widget::Button<'_, Message> = button(
            text("Upgrade").size(13).color(Color::WHITE)
        )
        .on_press(Message::Upgrade)
        .style(|_theme: &iced::Theme, _status| button::Style {
            background: Some(Background::Color(Color::from_rgb(0.18, 0.35, 0.64))),
            border: Border { radius: 6.0.into(), ..Default::default() },
            text_color: Color::WHITE,
            ..Default::default()
        });

        let detail_content: Element<'_, Message> = match &self.selected {
            Some(pkg) => {
                let status_text: iced::widget::Text<'_, iced::Theme>;
                if self.statusapp.is_empty() {
                    status_text = if self.packages.contains(pkg) {
                        text("Installed").size(13).color(Color::from_rgb(0.11, 0.62, 0.46))
                    } else {
                        text("Not installed").size(12).color(Color::from_rgb(0.5, 0.5, 0.5))
                    };
                } else {
                    status_text = text(self.statusapp.clone()).size(12).color(Color::from_rgb(0.11, 0.62, 0.46));
                }

                let button_install_uninstall = if self.statusapp.is_empty() {
                    if self.packages.contains(pkg) {
                        button(text("Désinstaller").size(13).color(Color::WHITE))
                            .on_press(Message::Uninstall)
                            .style(|_theme, _status| button::Style {
                                background: Some(Background::Color(Color::from_rgb(0.64, 0.18, 0.18))),
                                border: Border { radius: 6.0.into(), ..Default::default() },
                                text_color: Color::WHITE,
                                ..Default::default()
                            })
                    } else {
                        button(text("Installer").size(13).color(Color::WHITE))
                            .on_press(Message::Install)
                            .style(|_theme, _status| button::Style {
                                background: Some(Background::Color(Color::from_rgb(0.64, 0.18, 0.18))),
                                border: Border { radius: 6.0.into(), ..Default::default() },
                                text_color: Color::WHITE,
                                ..Default::default()
                            })
                    }
                } else {
                    if self.statusapp.starts_with("Installed") {
                        button(text("Désinstaller").size(13).color(Color::WHITE))
                            .on_press(Message::Uninstall)
                            .style(|_theme, _status| button::Style {
                                background: Some(Background::Color(Color::from_rgb(0.64, 0.18, 0.18))),
                                border: Border { radius: 6.0.into(), ..Default::default() },
                                text_color: Color::WHITE,
                                ..Default::default()
                            })
                    } else {
                        button(text("Installer").size(13).color(Color::WHITE))
                            .on_press(Message::Install)
                            .style(|_theme, _status| button::Style {
                                background: Some(Background::Color(Color::from_rgb(0.64, 0.18, 0.18))),
                                border: Border { radius: 6.0.into(), ..Default::default() },
                                text_color: Color::WHITE,
                                ..Default::default()
                            })
                    }
                };

                column![
                    text(pkg.as_str()).size(22).color(Color::WHITE),
                    status_text,
                    description,
                    version,
                    rel,
                    button_install_uninstall,
                    container(text("")).height(Length::Fixed(20.0)),
                ]
                .spacing(8)
                .into()
            },
            None => text("Sélectionne un paquet")
                .size(14)
                .color(Color::from_rgb(0.4, 0.4, 0.45))
                .into(),
        };

        let detail = container(detail_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(24)
            .style(|_| container::Style {
                background: Some(Background::Color(Color::from_rgb(0.11, 0.11, 0.13))),
                ..Default::default()
            });

        row![sidebar, detail, upgrade_btn, status_update]
            .height(Length::Fill)
            .into()
    }
}


pub fn update(&mut self, message: Message) -> Task<Message> {
    Message::Search(query) => { self.search = query; Task::none() }
    Message::Select(pkg) => { self.selected = Some(pkg); Task::none() }
    Message::Install => {
        if let Some(pkg) = &self.selected {
            install(pkg).ok();
            self.statusapp = String::from("Installed");
            Task::none()
        }
    }
    Message::Uninstall => {
        if let Some(pkg) = &self.selected {
            remove(pkg).ok();
            self.statusapp = String::from("Not installed");
            Task::none()
        }
    }
    Message::Upgrade => {
        Task::perform(
            async {
                raw::upgrade().await
            }, 
            |_| Message::UpgradeDone,
        )
    }
    Message::UpgradeDone => {
        self.status = String::from("Upgrade terminé");
        Task::none()
    }
}

fn main() -> iced::Result {
    iced::run(App::update, App::view);
    println!("Hello, world!");
}

// CARDS-GUI is a simple GUI interface for cards written in Rust using ICED.
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

#[derive(Debug, Clone)]
pub enum Message {
    Install,
    Select(String),
    Uninstall,
    Upgrade,
}

struct App {
    packages: Vec<String>,
    selected: Option<String>,
    pkglistwhole: Vec<String>,
}

impl Default for App {
    fn default() -> Self {
        let home = std::env::var("HOME").unwrap();
        if !Path::new(&format!("{}/tmp", home)).exists() {
            fs::create_dir(&format!("{}/tmp", home)).unwrap();
        } else {
            fs::remove_dir_all(format!("{}/tmp", home)).unwrap();
            fs::create_dir(format!("{}/tmp", home)).unwrap();
        }
        env::set_current_dir(format!("{}/tmp", home)).unwrap();
        download();
        let pkgrepo_one = fs::read(format!("{}/tmp/.REPO.0", home)).unwrap();
        let pkgrepo_one = String::from_utf8_lossy(&pkgrepo_one);
        let mut pkglistwhole = Vec::new();
        for i in pkgrepo_one.lines() {
            if i.starts_with("@") {
                let pkgname = i.split_once("@").map(|(_, pkg)| pkg).unwrap().split_once(".").map(|(pkg, _)| pkg).unwrap().to_string();
                if pkglistwhole.contains(&pkgname.to_string()) {
                    continue
                } else {
                    pkglistwhole.push(pkgname.to_string());
                }
            }
        }
        let pkgrepo_one = fs::read(format!("{}/tmp/.REPO.1", home)).unwrap();
        let pkgrepo_one = String::from_utf8_lossy(&pkgrepo_one);
        for i in pkgrepo_one.lines() {
            if i.starts_with("@") {
                let pkgname = i.split_once("@").map(|(_, pkg)| pkg).unwrap().split_once(".").map(|(pkg, _)| pkg).unwrap();
                if pkglistwhole.contains(&pkgname.to_string()) {
                    continue
                } else {
                    pkglistwhole.push(pkgname.to_string());
                }
            }
        }
        let pkgrepo_one = fs::read(format!("{}/tmp/.REPO.2", home)).unwrap();
        let pkgrepo_one = String::from_utf8_lossy(&pkgrepo_one);
        for i in pkgrepo_one.lines() {
            if i.starts_with("@") {
                let pkgname = i.split_once("@").map(|(_, pkg)| pkg).unwrap().split_once(".").map(|(pkg, _)| pkg).unwrap();
                if pkglistwhole.contains(&pkgname.to_string()) {
                    continue
                } else {
                    pkglistwhole.push(pkgname.to_string());
                }
            }
        }
        let pkgrepo_one = fs::read(format!("{}/tmp/.REPO.3", home)).unwrap();
        let pkgrepo_one = String::from_utf8_lossy(&pkgrepo_one);
        for i in pkgrepo_one.lines() {
            if i.starts_with("@") {
                let pkgname = i.split_once("@").map(|(_, pkg)| pkg).unwrap().split_once(".").map(|(pkg, _)| pkg).unwrap();
                if pkglistwhole.contains(&pkgname.to_string()) {
                    continue
                } else {
                    pkglistwhole.push(pkgname.to_string());
                }
            }
        }
        let pkgrepo_one = fs::read(format!("{}/tmp/.REPO.4", home)).unwrap();
        let pkgrepo_one = String::from_utf8_lossy(&pkgrepo_one);
        for i in pkgrepo_one.lines() {
            if i.starts_with("@") {
                let pkgname = i.split_once("@").map(|(_, pkg)| pkg).unwrap().split_once(".").map(|(pkg, _)| pkg).unwrap();
                if pkglistwhole.contains(&pkgname.to_string()) {
                    continue
                } else {
                    pkglistwhole.push(pkgname.to_string());
                }
            }
        }
        Self {
            packages: fs::read_dir("/var/lib/pkg/DB")
                .unwrap()
                .filter_map(|e| Some(e.ok()?.file_name().to_string_lossy().to_string()))
                .collect(),
            selected: None,
            pkglistwhole,
        }
    }
}

fn pkg_button(label: &str, active: bool) -> Button<'_, Message> {
    button(
        text(label).size(13)
    )
    .on_press(Message::Select(label.to_string()))
    .width(Length::Fill)
    .style(move |_theme, status| {
        let bg = if active {
            Color::from_rgb(0.11, 0.62, 0.46)
        } else if matches!(status, button::Status::Hovered) {
            Color::from_rgb(0.15, 0.15, 0.17)
        } else {
            Color::from_rgb(0.10, 0.10, 0.12)
        };
        button::Style {
            background: Some(Background::Color(bg)),
            text_color: if active {
                Color::WHITE
            } else {
                Color::from_rgb(0.75, 0.75, 0.78)
            },
            border: Border {
                radius: 6.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    })
}

impl App {
    pub fn view(&self) -> Element<'_, Message> {
        let active = "nothing";
        let list = self.pkglistwhole.iter().fold(column![].spacing(2), |col, pkg| {
            let active = self.selected.as_deref() == Some(pkg.as_str());
            col.push(pkg_button(pkg.as_str(), active))
        });
        let home = std::env::var("HOME").unwrap();
        let pkgrepo = fs::read_dir(format!("{}/tmp", home)).unwrap().filter_map(|e| e.ok());
        let mut desc = String::from("No description");
        let mut ver = String::from("No version");
        let mut pack = String::from("No packager");
        for i in pkgrepo {
            let i = i.file_name().to_str().unwrap().to_string();
            if i.starts_with(".REPO") {
                let repofile = fs::read(format!("{}/tmp/{}", home, i)).unwrap();
                let repofile = String::from_utf8_lossy(&repofile);
                let target = format!("@{}", self.selected.as_deref().unwrap_or(""));
                let mut in_pkg = false;
                println!("target: {:?}", target);
                for l in repofile.lines().take(10) {
                    println!("{:?}", l);
                }
                for l in repofile.lines() {
                    if l.starts_with(&target) {
                        in_pkg = true;
                        continue;
                    }
                    if in_pkg {
                        if l.starts_with('@') { break; }
                        if l.starts_with('D') {
                            desc = l.strip_prefix('D').unwrap_or("").to_string();
                            desc = format!("Description : {}", desc);
                        }
                        if l.starts_with('V') {
                            ver = l.strip_prefix('V').unwrap_or("").to_string();
                            ver = format!("Version : {}", ver);
                        }
                        if l.starts_with('P') {
                            pack = l.strip_prefix('P').unwrap_or("").to_string();
                            pack = format!("Packager : {}", pack);
                        }

                    }
                }
            }
        }
        let packager = text(pack).size(13).color(Color::WHITE);
        let version = text(ver).size(13).color(Color::WHITE);
        let description = text(desc).size(13).color(Color::WHITE);
        let sidebar = container(
            column![
                container(
                    text("Cards GUI").size(16).color(Color::WHITE)
                )
                .padding([14, 16]),
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
        let upgrade_btn: iced::widget::Button<'_, Message> = button(text("Upgrade").size(13).color(Color::WHITE))
            .on_press(Message::Upgrade)
            .style(|_theme: &iced::Theme, _status| button::Style {
                background: Some(Background::Color(Color::from_rgb(0.18, 0.35, 0.64))),
                border: Border { radius: 6.0.into(), ..Default::default() },
                text_color: Color::WHITE,
                ..Default::default()
            });
        let detail_content: Element<'_, Message> = match &self.selected {
            Some(pkg) => {
                let status_text = if self.packages.contains(pkg) {
                    text("Installé").size(12).color(Color::from_rgb(0.11, 0.62, 0.46))
                } else {
                    text("Non installé").size(12).color(Color::from_rgb(0.5, 0.5, 0.5))
                };
                let button_install_uninstall = if self.packages.contains(pkg) {
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
                };
                column![
                    text(pkg.as_str()).size(22).color(Color::WHITE),
                    status_text,
                    description,
                    version,
                    packager,
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

        row![sidebar, detail, upgrade_btn]
            .height(Length::Fill)
            .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Select(pkg) => self.selected = Some(pkg),
            Message::Install => {
                if let Some(pkg) = &self.selected {
                    Command::new("pkexec").args(["cards", "remove", pkg]).status().ok();
                }
            }
            Message::Uninstall => {
                if let Some(pkg) = &self.selected {
                    Command::new("pkexec").args(["cards", "remove", pkg]).status().ok();
                }
            }
            Message::Upgrade => {
                Command::new("pkgexec").args(["cards", "upgrade"]).status().ok();
            }
        }
    }
}

fn main() -> iced::Result {
    iced::run(App::update, App::view)
}



fn download() {
    for (i, collection) in ["base", "cli", "cli-extra", "gui", "gui-extra"].iter().enumerate() {
        let cmd = format!("wget -O .REPO.{} https://downloads.nutyx.org/x86_64/systemd/{}/.REPO", i, collection);
        Command::new("bash").args(["-c", &cmd]).status();
    }
}
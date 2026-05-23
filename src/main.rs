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
        if !Path::new("/home/alexis/tmp/").exists() {
            fs::create_dir("/home/alexis/tmp/").unwrap();
        } else {
            fs::remove_dir_all("/home/alexis/tmp/").unwrap();
            fs::create_dir("/home/alexis/tmp/").unwrap();
        }
        env::set_current_dir("/home/alexis/tmp/").unwrap();
        //download();
        let pkgrepo_one = fs::read("/home/alexis/tmp/.REPO").unwrap();
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
        let pkgrepo_one = fs::read("/home/alexis/tmp/.REPO.1").unwrap();
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
        let pkgrepo_one = fs::read("/home/alexis/tmp/.REPO.2").unwrap();
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
        let pkgrepo_one = fs::read("/home/alexis/tmp/.REPO.3").unwrap();
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
        let pkgrepo_one = fs::read("/home/alexis/tmp/.REPO.4").unwrap();
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
        let pkgrepo = fs::read_dir("/home/alexis/tmp").unwrap().filter_map(|e| e.ok());
        let description: iced::widget::Button<'_, Message> = button(text("No description").size(13).color(Color::WHITE));
        let desc = "No description";
        for i in pkgrepo {
            let i = i.file_name().to_str().unwrap().to_string();
            let desc = if i.starts_with(".REPO") {
                println!("ddddddddd");
                let repofile = fs::read(format!("/home/alexis/tmp/{}", i)).unwrap();
                let repofile = String::from_utf8_lossy(&repofile);
                println!("@{}", self.selected.as_deref().unwrap_or(""));
                let desc = repofile
                    .lines()
                    .skip_while(|l| !l.starts_with(&format!("@{}", active)))
                    .find(|l| l.strip_prefix("D").is_some()).and_then(|l| l.strip_prefix('D')).unwrap_or("No description");
            };
        }
        let description: iced::widget::Button<'_, Message> = button(text(desc).size(13).color(Color::WHITE));

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
                column![
                    text(pkg.as_str()).size(22).color(Color::WHITE),
                    status_text,
                    description,
                    container(text("")).height(Length::Fixed(20.0)),
                    button(text("Désinstaller").size(13).color(Color::WHITE))
                        .on_press(Message::Uninstall)
                        .style(|_theme, _status| button::Style {
                            background: Some(Background::Color(Color::from_rgb(0.64, 0.18, 0.18))),
                            border: Border { radius: 6.0.into(), ..Default::default() },
                            text_color: Color::WHITE,
                            ..Default::default()
                        }),
                    button(text("Installer").size(13).color(Color::WHITE))
                        .on_press(Message::Install)
                        .style(|_theme, _status| button::Style {
                            background: Some(Background::Color(Color::from_rgb(0.64, 0.18, 0.18))),
                            border: Border { radius: 6.0.into(), ..Default::default() },
                            text_color: Color::WHITE, 
                            ..Default::default()
                        }),
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
    Command::new("wget").arg("https://downloads.nutyx.org/x86_64/systemd/base/.REPO").status();
    Command::new("wget").arg("https://downloads.nutyx.org/x86_64/systemd/cli-extra/.REPO").status();
    Command::new("wget").arg("https://downloads.nutyx.org/x86_64/systemd/cli/.REPO").status();
    Command::new("wget").arg("https://downloads.nutyx.org/x86_64/systemd/gui/.REPO").status();
    Command::new("wget").arg("https://downloads.nutyx.org/x86_64/systemd/gui-extra/.REPO").status();
}
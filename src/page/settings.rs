use iced::{text_input, Column, Command, Container, Element, Length, Row, Text, TextInput};

use crate::message::Message;
use crate::ui::style::Theme;

#[derive(Debug, Clone)]
pub struct SettingsPage {
    background_input: text_input::State,
    foreground_input: text_input::State,
    background_color: String,
    foreground_color: String,
}

impl Default for SettingsPage {
    fn default() -> Self {
        Self::new()
    }
}

impl SettingsPage {
    pub fn new() -> SettingsPage {
        SettingsPage {
            background_input: text_input::State::new(),
            foreground_input: text_input::State::new(),
            background_color: String::new(),
            foreground_color: String::new(),
        }
    }

    pub fn update(&mut self, msg: Message) -> Command<Message> {
        match msg {
            Message::ForegroundChanged(value) => {
                self.foreground_color = value;
                Command::none()
            }
            Message::BackgroundChanged(value) => {
                self.background_color = value;
                Command::none()
            }
            _ => Command::none(),
        }
    }

    pub fn view(&mut self, theme: &Theme) -> Element<Message> {
        let settings_container = Row::new()
            .spacing(15)
            .push(Text::new("Customize Theme:").size(16))
            .push(
                Column::new()
                    .spacing(15)
                    .push(
                        Row::new().push(
                            TextInput::new(
                                &mut self.background_input,
                                "Enter Background Color (RGB Hex, i.e, #00000F)",
                                &self.background_color,
                                Message::BackgroundChanged,
                            )
                            .size(16)
                            .width(Length::Units(450))
                            .padding(15)
                            .style(*theme)
                            .on_submit(Message::LoadCustomBackground),
                        ),
                    )
                    .push(
                        Row::new().push(
                            TextInput::new(
                                &mut self.foreground_input,
                                "Enter Foreground Color (RGB Hex, i.e, #F100FF)",
                                &self.foreground_color,
                                Message::ForegroundChanged,
                            )
                            .size(16)
                            .width(Length::Units(450))
                            .padding(15)
                            .style(*theme)
                            .on_submit(Message::LoadCustomForeground),
                        ),
                    ),
            );

        Container::new(settings_container)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .center_x()
            .style(*theme)
            .into()
    }
}

use iced::{button, Button, Column, Container, Element, Length, Row, Text};

use crate::message::Message;
use crate::page::PageType;
use crate::ui::style::Theme;

#[derive(Debug, Clone)]
pub struct PageButton {
    label_text: String,
    button_state: button::State,
    page_type: PageType,
    is_disabled: bool,
}

#[derive(Debug, Clone)]
pub struct Toolbar {
    buttons: Vec<PageButton>,
    pub active_page: PageType,
    theme_button: button::State,
}

fn inverse(theme: &Theme) -> Theme {
    match theme {
        Theme::Dark => Theme::Light,
        Theme::Light => Theme::Dark,
    }
}

fn conditional_inverse(theme: &Theme, selected: bool) -> Theme {
    if selected {
        inverse(theme)
    } else {
        *theme
    }
}

impl Toolbar {
    pub fn new() -> Toolbar {
        let buttons = vec![
            PageButton {
                label_text: "Dashboard".to_string(),
                button_state: button::State::new(),
                page_type: PageType::Dashboard,
                is_disabled: true,
            },
            PageButton {
                label_text: "Feed".to_string(),
                button_state: button::State::new(),
                page_type: PageType::Feed,
                is_disabled: true,
            },
            PageButton {
                label_text: "Publish".to_string(),
                button_state: button::State::new(),
                page_type: PageType::Publish,
                is_disabled: false,
            },
            PageButton {
                label_text: "View".to_string(),
                button_state: button::State::new(),
                page_type: PageType::View,
                is_disabled: false,
            },
            PageButton {
                label_text: "Site".to_string(),
                button_state: button::State::new(),
                page_type: PageType::Site,
                is_disabled: true,
            },
            PageButton {
                label_text: "Settings".to_string(),
                button_state: button::State::new(),
                page_type: PageType::Settings,
                is_disabled: true,
            },
        ];

        Toolbar {
            buttons,
            active_page: PageType::Publish,
            theme_button: button::State::new(),
        }
    }

    pub fn view(&mut self, theme: &Theme) -> Element<Message> {
        let Toolbar {
            buttons,
            active_page,
            theme_button,
        } = self;

        Container::new(
            buttons
                .into_iter()
                .fold(Row::new(), |row, page_button| {
                    row.push(if page_button.is_disabled {
                        Column::new().padding(2).push(
                            Button::new(
                                &mut page_button.button_state,
                                Text::new(page_button.label_text.clone()).size(16),
                            )
                            .style(*theme),
                        )
                    } else {
                        Column::new().padding(2).push(
                            Button::new(
                                &mut page_button.button_state,
                                Text::new(page_button.label_text.clone()).size(16),
                            )
                            .style(conditional_inverse(
                                theme,
                                *active_page == page_button.page_type,
                            ))
                            .on_press(Message::PageChanged(page_button.page_type.clone())),
                        )
                    })
                })
                .push(Column::new().width(Length::Fill)) // spacer column
                .push(
                    Button::new(theme_button, Text::new("Day/Night").size(16))
                        .style(*theme)
                        .on_press(Message::ThemeChanged(inverse(theme))),
                ),
        )
        .padding(10)
        .style(*theme)
        .into()
    }
}
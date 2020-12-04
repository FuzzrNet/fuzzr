use iced::{Application, Column, Command, Container, Element, Length, Settings};

mod data;
mod message;
mod page;
mod ui;

use page::PageType;

use page::content::ContentPage;
use page::dashboard::DashPage;
use page::feed::FeedPage;
use page::publish::PublishPage;
use page::testing::TestingPage;

use message::Message;
use ui::page_selector::PageSelector;

pub fn main() -> iced::Result {
    pretty_env_logger::init();

    Fuzzr::run(Settings::default())
}

#[derive(Debug, Clone)]
struct Pages {
    dash: DashPage,
    feed: FeedPage,
    publish: PublishPage,
    content: ContentPage,
    testing: TestingPage,
}

#[derive(Debug, Clone)]
pub struct Fuzzr {
    pages: Pages, // All pages in the app
    current_page: PageType,
    page_buttons: PageSelector,
}

impl Application for Fuzzr {
    type Executor = iced::executor::Default;
    type Message = message::Message;
    type Flags = ();

    fn new(_flags: ()) -> (Fuzzr, Command<Message>) {
        let pages = Pages {
            dash: DashPage::new(),
            feed: FeedPage::new(),
            publish: PublishPage::new(),
            content: ContentPage::new(),
            testing: TestingPage::new(),
        };

        (
            Fuzzr {
                pages,
                current_page: PageType::Dashboard,
                page_buttons: PageSelector::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Fuzzr".to_string()
    }

    fn update(&mut self, event: Message) -> Command<Message> {
        self.current_page = match event {
            Message::PageChanged(page_type) => page_type,
            _ => {
                // Page not found
                PageType::Dashboard
            }
        };

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let page: Element<_> = match self.current_page {
            PageType::Dashboard => self.pages.dash.view(),
            PageType::Feed => self.pages.feed.view(),
            PageType::Publish => self.pages.publish.view(),
            PageType::Content => self.pages.content.view(),
            PageType::Testing => self.pages.testing.view(),
        };

        let content: Element<_> = Column::new()
            .max_width(540)
            .spacing(20)
            .padding(20)
            .push(self.page_buttons.view())
            .push(page)
            .into();

        Container::new(content)
            .height(Length::Fill)
            .center_y()
            .into()
    }
}

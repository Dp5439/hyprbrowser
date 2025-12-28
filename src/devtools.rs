use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Element, Length};

pub struct DevTools {
    pub console: Vec<String>,
    pub network_log: Vec<(String, u16)>,
    pub active_tab: DevToolsTab,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DevToolsTab {
    Console,
    Network,
    Elements,
}

impl DevTools {
    pub fn new() -> Self {
        Self {
            console: vec!["DevTools Console Ready".to_string()],
            network_log: vec![],
            active_tab: DevToolsTab::Console,
        }
    }

    pub fn log(&mut self, message: String) {
        self.console.push(format!(
            "[{}] {}",
            chrono::Local::now().format("%H:%M:%S"),
            message
        ));
    }

    pub fn log_network(&mut self, url: String, status: u16) {
        self.network_log.push((url, status));
    }
}

fn txt<'a>(s: impl ToString) -> Element<'a, crate::Message> {
    text(s).into()
}

pub fn view(devtools: &DevTools) -> Element<crate::Message> {
    let tabs = row![
        button(container(txt("Console")).padding(8)),
        button(container(txt("Network")).padding(8)),
        button(container(txt("Elements")).padding(8)),
    ]
    .spacing(5);

    let content: Element<_> = match devtools.active_tab {
        DevToolsTab::Console => {
            let mut col = column![].spacing(4).padding(8);
            for msg in &devtools.console {
                col = col.push(text(msg).size(11));
            }
            scrollable(col).into()
        }

        DevToolsTab::Network => {
            let mut col = column![text("Network Requests").size(12)]
                .spacing(4)
                .padding(8);

            for (url, status) in &devtools.network_log {
                col = col.push(
                    row![
                        text(url).size(10),
                        text(status.to_string()).size(10)
                    ]
                    .spacing(8),
                );
            }

            scrollable(col).into()
        }

        DevToolsTab::Elements => {
            scrollable(container(txt("Element Inspector")).padding(8)).into()
        }
    };

    container(
        column![tabs, content]
            .spacing(8)
            .width(Length::Fill)
            .height(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Shrink)
    .padding(8)
    .into()
}

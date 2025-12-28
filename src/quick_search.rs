use iced::widget::*;
use iced::Element;

pub fn view(input: &str) -> Element<crate::Message> {
    let search_result = if let Ok(result) = evalexpr::eval(input) {
        format!("= {}", result)
    } else {
        format!("Search: {}", input)
    };

    container(
        iced::widget::column![
            text("Quick Search").size(14),
            text_input("Search or calculate...", input)
                .on_input(crate::Message::QuickSearchInput)
                .padding(8)
                .size(12),
            text(search_result).size(12),
            button(text("Search"))
                .padding(8)
                .on_press(crate::Message::QuickSearchExecute),
        ]
        .spacing(8)
        .padding(12),
    )
    .width(iced::Length::Fill)
    .height(iced::Length::Shrink)
    .padding(8)
    .into()
}

/// Detect if input is a calculation or search query
pub fn is_calculation(input: &str) -> bool {
    evalexpr::eval(input).is_ok()
}

/// Perform calculation and return result
pub fn calculate(input: &str) -> Option<String> {
    evalexpr::eval(input).ok().map(|r| r.to_string())
}

/// Generate Google search URL
pub fn google_search_url(query: &str) -> String {
    let encoded = urlencoding::encode(query);
    format!("https://www.google.com/search?q={}", encoded)
}

use iced::{
    alignment,
    widget::{button, column as ui_column, pick_list, row, text, vertical_rule},
    Element, Length,
};

use crate::{language::ALL_LANGUAGES, my_theme::MyTheme, App, Message};

pub fn side_column<'a>(app: &'a App) -> Element<'a, Message> {
    let icon = if app.is_side_column_opened {
        " < "
    } else {
        " > "
    };

    let toggle_btn = button(icon).on_press(Message::ToggleSideColumnClicked);

    let column = if app.is_side_column_opened {
        let app_name = text(app.language.app_name)
            .width(Length::Fill)
            .height(Length::Units(60))
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Top)
            .size(30);

        let new_file_button = button(app.language.new_file_button_text)
            .padding(10)
            .width(Length::Fill)
            .on_press(Message::WithUnsavedFileDialog(Box::new(
                Message::NewFileClicked,
            )));

        let open_file_button = button(app.language.open_file_button_text)
            .padding(10)
            .width(Length::Fill)
            .on_press(Message::WithUnsavedFileDialog(Box::new(
                Message::OpenFileClicked,
            )));

        let save_file_button = button(app.language.save_file_button_text)
            .padding(10)
            .width(Length::Fill)
            .on_press(Message::SaveFileClicked);

        let save_file_as_button = button(app.language.save_file_as_button_text)
            .padding(10)
            .width(Length::Fill)
            .on_press(Message::SaveFileAsClicked);

        let theme_pick_list = pick_list(
            MyTheme::all(app.language).to_vec(),
            Some(MyTheme::from_theme(app.theme.clone(), app.language)),
            Message::ThemeChanged,
        )
        .padding(10)
        .width(Length::Fill);

        let language_picker_label = text("Language")
            .height(Length::Fill)
            .vertical_alignment(alignment::Vertical::Bottom);

        let language_pick_list = pick_list(
            &ALL_LANGUAGES[..],
            Some(app.language),
            Message::LanguageChanged,
        )
        .padding(10)
        .width(Length::Fill);

        ui_column![
            app_name,
            new_file_button,
            open_file_button,
            save_file_button,
            save_file_as_button,
            language_picker_label,
            language_pick_list,
            app.language.theme_picker_label,
            theme_pick_list
        ]
        .width(Length::Units(280))
        .spacing(10)
        .padding(40)
    } else {
        ui_column![]
    };

    row![column, vertical_rule(0), toggle_btn].into()
}

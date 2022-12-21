use std::{fmt::Display, str::FromStr};

use iced::widget::{button, row, text_input, Row};
use num_traits::Num;

pub fn number_input<
    'a,
    Message: 'a + Clone,
    N: 'a + Num + FromStr + Display + Copy + PartialOrd,
>(
    placeholder: &str,
    value: N,
    min: Option<N>,
    on_number_change: &'a impl Fn(N) -> Message,
) -> Row<'a, Message> {
    let change_number = move |s: String| {
        let num: N = if s.len() == 0 {
            min.unwrap_or(min.unwrap_or(N::zero()))
        } else {
            let n = s.parse().unwrap_or(value);
            match min {
                Some(m) if n < m => m,
                _ => n,
            }
        };

        on_number_change(num)
    };

    let dec = match min {
        Some(m) if value <= m => m,
        _ => value - N::one(),
    };

    let initial_cursor_position_input = text_input(placeholder, &format!("{value}"), change_number)
        .padding(10)
        .size(20);

    let initial_cursor_position_increment_button = button("+")
        .padding(10)
        .on_press(on_number_change(value + N::one()));

    let initial_cursor_position_decrement_button =
        button("-").padding(10).on_press(on_number_change(dec));

    row![
        initial_cursor_position_input,
        initial_cursor_position_increment_button,
        initial_cursor_position_decrement_button,
    ]
    .spacing(10)
}

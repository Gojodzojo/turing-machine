use std::{fmt::Display, str::FromStr};

use iced::{
    widget::{self, button, row, text_input},
    Element,
};
use iced_lazy::Component;
use num_traits::Num;

pub struct NumericInput<'a, Message, N, F: Fn(N) -> Message> {
    placeholder: &'a str,
    value: N,
    on_number_change: F,
    can_be_negative: bool,
}

pub fn numeric_input<'a, Message, N, F: Fn(N) -> Message>(
    placeholder: &'a str,
    value: N,
    on_number_change: F,
) -> NumericInput<'a, Message, N, F> {
    NumericInput {
        placeholder,
        value,
        on_number_change,
        can_be_negative: true,
    }
}

impl<'a, Message, N, F: Fn(N) -> Message> NumericInput<'a, Message, N, F> {
    pub fn can_be_negative(mut self, value: bool) -> Self {
        self.can_be_negative = value;
        self
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    InputChanged(String),
    IncrementPressed,
    DecrementPressed,
}

pub enum DisplayedValue {
    Blank,
    Minus,
    ActualValue,
}

pub struct State {
    displayed_value: DisplayedValue,
}

impl Default for State {
    fn default() -> Self {
        Self {
            displayed_value: DisplayedValue::ActualValue,
        }
    }
}

impl<'a, Message, N: Num + FromStr + Display + Copy, F: Fn(N) -> Message, Renderer>
    Component<Message, Renderer> for NumericInput<'a, Message, N, F>
where
    Renderer: iced_native::text::Renderer + 'static,
    Renderer::Theme:
        widget::button::StyleSheet + widget::text_input::StyleSheet + widget::text::StyleSheet,
{
    type State = State;
    type Event = Event;

    fn update(&mut self, state: &mut Self::State, event: Event) -> Option<Message> {
        let new_val = match event {
            Event::IncrementPressed => self.value + N::one(),
            Event::DecrementPressed => self.value - N::one(),
            Event::InputChanged(s) => 'b: {
                if s.len() == 0 {
                    state.displayed_value = DisplayedValue::Blank;
                } else if self.can_be_negative && s == "-" {
                    state.displayed_value = DisplayedValue::Minus;
                } else if let Ok(new_val) = s.parse() {
                    state.displayed_value = DisplayedValue::ActualValue;
                    break 'b new_val;
                }

                return None;
            }
        };

        Some((self.on_number_change)(new_val))
    }

    fn view(&self, state: &Self::State) -> Element<Event, Renderer> {
        let input_value = match state.displayed_value {
            DisplayedValue::Blank => "".to_string(),
            DisplayedValue::Minus => "-".to_string(),
            DisplayedValue::ActualValue => self.value.to_string(),
        };

        let input = text_input(self.placeholder, &input_value, Event::InputChanged)
            .padding(10)
            .size(20);

        let increment_button = button("+").padding(10).on_press(Event::IncrementPressed);
        let decrement_button = button("-").padding(10).on_press(Event::DecrementPressed);

        row![input, increment_button, decrement_button,]
            .spacing(10)
            .into()
    }
}

impl<'a, Message, N: 'a + Num + FromStr + Display + Copy, F: 'a + Fn(N) -> Message, Renderer>
    From<NumericInput<'a, Message, N, F>> for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: 'static + iced_native::text::Renderer,
    Renderer::Theme:
        widget::button::StyleSheet + widget::text_input::StyleSheet + widget::text::StyleSheet,
{
    fn from(numeric_input: NumericInput<'a, Message, N, F>) -> Self {
        iced_lazy::component(numeric_input)
    }
}

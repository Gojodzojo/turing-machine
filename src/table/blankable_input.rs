use iced::{
    widget::{self, text_input::Id as TxtId, text_input as txt_input},
    Element, Length,
};
use iced_lazy::Component;
use iced_native::widget::Id;

pub struct BlankableInput<'a, Message, F: Fn(String) -> Message> {
    placeholder: &'a str,
    value: String,
    focused_widget: &'a Option<Id>,
    on_change: F,
    width: Length,
}

pub fn blankable_input<'a, Message, F: Fn(String) -> Message>(
    placeholder: &'a str,
    value: String,
    focused_widget: &'a Option<Id>,
    width: Length,
    on_change: F,
) -> BlankableInput<'a, Message, F> {
    BlankableInput {
        placeholder,
        value,
        on_change,
        focused_widget,
        width,
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    InputChanged(String),
}

pub enum DisplayedValue {
    Blank,
    ActualValue,
}

pub struct State {
    displayed_value: DisplayedValue,
    input_id: TxtId,
}

impl Default for State {
    fn default() -> Self {
        Self {
            displayed_value: DisplayedValue::ActualValue,
            input_id: TxtId::unique(),
        }
    }
}

impl<'a, Message, F: Fn(String) -> Message, Renderer> Component<Message, Renderer>
    for BlankableInput<'a, Message, F>
where
    Renderer: iced_native::text::Renderer + 'static,
    Renderer::Theme:
        widget::button::StyleSheet + widget::text_input::StyleSheet + widget::text::StyleSheet,
{
    type State = State;
    type Event = Event;

    fn update(&mut self, state: &mut Self::State, event: Event) -> Option<Message> {
        match event {
            Event::InputChanged(s) => {
                if s.len() == 0 {
                    state.displayed_value = DisplayedValue::Blank;
                    return None;
                }

                state.displayed_value = DisplayedValue::ActualValue;
                return Some((self.on_change)(s));
            }
        };
    }

    fn view(&self, state: &Self::State) -> Element<Event, Renderer> {
        let input_value = match &self.focused_widget {
            Some(focused_widget) if *focused_widget == Id::from(state.input_id.clone()) => {
                match state.displayed_value {
                    DisplayedValue::Blank => "".to_string(),
                    DisplayedValue::ActualValue => self.value.to_string(),
                }
            }
            _ => self.value.to_string(),
        };

        txt_input(self.placeholder, &input_value, Event::InputChanged)
            .width(self.width)
            .id(state.input_id.clone())
            .into()
    }
}

impl<'a, Message, F: 'a + Fn(String) -> Message, Renderer> From<BlankableInput<'a, Message, F>>
    for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: 'static + iced_native::text::Renderer,
    Renderer::Theme:
        widget::button::StyleSheet + widget::text_input::StyleSheet + widget::text::StyleSheet,
{
    fn from(numeric_input: BlankableInput<'a, Message, F>) -> Self {
        iced_lazy::component(numeric_input)
    }
}

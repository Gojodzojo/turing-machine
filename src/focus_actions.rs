use iced::Command;
use iced_native::{
    command,
    widget::{
        operation::{Focusable, Outcome},
        Id, Operation,
    },
};

use crate::Message;

pub fn get_focused_element_id() -> Command<Message> {
    return Command::single(command::Action::Widget(
        iced_native::widget::Action::new(find_focused()).map(Message::FocusedWidget),
    ));
}

pub fn focus_next() -> Command<Message> {
    return Command::single(command::Action::Widget(
        iced_native::widget::Action::new(focus_next_internal()).map(Message::FocusedWidget),
    ));
}

fn find_focused() -> impl Operation<Option<Id>> {
    struct FindFocused {
        focused: Option<Id>,
    }

    impl Operation<Option<Id>> for FindFocused {
        fn focusable(&mut self, state: &mut dyn Focusable, id: Option<&Id>) {
            if state.is_focused() && id.is_some() {
                self.focused = id.cloned();
            }
        }

        fn container(
            &mut self,
            _id: Option<&Id>,
            operate_on_children: &mut dyn FnMut(&mut dyn Operation<Option<Id>>),
        ) {
            operate_on_children(self)
        }

        fn finish(&self) -> Outcome<Option<Id>> {
            Outcome::Some(self.focused.clone())
        }
    }

    FindFocused { focused: None }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Count {
    /// The index of the current focused widget, if any.
    focused: Option<usize>,

    /// The total amount of focusable widgets.
    total: usize,
}

fn count<T, O>(f: fn(Count) -> O) -> impl Operation<T>
where
    O: Operation<T> + 'static,
{
    struct CountFocusable<O> {
        count: Count,
        next: fn(Count) -> O,
    }

    impl<T, O> Operation<T> for CountFocusable<O>
    where
        O: Operation<T> + 'static,
    {
        fn focusable(&mut self, state: &mut dyn Focusable, _id: Option<&Id>) {
            if state.is_focused() {
                self.count.focused = Some(self.count.total);
            }

            self.count.total += 1;
        }

        fn container(
            &mut self,
            _id: Option<&Id>,
            operate_on_children: &mut dyn FnMut(&mut dyn Operation<T>),
        ) {
            operate_on_children(self)
        }

        fn finish(&self) -> Outcome<T> {
            Outcome::Chain(Box::new((self.next)(self.count)))
        }
    }

    CountFocusable {
        count: Count::default(),
        next: f,
    }
}

fn focus_next_internal() -> impl Operation<Option<Id>> {
    struct FocusNext {
        count: Count,
        current: usize,
        focused: Option<Id>,
    }

    impl Operation<Option<Id>> for FocusNext {
        fn focusable(&mut self, state: &mut dyn Focusable, id: Option<&Id>) {
            match self.count.focused {
                None if self.current == 0 => {
                    state.focus();
                    self.focused = id.cloned()
                }
                Some(focused) if focused == self.current => state.unfocus(),
                Some(focused) if focused + 1 == self.current => {
                    state.focus();
                    self.focused = id.cloned()
                }
                _ => {}
            }

            self.current += 1;
        }

        fn container(
            &mut self,
            _id: Option<&Id>,
            operate_on_children: &mut dyn FnMut(&mut dyn Operation<Option<Id>>),
        ) {
            operate_on_children(self)
        }

        fn finish(&self) -> Outcome<Option<Id>> {
            Outcome::Some(self.focused.clone())
        }
    }

    count(|count| FocusNext {
        count,
        current: 0,
        focused: None,
    })
}

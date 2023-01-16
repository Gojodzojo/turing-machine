use iced_native::widget::{Operation, Id, operation::{Focusable, Outcome}};

pub fn find_focused() -> impl Operation<Option<Id>> {
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
use vizia::prelude::*;

pub fn build_no_tabs_open_state(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, Localized::new("no_tabs_open")).class("empty-state");
    })
    .padding(Pixels(12.0));
}

use vizia::prelude::*;

pub fn build_text_info_sidebar(
    cx: &mut Context,
    path_display: String,
    editor_text: Signal<String>,
) {
    VStack::new(cx, move |cx| {
        Label::new(cx, Localized::new("info")).class("section-title");

        HStack::new(cx, |cx| {
            Label::new(cx, Localized::new("path"));
            Label::new(cx, path_display.clone())
                .width(Stretch(1.0))
                .text_wrap(true);
        })
        .height(Auto)
        .gap(Pixels(8.0));

        HStack::new(cx, move |cx| {
            Label::new(cx, Localized::new("length"));
            Binding::new(cx, editor_text, move |cx| {
                Label::new(cx, editor_text.get().len().to_string());
            });
        })
        .height(Auto)
        .gap(Pixels(8.0));
    })
    .class("info-sidebar")
    .width(Stretch(1.0))
    .height(Stretch(1.0))
    .gap(Pixels(8.0));
}

use crate::ui::commands::resources;
use crate::ui::events::ResourceEvent;
use crate::ui::model::UiModel;
use vizia::prelude::*;

impl UiModel {
    pub(crate) fn handle_resource_events(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|resource_event: &ResourceEvent, _| match resource_event {
            ResourceEvent::CacheImageResource(path, image_bytes) => {
                resources::cache_image_resource(cx, path.clone(), image_bytes.clone());
            }
        });
    }
}

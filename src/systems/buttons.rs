// extern crate amethyst;
// use amethyst::ecs::{System, World, Read, Write, SystemData};
// use amethyst::core::SystemDesc;
// use amethyst::shrev::{EventChannel, ReaderId};
// use amethyst::ui::{UiEvent, UiEventType};
//
// pub struct SimpleButtonSystem{
//     reader_id: ReaderId<UiEvent>,
// }
//
// impl<'s> System<'s> for SimpleButtonSystem {
//     type SystemData = Read<'s, EventChannel<UiEvent>>;
//
//
//     fn run(&mut self, (events, transforms, mut texts): Self::SystemData) {
//         for event in events.read(&mut self.reader_id) {
//             let button_text = texts.get_mut(event.target).unwrap();
//
//             match event.event_type {
//                 UiEventType::HoverStart => {
//                     button_text.color = [1.0, 1.0, 1.0, 1.0];
//                 },
//                 UiEventType::HoverStop  => {
//                     button_text.color = [1.0, 1.0, 1.0, 0.5];
//                 },
//                 _ => {},
//             }
//         }
//     }
// }
//
// impl SimpleButtonSystem {
//     pub fn new(reader_id: ReaderId<UiEvent>) -> Self {
//         Self {
//             reader_id,
//         }
//     }
// }
//
// #[derive(Default, Debug)]
// pub struct SimpleButtonSystemDesc;
//
// impl<'a, 'b> SystemDesc<'a, 'b, SimpleButtonSystem> for SimpleButtonSystemDesc {
//     fn build(self, world: &mut World) -> SimpleButtonSystem {
//         let mut event_channel = <Write<EventChannel<UiEvent>>>::fetch(world);
//         let reader_id = event_channel.register_reader();
//
//         SimpleButtonSystem::new(reader_id)
//     }
// }
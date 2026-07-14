use bevy::ecs::message::MessageCursor;
use bevy::prelude::*;
use bevy_test_helper::BevyTestHelperPlugin;
use bevy_test_helper::event::{DirectEvents, TestEvent1};
use bevy_test_helper::prelude::TriggerExtension;
use bevy_test_helper::resource::DirectResourceControl;
use bevy_test_helper::resource::bool::BoolExtension;
use bevy_test_helper::resource::count::Count;
use bevy_test_helper::system::SystemExt;

#[test]
fn smoke() {
    let mut app = App::new();
    app.add_plugins(BevyTestHelperPlugin);
    app.update();

    let mut cursor = MessageCursor::<TestEvent1>::default();
    app.assert_message_not_comes(&mut cursor);
    app.write(TestEvent1);
    app.assert_message_comes(&mut cursor);

    assert!(app.is_bool_false());
    app.set_bool(true);
    assert!(app.is_bool_true());

    app.resource_mut::<Count>().increment();
    app.assert_resource(1, |c: &Count| c.0);

    app.trigger(TestEvent1);

    let count = app.run_system_once(|count: Res<Count>| count.0);
    assert_eq!(count, 1);
}

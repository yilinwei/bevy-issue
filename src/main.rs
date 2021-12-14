use bevy::{core::CorePlugin, math::vec3, prelude::*, transform::TransformPlugin};


fn transform() -> Transform {
    Transform::from_translation(vec3(10.0, 10.0, 10.0))
}

fn spawn(mut commands: Commands) {
    let parent = commands
        .spawn()
        .insert_bundle((
            transform(),
            GlobalTransform::default(),
            // If you uncomment this line, the behaviour is as expected
            // Children::with(&[])
        ))
        .id();
    commands.spawn().insert_bundle((
        Transform::default(),
        GlobalTransform::default(),
        Parent(parent),
    ));
}

fn query_children(children: Query<(Entity, &Children)>) {
    for (e, c) in children.iter() {
        // The `Children` structure is /always/ created
        println!("{:?} {:?}", e, c);
    }
}

fn query_transforms(transforms: Query<(Entity, &Transform, &GlobalTransform)>) {
    let m = transform().compute_matrix();
    for (e, t, g) in transforms.iter() {
        if g.compute_matrix() != m {
            println!("unexpected matrix: {:?} {:?} {:?}", e, t, g);
        }
    }
}

fn main() {
    App::new()
        .add_plugin(CorePlugin::default())
        .add_plugin(TransformPlugin::default())
        .add_startup_system(spawn)
        .add_system(query_transforms)
        .add_system(query_children)
        .run();
}

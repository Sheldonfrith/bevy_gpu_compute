use bevy::{
    app::{App, Plugin, Startup},
    prelude::Commands,
};

use super::resources::{IterationSpace, WorkgroupSizes};

pub struct GpuAccBevyIterSpaceDependentResourcesPlugin;

impl Plugin for GpuAccBevyIterSpaceDependentResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GpuAccBevyIterSpaceDependentResourcesPlugin)
            .add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(IterationSpace { x: 1, y: 1, z: 1 });
    commands.insert_resource(WorkgroupSizes::one_d());
}

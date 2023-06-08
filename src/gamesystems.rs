
use bevy::{
    prelude::*, 
    //render::camera::OrthographicProjection
};
use bevy_editor_pls::prelude::*;


pub struct GameSystemsPlugin;

#[derive(Component)]
pub struct MainCamera;

 



impl Plugin for GameSystemsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    // resolution: (800., 460.).into(), //2850, 0
                    // position: WindowPosition::At((-810,550).into()),
                    resolution: (1000., 600.).into(), //2850, 0
                    position: WindowPosition::At((2550,0).into()),

                    //resolution: (1280., 720.).into(),
                    //position: (0, 0).into(),
                    // fill the entire browser window
                    fit_canvas_to_parent: true,
                    // don't hijack keyboard shortcuts like F5, F6, F12, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()))
        .add_plugin(EditorPlugin::default())
        .add_startup_system(setup)
        .insert_resource(ClearColor(Color::rgb(0.53, 0.53, 0.53)));

        
    }
}


fn setup(mut commands: Commands, /* asset_server: Res<AssetServer> */) {

    commands.spawn((Camera2dBundle{
        projection: OrthographicProjection {
            scale: 1.,
            
            ..default()
        },
        ..default()   
    }, MainCamera));

    //commands.spawn(LdtkWorldBundle {
     //   ldtk_handle: asset_server.load("maps/level1.ldtk"),
    //    ..Default::default()
    //});
}


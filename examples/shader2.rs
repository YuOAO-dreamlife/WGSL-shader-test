use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
};

// 匯入測試benchmark用的函式庫
use bevy::app::AppExit;
use std::time::Duration;

fn main() {
    let mut app = App::default();

    // 添加背景顏色
    app.insert_resource(ClearColor(Color::hex("071f3c").unwrap()));

    // 添加 Bevy的預設插件
    app.add_plugins(DefaultPlugins);

    // 設置 `Shader2Material`
    app.add_plugin(MaterialPlugin::<Shader2Material>::default());

    // 開始執行時的動作
    app.add_startup_system(setup);

    // 為了方便測試 benchmark，在此設定經過1秒後會自動關閉 Bevy軟體
    app.add_system(exit_system);

    // 執行
    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<Shader2Material>>,
) {
    // 視角相機
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // 方塊 + 使用移動方塊系統
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: materials.add(Shader2Material {}),
        ..default()
    });

    // benchmark測試用程式碼
    commands.spawn((FuseTime {
        timer: Timer::new(Duration::from_secs(1), TimerMode::Once),
    },));
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
struct Shader2Material {}

impl Material for Shader2Material {
    fn fragment_shader() -> ShaderRef {
        "shaders\\shader2.wgsl".into()
    }
}

// 以下為 benchmark測試用程式碼
#[derive(Component)]
struct FuseTime {
    timer: Timer,
}

fn exit_system(mut exit: EventWriter<AppExit>, mut q: Query<&mut FuseTime>, time: Res<Time>) {
    for mut fuse_timer in q.iter_mut() {
        // 時間倒數。
        fuse_timer.timer.tick(time.delta());

        // 若時間到則關閉視窗。
        if fuse_timer.timer.finished() {
            exit.send(AppExit);
        }
    }
}

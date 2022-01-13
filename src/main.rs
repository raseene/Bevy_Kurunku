
use	bevy::prelude::*;
use	bevy_kira_audio::AudioPlugin;

mod	mouse;
mod	fade;
mod	loading;
mod	game;


pub const	FRAME_RATE: f32		= 30.0;				// フレームレート
pub const	SCREEN_WIDTH: f32	= 640.0;			// 画面の大きさ
pub const	SCREEN_HEIGHT: f32	= 360.0;

pub const	SCREEN_SCALE: f32 = if cfg!(target_arch = "wasm32") {1.2} else {1.0};


#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState
{
	LOADING,			// アセット先読み
	GAME,				// ゲーム
	NEXT,				// 次のゲーム
}


/****************
    共通データ
 ****************/
pub struct	CommonData
{
	hi_score: u32,		// ハイスコア
}


/************
    メイン
 ************/
fn	main()
{
	let mut	app = App::new();

	app.insert_resource(WindowDescriptor
	{
		title: "くるんくる～ぱ".to_string(),
		width: SCREEN_WIDTH/SCREEN_SCALE,
		height: SCREEN_HEIGHT/SCREEN_SCALE,
		resizable: false,
		#[cfg(all(target_arch = "wasm32", not(debug_assertions)))]
		canvas: Some("canvas".to_string()),
		..Default::default()
	});
	app.add_plugins(DefaultPlugins);
	app.add_plugin(AudioPlugin);										// オーディオ

	app.add_startup_system(setup);										// 初期化
	app.insert_resource(mouse::Mouse::new())							// マウス管理
		.add_system(mouse::input);
	app.insert_resource(CommonData{hi_score: 0});						// 共通データ

	app.add_plugin(loading::LoadingPlugin);								// アセットの先読み
	app.add_plugin(game::game::GamePlugin);								// ゲーム

	app.add_system_set(
		SystemSet::on_enter(AppState::NEXT)
			.with_system(
				|mut _state: ResMut<State<crate::AppState>>|
				{
					_state.set(AppState::GAME).unwrap();
				}
			)
	);

	app.add_state(AppState::LOADING)									// ローディング画面開始
		.run();
}

/************
    初期化
 ************/
fn	setup(mut _commands: Commands, mut _materials: ResMut<Assets<ColorMaterial>>)
{
	let mut	_camera = OrthographicCameraBundle::new_2d();
	_camera.transform.scale.x = SCREEN_SCALE;
	_camera.transform.scale.y = SCREEN_SCALE;
	_commands.spawn_bundle(_camera);									// カメラ設定

	fade::Fade::init(&mut _commands, &mut _materials);					// フェード処理
}

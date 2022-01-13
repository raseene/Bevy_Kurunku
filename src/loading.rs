
use	bevy::
{
	prelude::*,
	asset::LoadState,
};
use	bevy_kira_audio::AudioSource;


pub struct	AssetsLoading(Vec<HandleUntyped>);

#[derive(Component)]
struct	NowLoading;

/******************
    データ先読み
 ******************/
fn	setup(mut _commands: Commands, _asset: Res<AssetServer>)
{
	let mut	_vec = Vec::new();

	let	_font: Handle<Font>= _asset.load("font/FiraSans-Bold.ttf");		// フォント
	_vec.push(_font.clone_untyped());

	let	sprite =							// スプライト
	[
		"sprite/board.png",
		"sprite/ball.png",
		"sprite/pivot.png",
		"sprite/timer.png",
		"sprite/number_s.png",
		"sprite/number_m.png",
		"sprite/game_over.png",
	];
	 for _file in sprite.iter() {
		let	_data: Handle<Image> = _asset.load(*_file);
		_vec.push(_data.clone_untyped());
	}

	let	audio =								// オーディオ
	[
		"audio/bgm_game.ogg",
		"audio/se_rot.ogg",
		"audio/se_erase.ogg",
	];
	 for _file in audio.iter() {
		let	_data: Handle<AudioSource> = _asset.load(*_file);
		_vec.push(_data.clone_untyped());
	}

	_commands.insert_resource(AssetsLoading(_vec));				// リソース


	for i in 0..2 {												// "Now Loading..."
		_commands
			.spawn_bundle(Text2dBundle
			{
				text: Text::with_section("Now Loading...",
											TextStyle
											{
												font: _font.clone(),
												font_size: 48.0,
												color: Color::rgb(1.0, 1.0, 1.0),
											},
											TextAlignment
											{
												vertical: VerticalAlign::Center,
												horizontal: HorizontalAlign::Center,
											}),
				transform: Transform::from_translation(Vec3::new((i*500) as f32, 0.0, 20.0)),
				..Default::default()
			})
			.insert(NowLoading);
	}
}

/******************
    読み込み待ち
 ******************/
fn	update(_asset: Res<AssetServer>, _loading: Res<AssetsLoading>, mut _state: ResMut<State<crate::AppState>>,
					mut _query: Query<&mut Transform, With<NowLoading>>)
{
	for mut _trans in _query.iter_mut() {										// 文字列スクロール
		_trans.translation.x -= 2.0;
		if _trans.translation.x < -500.0 {
			_trans.translation.x = 500.0;
		}
	}

	match _asset.get_group_load_state(_loading.0.iter().map(|h| h.id)) {
		LoadState::Loaded => {													// 読み込み完了
			_state.set(crate::AppState::GAME).unwrap();
		}
		_ => {}
	}
}

/**********
    終了
 **********/
fn	exit(mut _commands: Commands, _query: Query<Entity, With<Text>>)
{
	for _entity in _query.iter() {												// "Now Loading..."消去
		_commands.entity(_entity).despawn();
	}
	_commands.remove_resource::<AssetsLoading>();								// リソース削除
}


/****************
    プラグイン
 ****************/
pub struct	LoadingPlugin;

impl Plugin for LoadingPlugin
{
	fn	build(&self, _app: &mut App)
	{
		_app.add_system_set(
			SystemSet::on_enter(crate::AppState::LOADING)
				.with_system(setup)
		);
		_app.add_system_set(
			SystemSet::on_update(crate::AppState::LOADING)
				.with_system(update)
		);
		_app.add_system_set(
			SystemSet::on_exit(crate::AppState::LOADING)
				.with_system(exit)
		);
	}
}

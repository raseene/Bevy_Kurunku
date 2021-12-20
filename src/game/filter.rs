
use bevy::prelude::*;


/**********************
    タイムアップ警告
 **********************/
pub struct	WarningFilter;

impl WarningFilter
{
	/************
	    初期化
	 ************/
	pub fn	init(_commands: &mut Commands, _materials: &mut ResMut<Assets<ColorMaterial>>)
	{
		_commands.spawn_bundle(SpriteBundle
		{
			sprite: Sprite::new(Vec2::new(344.0, 360.0)),
			transform: Transform::from_translation(Vec3::new(-148.0, 0.0, 0.5)),
			material: _materials.add(Color::rgba(1.0, 0.0, 0.0, 0.0).into()),
			..Default::default()
		})
		.insert(WarningFilter);
	}

	/*******************************
	    設定
			引数	_aplha = α値
	 *******************************/
	pub fn	set(&mut self, _alpha: f32, _mat: &mut ColorMaterial)
	{
		_mat.color.set_a(_alpha);
	}
}


/**********************
    ゲームオーバー時
 **********************/
pub struct	OverFilter
{
	cnt: isize,				// カウンタ
}

impl OverFilter
{
	/************
	    初期化
	 ************/
	pub fn	init(_commands: &mut Commands, _materials: &mut ResMut<Assets<ColorMaterial>>)
	{
		_commands.spawn_bundle(SpriteBundle
		{
			sprite: Sprite::new(Vec2::new(344.0, 360.0)),
			transform: Transform::from_translation(Vec3::new(-148.0, 0.0, 4.5)),
			material: _materials.add(Color::rgba(0.0, 0.0, 0.0, 0.0).into()),
			..Default::default()
		})
		.insert(OverFilter{cnt: 0});
	}

	/**********
	    稼働
	 **********/
	pub fn	update(&mut self, _mat: &mut ColorMaterial)
	{
		if self.cnt < 24 {
			self.cnt += 1;
			_mat.color.set_a((self.cnt as f32)/72.0);
		}
	}
}


use bevy::prelude::*;


/**********************
    タイムアップ警告
 **********************/
#[derive(Component)]
pub struct	WarningFilter;

impl WarningFilter
{
	/************
	    初期化
	 ************/
	pub fn	init(_commands: &mut Commands)
	{
		_commands.spawn_bundle(SpriteBundle
		{
			sprite: Sprite
			{
				color: Color::rgba(1.0, 0.0, 0.0, 0.0),
				custom_size: Some(Vec2::new(344.0, 360.0)),
				..Default::default()
			},
			transform: Transform::from_translation(Vec3::new(-148.0, 0.0, 0.5)),
			..Default::default()
		})
		.insert(WarningFilter);
	}

	/*******************************
	    設定
			引数	_aplha = α値
	 *******************************/
	pub fn	set(&mut self, _alpha: f32, _spr: &mut Sprite)
	{
		_spr.color.set_a(_alpha);
	}
}


/**********************
    ゲームオーバー時
 **********************/
#[derive(Component)]
pub struct	OverFilter
{
	cnt: isize,				// カウンタ
}

impl OverFilter
{
	/************
	    初期化
	 ************/
	pub fn	init(_commands: &mut Commands)
	{
		_commands.spawn_bundle(SpriteBundle
		{
			sprite: Sprite
			{
				color: Color::rgba(0.0, 0.0, 0.0, 0.0),
				custom_size: Some(Vec2::new(344.0, 360.0)),
				..Default::default()
			},
			transform: Transform::from_translation(Vec3::new(-148.0, 0.0, 4.5)),
			..Default::default()
		})
		.insert(OverFilter{cnt: 0});
	}

	/**********
	    稼働
	 **********/
	pub fn	update(&mut self, _spr: &mut Sprite)
	{
		if self.cnt < 24 {
			self.cnt += 1;
			_spr.color.set_a((self.cnt as f32)/72.0);
		}
	}
}

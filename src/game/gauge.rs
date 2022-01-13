
use bevy::prelude::*;

use	super::game::Gauge;


/************
    ゲージ
 ************/
#[derive(Component)]
pub struct	GaugeBar
{
	pub index: Gauge,		// 番号
	x: f32,					// 画面座標
}

impl GaugeBar
{
	/*************************************
	    初期化
			引数	_id    = ゲージ番号
					_h     = 高さ
					_pos   = 左端の座標
					_tex   = テクスチャ
					_color = 色
	 *************************************/
	pub fn	init_texture(_id: Gauge, _h: f32, _pos: Vec3, _tex: Handle<Image>,
						_commands: &mut Commands)
	{
		_commands.spawn_bundle(SpriteBundle
		{
			sprite: Sprite
			{
				custom_size: Some(Vec2::new(1.0, _h)),
				..Default::default()
			},
			texture: _tex,
			transform: Transform::from_translation(_pos),
			..Default::default()
		})
		.insert(GaugeBar{index: _id, x: _pos.x});
	}

	pub fn	init_color(_id: Gauge, _h: f32, _pos: Vec3, _color: Color,
						_commands: &mut Commands)
	{
		_commands.spawn_bundle(SpriteBundle
		{
			sprite: Sprite
			{
				color: _color,
				custom_size: Some(Vec2::new(1.0, _h)),
				..Default::default()
			},
			transform: Transform::from_translation(_pos),
			..Default::default()
		})
		.insert(GaugeBar{index: _id, x: _pos.x});
	}

	/*****************************
	    幅設定
			引数	_width = 幅
	 *****************************/
	pub fn	set(&mut self, _width: f32,
					_trans: &mut Transform)
	{
		_trans.scale.x = _width;
		_trans.translation.x = self.x + _width/2.0;
	}
}

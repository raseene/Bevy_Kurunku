
use bevy::prelude::*;

use	super::game::Gauge;


/************
    ゲージ
 ************/
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
					_color = 色
	 *************************************/
	pub fn	init(_id: Gauge, _h: f32, _pos: Vec3, _color: Handle<ColorMaterial>,
						_commands: &mut Commands)
	{
		_commands.spawn_bundle(SpriteBundle
		{
			sprite: Sprite::new(Vec2::new(0.0, _h)),
			transform: Transform::from_translation(_pos),
			material: _color,
			..Default::default()
		})
		.insert(GaugeBar{index: _id, x: _pos.x});
	}

	/*****************************
	    幅設定
			引数	_width = 幅
	 *****************************/
	pub fn	set(&mut self, _width: f32,
					_spr: &mut Sprite, _trans: &mut Transform)
	{
		_spr.size.x = _width;
		_trans.translation.x = self.x + _width/2.0;
	}
}

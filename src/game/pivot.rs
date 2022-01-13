
use bevy::prelude::*;

use	super::game;


const	PIVOT_W: f32 = 48.0;			// 回転軸のタッチ範囲
const	PIVOT_H: f32 = 54.0;


/************
    回転軸
 ************/
#[derive(Component)]
pub struct	Pivot
{
	pub	cx: usize,			// 位置
	pub	cy: usize,
	x: f32,					// 画面座標
	y: f32,
	cnt: isize,				// アニメーションカウンタ
}

impl Pivot
{
	/***********************************************
	    初期化
			引数	_cx, _cy = フィールド上の位置
	 ***********************************************/
	pub fn	init(_cx: usize, _cy: usize,
						_commands: &mut Commands,
						_tex: &Handle<Image>)
	{
		let	x = game::FIELD_X - 25.0 + game::BALL_H + (_cx as f32)*game::BALL_W;				// 画面座標
		let	y = game::FIELD_Y + 28.0 - (((_cx + 1) & 1) as f32)*game::BALL_H/2.0 + (_cy as f32)*game::BALL_H;

		_commands
			.spawn_bundle(SpriteBundle
			{
				texture: _tex.clone(),
				transform: Transform::from_translation(Vec3::new(x, y, 2.0)),
				..Default::default()
			})
			.insert(Pivot{cx: _cx, cy: _cy, x: x, y: y, cnt: 0});
	}


	/***************************************
	    範囲チェック
			引数	_x, _y = カーソル座標
			戻り値	タッチ範囲内か
	 ***************************************/
	pub fn	check_rect(&self, x: f32, y: f32) -> bool
	{
		(x >= self.x - PIVOT_W/2.0) && (x < self.x + PIVOT_W/2.0) && (y >= self.y - PIVOT_H/2.0) && (y < self.y + PIVOT_H/2.0)
	}

	/**********
	    押下
	 **********/
	pub fn	push(&mut self)
	{
		self.cnt = 4;
	}

	/**********
	    稼働
	 **********/
	pub fn	update(&mut self, _trans: &mut Transform)
	{
		if self.cnt > 0 {
			self.cnt -= 1;
			_trans.scale.x = 0.75 + (((self.cnt - 2)*(self.cnt - 2)) as f32)*0.25/4.0;
			_trans.scale.y = _trans.scale.x;
		}
	}
}

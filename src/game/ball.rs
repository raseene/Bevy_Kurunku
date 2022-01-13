
use bevy::prelude::*;

use	super::game;


/********
    球
 ********/
#[derive(Component)]
pub struct	Ball
{
	pub	cx: usize,				// 位置
	pub	cy: usize,
	x: f32,						// 画面座標
	y: f32,
	pub	color: usize,			// 色

	rot_num: usize,				// 回転場所
	rot_cnt: usize,				// 回転カウンタ
	pub	erase_cnt: isize,		// 消去カウンタ
	fall_cnt: isize,			// 落下カウンタ
	pub	wait_cnt: isize,		// 消去判定待ちカウンタ
}

impl Ball
{
	const ROT_POSITION: [[[f32; 2]; 4]; 3] =				// 回転相対座標
	[
		[[0.0, 0.0], [ 15.1,  15.4], [ 22.2,  39.2], [ 15.1,  57.4]],
		[[0.0, 0.0], [  5.5, -21.0], [ 22.2, -39.2], [ 41.2, -42.0]],
		[[0.0, 0.0], [-20.6,   5.6], [-44.4,   0.0], [-56.3, -15.4]],
	];


	/***********************************************
	    初期化
			引数	_cx, _cy = フィールド上の位置
					_color   = 色
	 ***********************************************/
	pub fn	init(_cx: usize, _cy: usize, _color: usize,
							_commands: &mut Commands,
							_tex: &Handle<TextureAtlas>)
	{
		let mut	_ball = Ball
		{
			cx: _cx,
			cy: _cy,
			x: game::FIELD_X + (_cx as f32)*game::BALL_W,
			y: game::FIELD_Y - (_cx as f32)*(game::BALL_H/2.0) + (((_cx >> 1) + _cy) as f32)*game::BALL_H,
			color: _color,

			rot_num: 0,
			rot_cnt: 0,
			erase_cnt: 0,
			fall_cnt: 0,
			wait_cnt: 0,
		};

		let mut	_spr = SpriteSheetBundle::default();
		_spr.texture_atlas = _tex.clone();
		_ball.update(&mut _spr.transform, &mut _spr.sprite, &mut _spr.visibility);
		_commands
			.spawn_bundle(_spr)
			.insert(_ball);
	}

	/***********************************
	    回転
			引数	_num   = 回転場所
					_color = 新しい色
	 ***********************************/
	pub fn	rotate(&mut self, _num: usize, _color: usize)
	{
		self.color = _color | 0x30;				// 色
		self.rot_num = _num;					// 回転場所
		self.rot_cnt = 4;						// 回転用カウンタ
	}

	/**********
	    消去
	 **********/
	pub fn	erase(&mut self, _commands: &mut Commands, _tex: &Handle<TextureAtlas>)
	{
		if self.erase_cnt != 0 {
			return;
		}
		self.erase_cnt = 0xff;
		self.wait_cnt  = 0;

		BallEffect::init(self.x, self.y,	_commands, _tex);			// エフェクト
	}

	/***********************************
	    落下
			引数	_color = 新しい色
	 ***********************************/
	pub fn	fall(&mut self, _color: usize)
	{
		self.color		= _color;
		self.rot_num	= 0;
		self.rot_cnt	= 0;
		self.erase_cnt	= 0;
		self.fall_cnt	= 0;
		self.wait_cnt	= 0;
		if (_color & 0x08) == 0 {				// 非表示
			self.fall_cnt = 4;
			self.color |= 0x30;
		}
	}

	/**************************
	    稼働
			戻り値	動作中か
	 **************************/
	pub fn	update(&mut self, _trans: &mut Transform, _spr: &mut TextureAtlasSprite, _visible: &mut Visibility) -> bool
	{
		_trans.translation.x = self.x;									// 画面座標
		_trans.translation.y = self.y;
		_trans.translation.z = 1.0;
		if (self.color & 0x08) == 0 {
			_spr.index = (self.color & 0x0f) as usize;					// 色
			_visible.is_visible = true;
		}
		else {															// 空白
			_visible.is_visible = false;
		}

		if self.rot_cnt > 0 {					// 回転中
			self.rot_cnt -= 1;
			if self.rot_cnt == 0 {
				self.color &= 0x0f;
				self.wait_cnt = -6;
			}
			else {
				_trans.translation.x += Ball::ROT_POSITION[self.rot_num][self.rot_cnt][0];
				_trans.translation.y += Ball::ROT_POSITION[self.rot_num][self.rot_cnt][1];
				_trans.translation.z = 1.5;
			}
		}
		else if self.erase_cnt > 0 {			// 消去中
			if self.erase_cnt == 0xff {
				self.erase_cnt = 12;
			}
			else {
				self.erase_cnt -= 1;
				if self.erase_cnt == 0 {
					self.color = 0x08;
				}
			}
			if self.erase_cnt % 3 == 0 {
				_visible.is_visible = false;
			}
		}
		else if self.fall_cnt > 0 {				// 落下中
			self.fall_cnt -= 1;
			if self.fall_cnt == 0 {
				self.color &= 0x0f;
				self.wait_cnt = 8;
			}
			else {
				_trans.translation.y += (self.fall_cnt as f32)*game::BALL_H/4.0;
			}
		}
		else if self.wait_cnt < 0 {				// 消去判定待ち（回転）
			self.wait_cnt += 1;
		}
		else if self.wait_cnt > 0 {				// 消去判定待ち（落下）
			self.wait_cnt -= 1;
		}
		else {
			return	false;
		}
		true
	}
}


/**********************
    球消去エフェクト
 **********************/
#[derive(Component)]
pub struct	BallEffect
{
	cnt: isize,				// カウンタ
}

impl BallEffect
{
	/***********************************
	    初期化
			引数	_x, _y = 画面座標
	 ***********************************/
	pub fn	init(_x: f32, _y: f32,
						_commands: &mut Commands,
						_tex: &Handle<TextureAtlas>)
	{
		_commands
			.spawn_bundle(SpriteSheetBundle
			{
				sprite: TextureAtlasSprite
				{
					index: 5,
					color: Color::rgba(1.0, 1.0, 1.0, 0.7),
					..Default::default()
				},
				texture_atlas: _tex.clone(),
				transform: Transform::from_translation(Vec3::new(_x, _y, 2.0)),
				..Default::default()
			})
			.insert(BallEffect{cnt: 12});
	}

	/************************
	    稼働
			戻り値	終了か
	 ************************/
	pub fn	update(&mut self, _spr: &mut TextureAtlasSprite) -> bool
	{
		self.cnt -= 1;
		_spr.color.set_a((self.cnt as f32)*0.7/12.0);

		self.cnt == 0
	}
}

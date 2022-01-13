
use bevy::prelude::*;

use	super::game::Value;


/**************
    描画数値
 **************/
#[derive(Copy, Clone, Default, Component)]
pub struct	ValueSpr
{
	pub value: u32,			// 数値
	pub target: u32,		// 目標
	base: u32,				// 基準
	cnt: isize,				// カウンタ
}

impl ValueSpr
{
	/****************************************
	    初期化
			引数	_id     = 数値番号
					_figure = 桁数
					_x, _y  = 一桁目の座標
					_w      = 数字の幅
	 ****************************************/
	pub fn	new(_id: Value, _figure: usize, mut _x: f32, _y: f32, _w: f32,
						_commands: &mut Commands,
						_tex: &Handle<TextureAtlas>) -> Self
	{
		let mut	_denom = 1;
		for _i in 0.._figure {
			NumberSpr::init(_id, _denom, _x, _y,	_commands, _tex);			// 数字スプライト
			_x -= _w;
			_denom *= 10;
		}

		ValueSpr::default()
	}

	/*****************************
	    数値設定
			引数	_val = 数値
	 *****************************/
	pub fn	set(&mut self, _val: u32) ->Self
	{
		self.value	= _val;
		self.base	= _val;
		self.target	= _val;
		self.cnt	= 0;
		*self
	}

	pub fn	set_up(&mut self, _val: u32)
	{
		self.base	= if self.cnt == 0 {self.value} else {self.target};
		self.target	= _val;
		self.cnt	= 10;
	}

	/**********
	    稼働
	 **********/
	pub fn	update(&mut self)
	{
		if self.cnt > 0 {
			self.cnt -= 1;
			self.value = (self.target*((10 - self.cnt) as u32) + self.base*(self.cnt as u32) + 9)/10;
		}
	}
}


/********************
    数字スプライト
 ********************/
#[derive(Component)]
pub struct	NumberSpr
{
	pub index: Value,		// 番号
	denominator: u32,		// 桁
}

impl NumberSpr
{
	/***********************************
	    初期化
			引数	_id    = 数値番号
					_denom = 桁
					_x, _y = 表示座標
	 ***********************************/
	pub fn	init(_id: Value, _denom: u32, _x: f32, _y: f32,
						_commands: &mut Commands,
						_tex: &Handle<TextureAtlas>)
	{
		_commands
			.spawn_bundle(SpriteSheetBundle
			{
				texture_atlas: _tex.clone(),
				visibility: Visibility{is_visible: _denom == 1},
				transform: Transform::from_translation(Vec3::new(_x, _y, 6.0)),
				..Default::default()
			})
			.insert(NumberSpr{index:_id, denominator: _denom});
	}

	/*****************************
	    数値設定
			引数	_val = 数値
	 *****************************/
	pub fn	set(&self, _val: u32, _spr: &mut TextureAtlasSprite, _visible: &mut Visibility)
	{
		let	_t = _val/self.denominator;
		_spr.index = (_t % 10) as usize;
		_visible.is_visible = (_t > 0) || (self.denominator == 1);
	}
}



/**************************
    獲得スコアエフェクト
 **************************/
#[derive(Component)]
pub struct	ScoreEffect
{
	cnt: isize,				// カウンタ
}

impl ScoreEffect
{
	/*****************************************
	    初期化
			引数	_n     = 数字
					_x, _y = 表示座標
					_cnt   = カウンタ初期値
	 *****************************************/
	pub fn	init(_n: usize, _x: f32, _y: f32, _cnt: isize,
						_commands: &mut Commands,
						_tex: &Handle<TextureAtlas>)
	{
		_commands
			.spawn_bundle(SpriteSheetBundle
			{
				sprite: TextureAtlasSprite::new(_n),
				texture_atlas: _tex.clone(),
				visibility: Visibility{is_visible: false},
				transform: Transform::from_translation(Vec3::new(_x, _y, 4.0)),
				..Default::default()
			})
			.insert(ScoreEffect{cnt: _cnt});
	}

	/****************************
	    稼働
			戻り値	終了したか
	 ****************************/
	pub fn	update(&mut self, _trans: &mut Transform, _spr: &mut TextureAtlasSprite, _visible: &mut Visibility) -> bool
	{
		self.cnt -= 1;
		_trans.translation.y += 2.0;
		_visible.is_visible = self.cnt < 24;
		_spr.color.set_a(if self.cnt < 8 {(self.cnt as f32)/8.0} else {1.0});

		self.cnt == 0
	}
}

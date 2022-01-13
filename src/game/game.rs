
use	bevy::
{
	prelude::*,
	core::FixedTimestep,
};
use	rand::random;
use	bevy_kira_audio::{Audio, AudioSource};

use	crate::mouse::Mouse;
use	crate::fade::Fade;
use	super::ball::{Ball, BallEffect};
use	super::pivot::Pivot;
use	super::gauge::GaugeBar;
use	super::number::{ValueSpr, NumberSpr, ScoreEffect};
use	super::message::{StartMessage, GameOver};
use	super::filter::{WarningFilter, OverFilter};


/*** 定数定義 *******/
pub const	FIELD_W: usize	= 7;						// フィールドの大きさ
pub const	FIELD_H: usize	= 7;
pub const	BALL_W: f32		= 50.0;						// 球の間隔
pub const	BALL_H: f32		= 56.0;
pub const	FIELD_X: f32	= -299.0;					// フィールド位置
pub const	FIELD_Y: f32	= -180.0 + BALL_H/2.0;


/*** ゲージ *******/
#[derive(Copy, Clone)]
pub enum Gauge
{
	TIME,					// 残り時間
	COMBO,					// コンボ減少
}

/*** 描画数値 *******/
#[derive(Copy, Clone)]
pub enum Value
{
	HISCORE,				// ハイスコア
	SCORE,					// スコア
	COMBO,					// コンボ数
}


/*** 状態 *******/
enum Phase
{
	READY,					// 開始待ち
	GAME,					// ゲーム中
	END,					// 終了待ち
	OVER,					// ゲームオーバー
}


/********************
    ゲーム進行処理
 ********************/
struct	GameProc
{
	phase: Phase,							// 状態
	time: u32	,							// 残り時間
	cnt: isize,								// カウンタ
	move_flag: bool,						// 球稼働中か

	combo: u32,								// コンボ数
	dec_combo: isize,
	combo_cnt: isize,						// コンボアップ用カウンタ
	push: (usize, usize),					// 前回押した回転軸
	forbidden_color: usize,					// 非選択色

	tex_ball: Handle<TextureAtlas>,			// 球用テクスチャ
	tex_number: Handle<TextureAtlas>,		// 数字エフェクト用テクスチャ
	tex_over: Handle<Image>,				// "GAME OVER"テクスチャ

	bgm_game: Handle<AudioSource>,			// ゲームBGM
	se_rot: Handle<AudioSource>,			// 回転効果音
	se_erase: Handle<AudioSource>,			// 消去効果音
}

impl GameProc
{
	/************
	    初期化
	 ************/
	fn	new(_commands: &mut Commands,
					_asset: &Res<AssetServer>,
					_tex_atlases: &mut ResMut<Assets<TextureAtlas>>) -> Self
	{
		let	_game = GameProc
		{
			phase: Phase::READY,
			time: 60*(crate::FRAME_RATE as u32),
			cnt: 0,
			move_flag: false,

			combo: 0,
			dec_combo: 0,
			combo_cnt: 0,
			push: (0xff, 0xff),
			forbidden_color: 0xff,

			tex_ball:	_tex_atlases.add(TextureAtlas::from_grid(_asset.load("sprite/ball.png"), Vec2::new(60.0, 60.0), 6, 1)),
			tex_number:	_tex_atlases.add(TextureAtlas::from_grid(_asset.load("sprite/number_s.png"), Vec2::new(18.0, 24.0), 10, 1)),
			tex_over:	_asset.load("sprite/game_over.png"),

			bgm_game:	_asset.load("audio/bgm_game.ogg"),
			se_rot:		_asset.load("audio/se_rot.ogg"),
			se_erase:_asset.load("audio/se_erase.ogg"),
		};
		let	_field = GameProc::init_field();												// フィールド初期化


		_commands.insert_resource(ClearColor(Color::rgb(0.25, 0.25, 0.25)));				// 背景色
		_commands.spawn_bundle(SpriteBundle													// スコアボード
		{
			texture: _asset.load("sprite/board.png"),
			transform: Transform::from_translation(Vec3::new(172.0, 0.0, 5.0)),
			..Default::default()
		});

		for _x in 0..FIELD_W {																// 球
			for _y in 0..FIELD_H {
				Ball::init(_x, _y, _field[_x][_y],	_commands, &_game.tex_ball);
			}
		}

		let	_tex = _asset.load("sprite/pivot.png");
		for _x in 0..(FIELD_W - 1) {														// 回転軸
			for _y in 0..(FIELD_H - 1) {
				Pivot::init(_x, _y,	_commands, &_tex);
			}
		}

		GaugeBar::init_texture(Gauge::TIME,	16.0, Vec3::new(36.0, -160.0, 6.0), _asset.load("sprite/timer.png"),	_commands);
																							// 残り時間
		GaugeBar::init_color(Gauge::COMBO,	6.0, Vec3::new(108.0, -127.0, 6.0), Color::rgb(0.25, 0.75, 0.5),		_commands);
																							// コンボ減少ゲージ
		WarningFilter::init(_commands);														// タイムアップ警告用
		StartMessage::init(_commands, &_asset.load("font/FiraSans-Bold.ttf"));				// 開始待ちメッセージ

		_game
	}

	/**********************************
	    フィールド初期化
				戻り値	球の色の配列
	 **********************************/
	fn	init_field() -> [[usize; FIELD_H]; FIELD_W]
	{
		let mut	_field = [[0 as usize; FIELD_H]; FIELD_W];

		for _x in 0..FIELD_W {
			for _y in 0..FIELD_H {
				let mut	_t: usize = 0;
				loop {
					_t = random::<usize>() % 5;
					if _x % 2 == 0 {
						if _x == 0 {
							break;
						}
						if _t != _field[_x - 1][_y]
								|| ((_y == 0 || _t != _field[_x][_y - 1]) && (_y == FIELD_H - 1 || _t != _field[_x - 1][_y + 1])) {
							break;
						}
					}
					else {
						if _y == 0 {
							break;
						}
						if _t != _field[_x - 1][_y - 1]
								|| (_t != _field[_x][_y - 1] && _t != _field[_x - 1][_y]) {
							break;
						}
					}
				}
				_field[_x][_y] = _t;
			}
		}
		_field
	}


	/**********
	    稼働
	 **********/
	fn	update(&mut self, _val: &mut GameValue, _mouse: &Mouse,
						_commands: &mut Commands,
						_audio: &Res<Audio>,
						_fade_query: &mut Query<&mut Fade>,
						_ball_query: &mut Query<&mut Ball>,
						_pivot_query: &mut Query<&mut Pivot>,
						_ready_query: &mut Query<Entity, With<StartMessage>>) -> bool
	{

		let mut	ball = Vec::new();												// 球配列
		for _x in 0..FIELD_W {
			ball.push(Vec::new());
		}
		for _ball in _ball_query.iter_mut() {
			ball[_ball.cx].push(_ball);
		}
		for _x in 0..FIELD_W {
			ball[_x].sort_by_key(|_ball| _ball.cy);
		}

		match self.phase {
		  Phase::READY =>					// 開始待ち
			{
				if _mouse.is_trigger_l() {
					self.phase = Phase::GAME;
					_audio.set_volume(0.2);
					_audio.play(self.bgm_game.clone());							// BGM再生
					_commands.entity(_ready_query.single_mut()).despawn_recursive();			// 開始待ちメッセージ消去
				}
			},

		  Phase::GAME =>					// ゲーム中
			{
				if _mouse.is_trigger_l() {										// 回転軸押下チェック
					for mut _pivot in _pivot_query.iter_mut() {
						if _pivot.check_rect(_mouse.x, _mouse.y) {
							if self.rotate(&mut ball, _pivot.cx, _pivot.cy) {
								_pivot.push();
								_audio.play(self.se_rot.clone());
							}
							break;
						}
					}
				}

				self.time -= 1;													// 時間経過
				if self.time == 0 {
					self.phase = Phase::END;
				}
				else if self.time == 20*(crate::FRAME_RATE as u32) {			// 場に最も少ない色の球が出なくなる
					let mut	_num = [0 as usize; 5];
					for _x in 0..FIELD_W {										// 球の色を数える
						for _y in 0..FIELD_H {
							if ball[_x][_y].color < 5 {
								_num[ball[_x][_y].color] += 1;
							}
						}
					}

					let mut	t = FIELD_W*FIELD_H;
					for i in 0..5 {
						if _num[i] < t {
							self.forbidden_color = i;
							t = _num[i];
						}
					}
				}
			},

		  Phase::END =>						// 終了待ち
			{
				if !self.move_flag {
					self.phase = Phase::OVER;
					self.cnt = 45 + 30;
				}
			},

		  Phase::OVER =>					// ゲームオーバー
			{
				if self.cnt > 0 {
					self.cnt -= 1;
					if self.cnt == 30 {
						GameOver::init(_commands, &self.tex_over);				// "GAME OVER"
						OverFilter::init(_commands);							// フィールドを暗くする
					}
				}
				else if self.cnt == 0 {
					if _mouse.is_trigger_l() {
						_fade_query.single_mut().fade_out(8);					// フェードアウト
						self.cnt = -8;
					}
				}
				else {
					self.cnt += 1;
					 if self.cnt == 0 {
						return	false;											// ゲーム終了
					}
				}
			},
		}

		self.check_erase(_val, &mut ball, _commands, _audio);					// 球消去チェック
		self.check_fall(&mut ball);												// 球落下チェック

		true
	}

	/****************************
	    球回転
			戻り値	回転したか
	 ****************************/
	fn	rotate(&mut self, ball: &mut Vec<Vec<Mut<Ball>>>, x: usize, y: usize) -> bool
	{
		let	x0 = x;
		let	y0 = y + (x & 1);
		let	x1 = x + 1;
		let	y1 = y;
		let	x2 = x + 1;
		let	y2 = y + 1;

		if (ball[x0][y0].erase_cnt != 0) || (ball[x1][y1].erase_cnt != 0) || (ball[x2][y2].erase_cnt != 0) {
			return	false;														// 回転できない球がある
		}

		let	c0 = ball[x0][y0].color;
		let	c1 = ball[x1][y1].color;
		let	c2 = ball[x2][y2].color;
		ball[x0][y0].rotate(1, c1);												// 球回転
		ball[x1][y1].rotate(0, c2);
		ball[x2][y2].rotate(2, c0);

		if (x != self.push.0) || (y != self.push.1) {
			self.push.0 = x;
			self.push.1 = y;
			if self.dec_combo > 0 {												// コンボゲージ減算
				self.dec_combo -= 8;
				if self.dec_combo < 0 {
					self.dec_combo = 0;
				}
			}
			else if self.combo > 0 {											// コンボ減算
				self.combo -= 1;
			}
		}

		true
	}

	/********************************
	    球消去チェック
			戻り値	消去があったか
	 ********************************/
	fn	check_erase(&mut self, _val: &mut GameValue, ball: &mut Vec<Vec<Mut<Ball>>>,
							_commands: &mut Commands,
							_audio: &Res<Audio>) -> bool
	{
		let mut	d = 0;
		let mut	d_score = 0;

		for x in 0..FIELD_W {
			for y in 0..FIELD_H - 1 {
				let	x0 = x;
				let	y0 = y;
				let	x1 = x;
				let	y1 = y + 1;
				let	t = ball[x0][y0].color;
				if ((t & 0xe8) == 0) && (t == ball[x1][y1].color) {				// 空白・稼働中を除く
					let	y2 = y + 1 - (x & 1);
					if x > 0 {
						let	x2 = x - 1;
						if (t == ball[x2][y2].color)
									&& (((ball[x0][y0].wait_cnt == 0) && (ball[x1][y1].wait_cnt == 0) && (ball[x2][y2].wait_cnt == 0))
										|| (ball[x0][y0].erase_cnt > 4) || (ball[x1][y1].erase_cnt > 4) || (ball[x2][y2].erase_cnt > 4)) {
							ball[x0][y0].erase(_commands, &self.tex_ball);
							ball[x1][y1].erase(_commands, &self.tex_ball);
							ball[x2][y2].erase(_commands, &self.tex_ball);
							if (ball[x0][y0].erase_cnt == 0xff) || (ball[x1][y1].erase_cnt == 0xff) || (ball[x2][y2].erase_cnt == 0xff) {
								d_score += self.add_score(_commands,
															FIELD_X - 22.0 + (x as f32)*BALL_W,
															FIELD_Y + (BALL_H/2.0) - (x as f32)*(BALL_H/2.0) + (((x >> 1) + y) as f32)*BALL_H, d);
								d += 1;
							}
						}
					}
					if x < FIELD_W - 1 {
						let	x2 = x + 1;
						if (t == ball[x2][y2].color)
									&& (((ball[x0][y0].wait_cnt == 0) && (ball[x1][y1].wait_cnt == 0) && (ball[x2][y2].wait_cnt == 0))
										|| (ball[x0][y0].erase_cnt > 4) || (ball[x1][y1].erase_cnt > 4) || (ball[x2][y2].erase_cnt > 4)) {
							ball[x0][y0].erase(_commands, &self.tex_ball);
							ball[x1][y1].erase(_commands, &self.tex_ball);
							ball[x2][y2].erase(_commands, &self.tex_ball);
							if (ball[x0][y0].erase_cnt == 0xff) || (ball[x1][y1].erase_cnt == 0xff) || (ball[x2][y2].erase_cnt == 0xff) {
								d_score += self.add_score(_commands,
															FIELD_X + 22.0 + (x as f32)*BALL_W,
															FIELD_Y + (BALL_H/2.0) - (x as f32)*(BALL_H/2.0) + (((x >> 1) + y) as f32)*BALL_H, d);
								d += 1;
							}
						}
					}
				}
			}
		}
		if d_score > 0 {														// 消去があった
			d_score += _val.score.target;
			_val.score.set_up(d_score);
			_audio.play(self.se_erase.clone());
			return	true;
		}
		false
	}

	/*********************************************
	    スコア加算
			引数	_x, _y = エフェクト表示位置
					_t     = エフェクトディレイ
			戻り値	スコア加算値
	 *********************************************/
	fn	add_score(&mut self, _commands: &mut Commands, mut _x: f32, _y: f32, _t: isize) -> u32
	{
		if self.combo < 99 {
			self.combo += 1;
			self.combo_cnt = 1;
		}
		self.dec_combo = self.dec_combo*4/5 + 18;

		let	_d = self.combo*5;

		let mut	score = _d as usize;
		if _d >= 10 {
			_x += if _d < 100 {9.0} else {18.0};
		}
		while score > 0 {
			ScoreEffect::init(score % 10, _x, _y, 28 + 4 + _t*4,				// エフェクト
										_commands, &self.tex_number);
			score /= 10;
			_x -= 18.0;
		}

		_d
	}

	/********************************
	    落下チェック
			戻り値	落下があったか
	 ********************************/
	fn	check_fall(&mut self, ball: &mut Vec<Vec<Mut<Ball>>>) -> bool
	{
		let mut	_ret = false;

		for x in 0..FIELD_W {
			for  y in 0..FIELD_H - 1 {
				if ((ball[x][y].color & 0xef) == 0x08) && ((ball[x][y + 1].color & 0xe8) == 0) && (ball[x][y + 1].erase_cnt == 0) {
					let	color = ball[x][y + 1].color;
					ball[x][y].fall(color);
					ball[x][y + 1].color = 0x08;
				}
			}
			if ball[x][FIELD_H - 1].color == 0x08 {								// 最上段が空白
				loop {
					let	c = random::<usize>() % 5;
					if (c != (ball[x][FIELD_H - 2].color & 0x0f)) && (c != self.forbidden_color)  {
						ball[x][FIELD_H - 1].fall(c);							// 新しい球
						break;
					}
				}
				_ret = true;
			}
		}
		_ret
	}


	/********************
	    コンボ関連稼働
	 ********************/
	fn	update_combo(&mut self, _val: &mut GameValue,
								mut _query: Query<(&mut GaugeBar, &mut Transform)>)
	{
		if !self.move_flag && (self.dec_combo > 0) {							// コンボゲージ減算
			self.dec_combo -= 1;
		}
		if _val.combo.value < self.combo {										// コンボ表示
			self.combo_cnt -= 1;
			if self.combo_cnt == 0 {
				let	t = _val.combo.value;
				_val.combo.set(t + 1);
				self.combo_cnt = 3;
			}
		}
		else if _val.combo.value != self.combo {
			_val.combo.set(self.combo);
		}

		for (mut _gauge, mut _trans) in _query.iter_mut() {						// ゲージ稼働
			let	_w =	match _gauge.index
						{
							Gauge::TIME  => (self.time as f32)*272.0/(60.0*crate::FRAME_RATE),
							Gauge::COMBO => self.dec_combo as f32,
						};
			_gauge.set(_w, &mut _trans);
		}
	}
}

/********************
    ゲーム進行稼働
 ********************/
fn	update_game(mut _game: ResMut<GameProc>,
							mut _state: ResMut<State<crate::AppState>>,
							mut _val: ResMut<GameValue>,
							_mouse: Res<Mouse>,
							mut _commands: Commands,
							_audio: Res<Audio>,
							mut _fade_query: Query<&mut Fade>,
							mut _ball_query: Query<&mut Ball>,
							mut _pivot_query: Query<&mut Pivot>,
							mut _ready_query: Query<Entity, With<StartMessage>>)
{
	if !_game.update(&mut _val, &_mouse,
						&mut _commands,
						&_audio,
						&mut _fade_query,
						&mut _ball_query,
						&mut _pivot_query,
						&mut _ready_query) {
		_state.set(crate::AppState::NEXT).unwrap();
	}
}

/************
    球稼働
 ************/
fn	update_ball(mut _game: ResMut<GameProc>,
							mut _query: Query<(&mut Ball, &mut Transform, &mut TextureAtlasSprite, &mut Visibility)>)
{
	_game.move_flag = false;
	for (mut _ball, mut _trans, mut _spr, mut _visible) in _query.iter_mut() {
		if _ball.update(&mut _trans, &mut _spr, &mut _visible) {
			_game.move_flag = true;												// 球動作中
		}
	}
}

/********************
    コンボ関連稼働
 ********************/
fn	update_combo(mut _game: ResMut<GameProc>,
							mut _val: ResMut<GameValue>,
							_query: Query<(&mut GaugeBar, &mut Transform)>)
{
	_game.update_combo(&mut _val, _query);
}


/**************
    数値描画
 **************/
struct	GameValue
{
	hi_score: ValueSpr,			// ハイスコア
	score: ValueSpr,			// スコア
	combo: ValueSpr,			// コンボ数
}

impl GameValue
{
	/****************************************
	    初期化
			引数	_hi_score = ハイスコア
	 ****************************************/
	fn	new(_hi_score: u32,
					_commands: &mut Commands,
					_asset: &Res<AssetServer>,
					_tex_atlases: &mut ResMut<Assets<TextureAtlas>>) -> Self
	{
		let	_tex = _tex_atlases.add(TextureAtlas::from_grid(_asset.load("sprite/number_m.png"), Vec2::new(32.0, 32.0), 10, 1));
																				// 数値用テクスチャ
		GameValue
		{
			hi_score:	ValueSpr::new(Value::HISCORE,	6, 280.0,   16.0, 32.0,	_commands, &_tex).set(_hi_score),		// ハイスコア
			score:		ValueSpr::new(Value::SCORE,		6, 280.0,  -60.0, 32.0,	_commands, &_tex),						// スコア
			combo:		ValueSpr::new(Value::COMBO,		2, 248.0, -114.0, 32.0,	_commands, &_tex),						// コンボ数
		}
	}

	/**********
	    稼働
	 **********/
	fn	update(&mut self, mut _query: Query<(&NumberSpr, &mut TextureAtlasSprite, &mut Visibility)>)
	{
		self.score.update();													// スコアアップ稼働
		if self.score.value > self.hi_score.value {								// ハイスコア更新
			self.hi_score.set(self.score.value);
		}
		for (_num, mut _spr, mut _visible) in _query.iter_mut() {				// 数値スプライト稼働
			let	_val =	match _num.index
						{
							Value::HISCORE => self.hi_score.value,
							Value::SCORE   => self.score.value,
							Value::COMBO   => self.combo.value,
						};
			_num.set(_val, &mut _spr, &mut _visible);
		}
	}
}

/******************
    数値描画稼働
 ******************/
fn	update_value(mut _val: ResMut<GameValue>,
							mut _query: Query<(&NumberSpr, &mut TextureAtlasSprite, &mut Visibility)>)
{
	_val.update(_query);
}


/****************
    回転軸稼働
 ****************/
fn	update_pivot(mut _query: Query<(&mut Pivot, &mut Transform)>)
{
	for (mut _pivot, mut _trans) in _query.iter_mut() {
		_pivot.update(&mut _trans);
	}
}

/**************************
    球消去エフェクト稼働
 **************************/
fn	update_ball_effect(mut _commands: Commands,
								mut _query: Query<(Entity, &mut BallEffect, &mut TextureAtlasSprite)>)
{
	for (_entity, mut _effect, mut _spr) in _query.iter_mut() {
		if _effect.update(&mut _spr) {
			_commands.entity(_entity).despawn();
		}
	}
}

/******************************
    獲得スコアエフェクト稼働
 ******************************/
fn	update_score_effect(mut _commands: Commands,
								mut _query: Query<(Entity, &mut ScoreEffect, &mut Transform, &mut TextureAtlasSprite, &mut Visibility)>)
{
	for (_entity, mut _effect, mut _trans, mut _spr, mut _visible) in _query.iter_mut() {
		if _effect.update(&mut _trans, &mut _spr, &mut _visible) {
			_commands.entity(_entity).despawn();
		}
	}
}

/**************************
    タイムアップ警告稼働
 **************************/
fn	update_warning(_game: Res<GameProc>,
								mut _query: Query<(&mut WarningFilter, &mut Sprite)>)
{
	if _game.time < 10*(crate::FRAME_RATE as u32) {								// タイムアップ警告
		let	(mut _filter, mut _spr) = _query.single_mut();
		_filter.set(0.35 - ((_game.time as f32)*(std::f32::consts::PI/30.0)).cos()*0.35, &mut _spr);
	}
}

/****************************
    開始待ちメッセージ稼働
 ****************************/
fn	update_ready(mut _query: Query<(&mut StartMessage, &mut Transform)>)
{
	if let Ok((mut _mes, mut _trans)) = _query.get_single_mut() {				// 開始待ちメッセージ稼働
		_mes.update(&mut _trans);
	}
}

/****************************
    ゲームオーバー表示稼働
 ****************************/
fn	update_over(mut _query: QuerySet<(
								QueryState<(&mut GameOver, &mut Transform, &mut Sprite)>,
								QueryState<(&mut OverFilter, &mut Sprite)>
							)>)
{
	if let Ok((mut _mes, mut _trans, mut _spr)) = _query.q0().get_single_mut() {		// "GAME OVER"
		_mes.update(&mut _trans, &mut _spr);
	}
	if let Ok((mut _filter, mut _spr)) = _query.q1().get_single_mut() {					// フィールドを暗くする
		_filter.update(&mut _spr);
	}
}


struct	ResidentEntity(Vec<u32>);		// 常駐エンティティ

/******************
    ゲーム初期化
 ******************/
fn	setup(_data: Res<crate::CommonData>,
					mut _commands: Commands,
					_asset: Res<AssetServer>,
					_entity_query: Query<Entity>,
					mut _tex_atlases: ResMut<Assets<TextureAtlas>>,
					mut _fade_query: Query<&mut Fade>)
{
	let mut	_vec = Vec::new();													// 既に存在するエンティティ
	for _entity in _entity_query.iter() {
		_vec.push(_entity.id());
	}
	_commands.insert_resource(ResidentEntity(_vec));

	let	_game = GameProc::new(&mut _commands, &_asset, &mut _tex_atlases);		// ゲーム処理
	_commands.insert_resource(_game);

	let	_value = GameValue::new(_data.hi_score, &mut _commands, &_asset, &mut _tex_atlases);		// 数値描画
	_commands.insert_resource(_value);

	_fade_query.single_mut().fade_in(8);										// フェードイン
}

/**********
    終了
 **********/
fn	exit(mut _data: ResMut<crate::CommonData>,
					_val: Res<GameValue>,
					mut _commands: Commands,
					_query: Query<Entity>,
					_vec: Res<ResidentEntity>)
{
	_data.hi_score = _val.hi_score.value;										// ハイスコア記録

	for _entity in _query.iter() {												// エンティティ削除
		if !_vec.0.iter().any(|_id| *_id == _entity.id()) {
			_commands.entity(_entity).despawn();
		}
	}
	_commands.remove_resource::<GameProc>();									// リソース削除
	_commands.remove_resource::<GameValue>();
}


use bevy::ecs::schedule::ShouldRun;

/**********************
    ゲームプラグイン
 **********************/
pub struct	GamePlugin;

impl Plugin for GamePlugin
{
	fn	build(&self, _app: &mut App)
	{
		_app.add_system_set(
			SystemSet::on_enter(crate::AppState::GAME)
				.with_system(setup)												// ゲーム初期化
		);
		_app.add_system_set(
			SystemSet::on_update(crate::AppState::GAME)
				.with_run_criteria(FixedTimestep::step(1.0/(crate::FRAME_RATE as f64)).chain(
					|In(_input): In<ShouldRun>, state: Res<State<crate::AppState>>|
					{
						if state.current() == &crate::AppState::GAME {_input} else {ShouldRun::No}
					})
				)
				.with_system(crate::mouse::update.label("mouse"))				// マウス処理
				.with_system(update_game.label("game").after("mouse"))			// ゲーム進行処理
				.with_system(update_ball.label("ball").after("game"))			// 球稼働
				.with_system(update_combo.label("combo").after("ball"))			// コンボ関連稼働
				.with_system(update_value.after("combo"))						// 数値描画稼働
				.with_system(update_pivot.after("game"))						// 回転軸稼働
				.with_system(update_ball_effect.after("game"))					// 球消去エフェクト稼働
				.with_system(update_score_effect.after("game"))					// 獲得スコアエフェクト稼働
				.with_system(update_warning.after("game"))						// タイムアップ警告稼働
				.with_system(update_ready.after("game"))						// 開始待ちメッセージ稼働
				.with_system(update_over.after("game"))							// ゲームオーバー表示稼働
				.with_system(crate::fade::update.after("game"))					// フェード処理
		);
		_app.add_system_set(
			SystemSet::on_exit(crate::AppState::GAME)
				.with_system(exit)												// 終了
		);
	}
}

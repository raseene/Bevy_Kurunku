
use bevy::prelude::*;


/************************
    開始待ちメッセージ
 ************************/
#[derive(Component)]
pub struct	StartMessage
{
	cnt: isize,				// アニメーション用カウンタ
}

impl StartMessage
{
	/************
	    初期化
	 ************/
	pub fn	init(_commands: &mut Commands, _font: &Handle<Font>)
	{
		let	_message = "Click to Start!";
		let	_alignment = TextAlignment
		{
			vertical: VerticalAlign::Center,
			horizontal: HorizontalAlign::Center,
		};
		_commands
			.spawn_bundle(Text2dBundle					// メッセージ
			{
				text: Text::with_section(_message,
											TextStyle
											{
												font: _font.clone(),
												font_size: 52.0,
												color: Color::rgb(0.9, 1.0, 0.95),
											},
											_alignment),
				transform: Transform::from_translation(Vec3::new(-152.0, 0.0, 7.0)),
				..Default::default()
			})
			.with_children(|parent|						// 影
			{
				parent
					.spawn_bundle(Text2dBundle
					{
						text: Text::with_section(_message,
													TextStyle
													{
														font: _font.clone(),
														font_size: 52.0,
														color: Color::rgba(0.0, 0.0, 0.0, 0.9),
													},
													_alignment),
						transform: Transform::from_translation(Vec3::new(3.0, -3.0, -0.1)),
						..Default::default()
					});
			})
			.insert(StartMessage{cnt: 0});
	}

	/**********
	    稼働
	 **********/
	pub fn	update(&mut self, _trans: &mut Transform)
	{
		self.cnt = (self.cnt + 1) % 64;
		let	_t = ((self.cnt as f32)*(std::f32::consts::PI*2.0/64.0)).sin()/8.0;
		_trans.scale.x = 1.0 + _t;
		_trans.scale.y = 1.0 - _t;
	}
}


/***************
    GAME OVER
 ***************/
#[derive(Component)]
pub struct	GameOver
{
	cnt: isize,				// アニメーション用カウンタ
	alpha: f32,				// α値
}

impl GameOver
{
	/************
	    初期化
	 ************/
	pub fn	init(_commands: &mut Commands, _tex: &Handle<Image>)
	{
		_commands.spawn_bundle(SpriteBundle
		{
			sprite: Sprite
			{
				color: Color::rgba(1.0, 1.0, 1.0, 0.0),
				..Default::default()
			},
			texture: _tex.clone(),
			transform: Transform::from_translation(Vec3::new(-150.0, 14.0, 7.0)),
			..Default::default()
		})
		.insert(GameOver{cnt: 0, alpha: 0.0});
	}

	/**********
	    稼働
	 **********/
	pub fn	update(&mut self, _trans: &mut Transform, _spr: &mut Sprite)
	{
		self.cnt = (self.cnt + 1) % 96;
		_trans.translation.y = 14.0 - ((self.cnt as f32)*(std::f32::consts::PI*2.0/96.0)).sin()*12.0;

		self.alpha += 0.05;
		if self.alpha > 1.0 {
			self.alpha = 1.0;
		}
		_spr.color.set_a(self.alpha);
	}
}

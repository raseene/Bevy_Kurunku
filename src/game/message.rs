
use bevy::prelude::*;
use	rand::random;


/************************
    開始待ちメッセージ
 ************************/
pub struct	StartMessage;

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
			.insert(StartMessage);
	}

	/**********
	    稼働
	 **********/
	pub fn	update(&self, _trans: &mut Transform)
	{
		_trans.translation.x = -152.0 + (((random::<isize>() % 2) + (random::<isize>() % 2) - 2) as f32);
		_trans.translation.y =    0.0 + (((random::<isize>() % 3) + (random::<isize>() % 3) - 3) as f32);
	}
}


/***************
    GAME OVER
 ***************/
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
	pub fn	init(_commands: &mut Commands, _mat: &Handle<ColorMaterial>)
	{
		_commands.spawn_bundle(SpriteBundle
		{
			material: _mat.clone(),
			transform: Transform::from_translation(Vec3::new(-150.0, 14.0, 7.0)),
			..Default::default()
		})
		.insert(GameOver{cnt: 0, alpha: 0.0});
	}

	/**********
	    稼働
	 **********/
	pub fn	update(&mut self, _trans: &mut Transform, _mat: &mut ColorMaterial)
	{
		self.cnt = (self.cnt + 1) % 96;
		_trans.translation.y = 14.0 - ((self.cnt as f32)*(std::f32::consts::PI*2.0/96.0)).sin()*12.0;

		self.alpha += 0.05;
		if self.alpha > 1.0 {
			self.alpha = 1.0;
		}
		_mat.color.set_a(self.alpha);
	}
}

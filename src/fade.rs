
use bevy::prelude::*;


/*************************
    フェードイン/アウト
 *************************/
#[derive(Component)]
pub struct Fade
{
	alpha: f32,			// α値
	speed: f32,			// 変化速度
}

impl Fade
{
	/************
	    初期化
	 ************/
	pub fn	init(_commands: &mut Commands, _materials: &mut ResMut<Assets<ColorMaterial>>)
	{
		_commands.spawn_bundle(SpriteBundle
		{
			sprite: Sprite
			{
				color: Color::rgba(0.0, 0.0, 0.0, 1.0),
				custom_size: Some(Vec2::new(crate::SCREEN_WIDTH, crate::SCREEN_HEIGHT)),
				..Default::default()
			},
			transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
			..Default::default()
		})
		.insert(Fade{alpha: 1.0, speed: 0.0});
	}

	/**********
	    稼働
	 **********/
	pub fn	update(&mut self, _spr: &mut Sprite)
	{
		if self.speed != 0.0 {
			self.alpha += self.speed;
			if self.alpha <= 0.0 {						// フェードイン完了
				self.alpha = 0.0;
				self.speed = 0.0;
			}
			else if self.alpha >= 1.0 {					// フェードアウト完了
				self.alpha = 1.0;
				self.speed = 0.0;
			}
			_spr.color.set_a(self.alpha);
		}
	}

	/*************************************
	    フェードイン
			引数	_cnt = フェード時間
	 *************************************/
	pub fn	fade_in(&mut self, _cnt: i32)
	{
		self.speed = if _cnt > 0 {-1.0/(_cnt as f32)} else {-1.0};
	}

	/*************************************
	    フェードアウト
			引数	_cnt = フェード時間
	 *************************************/
	pub fn	fade_out(&mut self, _cnt: i32)
	{
		self.speed = if _cnt > 0 {1.0/(_cnt as f32)} else {1.0};
	}

	/******************************
	    フェード判定
			戻り値	フェード中か
	 ******************************/
	#[allow(dead_code)]
	pub fn	is_fade(&mut self) -> bool
	{
		self.speed != 0.0
	}
}

/**********
    稼働
 **********/
pub fn	update(mut _materials: ResMut<Assets<ColorMaterial>>, mut _query: Query<(&mut Fade, &mut Sprite)>)
{
	let	(mut _fade, mut _spr) = _query.single_mut();
	_fade.update(&mut _spr);
}


use bevy::
{
	prelude::*,
	input::{mouse::MouseButtonInput, ElementState},
	window::CursorMoved,
};
use std::collections::HashMap;


/****************
    マウス管理
 ****************/
pub struct	Mouse
{
	pub	x: f32,									// カーソル位置
	pub	y: f32,
	pub	button: HashMap<MouseButton, u16>,		// ボタンの状態
}

impl Mouse
{
	const RAW: u16		= (1 << 0);				// 生データ
	const DOWN: u16		= (1 << 1);				// 押下
	const TRIGGER: u16	= (1 << 2);				// 初回押下
	const RELEASE: u16	= (1 << 3);				// リリース


	/************
	    初期化
	 ************/
	pub fn	new() -> Self
	{
		Mouse
		{
			x: 0.0,
			y: 0.0,
			button: HashMap::from(
					[
						(MouseButton::Left, 0),
						(MouseButton::Right, 0),
						(MouseButton::Middle, 0),
					]),
		}
	}

	/**********
	    入力
	 **********/
	pub fn	cursor_input(&mut self, _event: &CursorMoved)
	{
		self.x = _event.position.x*crate::SCREEN_SCALE - crate::SCREEN_WIDTH/2.0;			// カーソル座標
		self.y = _event.position.y*crate::SCREEN_SCALE - crate::SCREEN_HEIGHT/2.0;
	}

	pub fn	button_input(&mut self, _event: &MouseButtonInput)
	{
		self.button.insert(_event.button,													// ボタンの状態
			match _event.state {
				ElementState::Pressed	=> self.button[&_event.button] | Self::RAW,
				ElementState::Released	=> self.button[&_event.button] & !Self::RAW,
			}
		);
	}

	/**********************
	    フレーム毎の処理
	 **********************/
	pub fn	update(&mut self)
	{
		for _button in &[MouseButton::Left, MouseButton::Middle, MouseButton::Right] {
			let mut	_state = self.button[_button];

			if (_state & Self::RAW) != 0 {									// 押下中
				_state = if (_state & Self::DOWN) == 0 { Self::RAW | Self::DOWN | Self::TRIGGER } else { Self::RAW | Self::DOWN };
			}
			else {
				_state = if (_state & Self::DOWN) != 0 { Self::RELEASE } else { 0 };
			}

			self.button.insert(*_button, _state);							// 状態更新
		}
	}

	/********************************
	    押下チェック
			戻り値	押されているか
	 ********************************/
	#[allow(dead_code)]
	pub fn	is_down(&self, _btn: MouseButton) -> bool
	{
		(self.button[&_btn] & Self::DOWN) != 0
	}

	#[allow(dead_code)]
	pub fn	is_down_l(&self) -> bool
	{
		self.is_down(MouseButton::Left)
	}

	#[allow(dead_code)]
	pub fn	is_trigger(&self, _btn: MouseButton) -> bool
	{
		(self.button[&_btn] & Self::TRIGGER) != 0
	}

	#[allow(dead_code)]
	pub fn	is_trigger_l(&self) -> bool
	{
		self.is_trigger(MouseButton::Left)
	}

	#[allow(dead_code)]
	pub fn	is_release(&self, _btn: MouseButton) -> bool
	{
		(self.button[&_btn] & Self::RELEASE) != 0
	}

	#[allow(dead_code)]
	pub fn	is_release_l(&self) -> bool
	{
		self.is_release(MouseButton::Left)
	}
}


/**************
    状態監視
 **************/
pub fn	input(mut _mouse: ResMut<Mouse>, mut _button_events: EventReader<MouseButtonInput>, mut _cursor_events: EventReader<CursorMoved>)
{
	for _event in _cursor_events.iter() {				// カーソル
		_mouse.cursor_input(&_event);
	}
	for _event in _button_events.iter() {				// ボタン
		_mouse.button_input(&_event);
	}
}

/**********************
    フレーム毎の処理
 **********************/
pub fn	update(mut _mouse: ResMut<Mouse>)
{
	_mouse.update();
}

//! Set up an app with 2 buttons that have multiple "accelerators"
#![allow(unused_imports,unused_variables,unreachable_code,dead_code,non_upper_case_globals)]
extern crate helperes      as h    ;
extern crate helperes_proc as hproc;
use ::h            	::*; // gets macros :: prefix needed due to proc macro expansion
pub use hproc      	::*; // gets proc macros
pub use ::h::alias 	::*;
pub use ::h::helper	::*;

_mod!(binmod); //→ #[path="binmod/[binmod].rs"] pub mod binmod;
use crate::binmod::print42;

use std::error::Error;
use std::result;

type Result<T> = result::Result<T, Box<dyn Error>>;
#![allow(unused_imports)]
use cacao::appkit::menu::{Menu, MenuItem};use cacao::appkit::window::{Window, WindowConfig, WindowDelegate};use cacao::appkit::{App, AppDelegate};
struct BasicApp {window: Window<AppWindow>}
impl AppDelegate for BasicApp {
  fn did_finish_launching(&self) {
    App::set_menu(vec![
      Menu::new("", vec![MenuItem::Services,MenuItem::Separator,MenuItem::Hide,MenuItem::HideOthers,MenuItem::ShowAll,MenuItem::Separator,MenuItem::Quit,]),
      Menu::new("File", vec![MenuItem::CloseWindow]),
    ]);
    App::activate();self.window.show();
  }
  fn should_terminate_after_last_window_closed(&self) -> bool {true}
}
#[derive(Default)] struct AppWindow {content:View, button:Option<Button>, button2:Option<Button>,} //option avoids lack of default Button
use cacao::{
  layout 	::{Layout,LayoutConstraint},
  text   	::{Label,TextAlign},
  view   	::{View,ViewDelegate,ViewController},
  switch 	::Switch,
  button 	::{Button,BezelStyle, BezelStyle as Border},
  control	::{Control,ControlSize,},
  color  	::{Color, Theme},
};

impl WindowDelegate for AppWindow {
  const NAME: &'static str = "WindowDelegate";
  fn did_load(&mut self, win: Window) {
    win.set_title("AutoLayout Example");
    win.set_minimum_content_size(30., 30.);
    win.set_maximum_content_size(430., 330.);
    win.set_movable_by_background	(true	);

    let dynamic = Color::dynamic(|style| match (style.theme, style.contrast) {
      (Theme::Dark, _)	=> Color::SystemGreen,
      _               	=> Color::SystemRed});

    // let mut y=Button::new("❗Overwrite"	);y.set_action(|| {});y.set_key_equivalent("\r");//🖍
    // self.content.add_subview(&y);
    // let mut n=Button::new("Overwrite"	);n.set_action(|| {});n.set_key_equivalent("a");//🖍
    // self.content.add_subview(&n);

    let mut y=Button::new("❗Overwrite"	);y.set_action(|| {});y.set_key_equivalent("\r");//🖍
    let mut n=Button::new("Cancel"    	);n.set_action(|| {});n.set_key_equivalent("a");
    y.set_highlighted(false);y.set_bezel_style(BezelStyle::Inline);y.set_control_size(ControlSize::Large);y.set_text_color(Color::SystemRed  );
    n.set_highlighted(true );n.set_bezel_style(BezelStyle::RegularSquare);n.set_control_size(ControlSize::Large);n.set_text_color(Color::SystemBlack);
    self.content.add_subview(&y);
    self.content.add_subview(&n);

    win.set_content_view(&self.content);

    LayoutConstraint::activate(&[
      n            	.top     	.constraint_equal_to(&self.content.top     	).offset(16.),
      n            	.bottom  	.constraint_equal_to(&self.content.bottom  	).offset(-16.),
      n            	.leading 	.constraint_equal_to(&self.content.leading 	).offset( 16.),
      n            	.width   	.constraint_equal_to_constant(100.         	),
      n            	.height  	.constraint_equal_to_constant(20.          	),
      y            	.top     	.constraint_equal_to(&self.content.top     	).offset(126.),
      y            	.bottom  	.constraint_equal_to(&self.content.bottom  	).offset(-16.),
      // y         	.leading 	.constraint_equal_to(&self.content.leading 	).offset(206.),
      y            	.trailing	.constraint_equal_to(&self.content.trailing	).offset(-16.),
      y            	.width   	.constraint_equal_to_constant(200.         	),
      y            	.height  	.constraint_equal_to_constant(20.          	),
    ]);
    let bYes = y;
    let bNo  = n;
    self.button = Some(bYes);
    self.button2 = Some(bNo);
  }
}

fn main() {
  App::new("com.test.window", BasicApp {window: Window::with(WindowConfig::default(), AppWindow::default())}).run();
}

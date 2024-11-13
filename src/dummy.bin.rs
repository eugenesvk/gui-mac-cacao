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
use cacao::appkit::FocusRingType;

use objc::{class, msg_send, sel, sel_impl,runtime	::Object, };
use core_graphics::base::CGFloat;

// no effect since multiplier doesn't seem to be used
#[allow(dead_code)]
trait LayoutConstraintExt {
  fn multiplier    <F:Into<f64>>( self, multiplier:F) -> Self;
  fn set_multiplier<F:Into<f64>>(&self, multiplier:F);
}
impl LayoutConstraintExt for cacao::layout::LayoutConstraint {
  /// Sets the multiplier for this constraint
  fn multiplier<F:Into<f64>>(self, multiplier:F) -> Self {
    let multiplier: f64 = multiplier.into();
    unsafe {let m = multiplier as CGFloat; let _: () = msg_send![&*self.constraint, setConstant:m];}
    // println!("multiplier={}",&multiplier);
    LayoutConstraint {
      animator  	: self.animator,
      constraint	: self.constraint,
      offset    	: self.offset,
      multiplier	: multiplier,
      priority  	: self.priority
    }
  }
  /// Sets the multiplier of a borrowed constraint
  fn set_multiplier<F:Into<f64>>(&self, multiplier:F) {
    let multiplier: f64 = multiplier.into();
    unsafe {let m = multiplier as CGFloat; let _: () = msg_send![&*self.constraint, setConstant:m];}
  }
}
impl WindowDelegate for AppWindow {
  const NAME: &'static str = "WindowDelegate";
  fn did_load(&mut self, win: Window) {
    win.set_title("AutoLayout Example");
    win.set_minimum_content_size(30., 30.);
    // win.set_maximum_content_size(430., 630.);
    win.set_movable_by_background	(true	);

    let dynamic = Color::dynamic(|style| match (style.theme, style.contrast) {
      (Theme::Dark, _)	=> Color::SystemGreen,
      _               	=> Color::SystemRed});

    let mut y=Button::new("❗Overwrite"	);y.set_action(|| {});y.set_key_equivalent("y");
    let mut n=Button::new("Cancel"    	);n.set_action(|| {});n.set_key_equivalent("\r");
    y.set_control_size(ControlSize::Large);
    n.set_control_size(ControlSize::Large);
    // y.set_alpha(0.1);
    // n.set_alpha(0.9);
    y.set_bezel_style(BezelStyle::RegularSquare);
    n.set_bezel_style(BezelStyle::RegularSquare); // RegularSquare, ShadowlessSquare,SmallSquare,TexturedSquare break become vertical 100% of the height
    n.set_bezel_style(BezelStyle::Rounded);
    y.set_focus_ring_type(FocusRingType::Exterior); // seems to have no effect
    n.set_focus_ring_type(FocusRingType::None); // already an highlighted button, don't need another indicator
    y.set_text_color(Color::SystemRed  );
    // n.set_text_color(Color::SystemBlack);
    self.content.add_subview(&y);
    self.content.add_subview(&n);

    win.set_content_view(&self.content);

    let yl	= Label::new();yl.set_text("y")	;self.content.add_subview(&yl	);
    let nl	= Label::new();nl.set_text("↩")	;self.content.add_subview(&nl	);

    let hn:f64 = 40.0; let hy:f64 = hn;
    LayoutConstraint::activate(&[
      n    	.top     	.constraint_equal_to(&self.content.top                	).offset( 16.),
      nl   	.top     	.constraint_equal_to(&n.center_y                      	).offset(hn/2.0 +8.),
      y    	.top     	.constraint_equal_to(&self.content.top                	).offset(126.),
      yl   	.top     	.constraint_equal_to(&y.center_y                      	).offset(hy/2.0 +0.),
      n    	.bottom  	.constraint_equal_to(&self.content.bottom             	).offset(-16.),
      y    	.bottom  	.constraint_equal_to(&self.content.bottom             	).offset(-16.),
      n    	.leading 	.constraint_equal_to(&self.content.leading            	).offset( 16.),y	.leading	.constraint_greater_than_or_equal_to(&n.trailing	).offset(5.),
      nl   	.center_x	.constraint_equal_to(&n.center_x                      	),
      y    	.trailing	.constraint_equal_to(&self.content.trailing           	).offset(-46.),
      yl   	.center_x	.constraint_equal_to(&y.center_x                      	),
      n    	.width   	.constraint_equal_to_constant(200.                    	)             ,n	.height	.constraint_equal_to_constant(hn	),
      y    	.width   	.constraint_equal_to(&n.width                         	)             ,y	.height	.constraint_equal_to_constant(hy	),
      // yl	.top     	.constraint_equal_to(&self.content.bottom             	).offset( -8.),
      // yl	.top     	.constraint_equal_to(&y.bottom                        	).offset(16.),
      // yl	.top     	.constraint_equal_to(&self.content.top                	).offset(126.),
      // nl	.bottom  	.constraint_equal_to(&self.content.bottom             	).offset(-18.),
      // nl	.top     	.constraint_equal_to(&self.content.bottom             	).offset( -8.),
      // nl	.top     	.constraint_greater_than_or_equal_to(&self.content.top	).offset( 16.),
      // nl	.top     	.constraint_equal_to(&n.top                           	).offset(30.),
      // nl	.bottom  	.constraint_equal_to(&n.bottom                        	).offset(- 8.),
      // nl	.top     	.constraint_equal_to(&n.top                           	).offset( 40.),
      // yl	.top     	.constraint_equal_to(&y.top                           	).offset( 40.),
      // y 	.width   	.constraint_equal_to(&n.width                         	).multiplier(4.),
      // y 	.leading 	.constraint_equal_to(&self.content.leading            	).offset(206.),
      // y 	.height  	.constraint_equal_to(&n.height                        	).multiplier(2.), //bugs
      //
      // self.blue 	.top     	.constraint_equal_to(&self.content.top     	).offset(146.),
      // self.blue 	.leading 	.constraint_equal_to(&self.content.leading 	).offset( 16.),
      // self.blue 	.bottom  	.constraint_equal_to(&self.content.bottom  	).offset(-16.),
      // self.blue 	.width   	.constraint_equal_to_constant(100.         	),
      // self.blue 	.height  	.constraint_equal_to_constant(10.          	),
      // self.red  	.top     	.constraint_equal_to(&self.content.top     	).offset( 46.),
      // self.red  	.leading 	.constraint_equal_to(&self.blue.trailing   	).offset( 16.),
      // self.red  	.bottom  	.constraint_equal_to(&self.content.bottom  	).offset(-16.),
      // self.green	.top     	.constraint_equal_to(&self.content.top     	).offset( 46.),
      // self.green	.leading 	.constraint_equal_to(&self.red.trailing    	).offset( 16.),
      // self.green	.trailing	.constraint_equal_to(&self.content.trailing	).offset(-16.),
      // self.green	.bottom  	.constraint_equal_to(&self.content.bottom  	).offset(-16.),
      // self.green	.width   	.constraint_equal_to_constant(100.         	),
    ]);
    // LayoutConstraint::activate(&[
      // y	.width	.constraint_equal_to(&n.width	).multiplier(4.),
    // ]);
      // y	.width	.constraint_equal_to(&n.width	).set_multiplier(4.);
    self.button  = Some(y);
    self.button2 = Some(n);
  }
}

fn main() {
  App::new("com.test.window", BasicApp {window: Window::with(WindowConfig::default(), AppWindow::default())}).run();
}

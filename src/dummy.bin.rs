// RegularSquare button style makes both buttons take the ~full height of the window
// Rounded button style makes the button look normal even if it's only added to 1 of 2 buttons (uncomment line 40)
#![allow(unused_imports)]
use cacao::appkit::menu::{Menu, MenuItem};use cacao::appkit::window::{Window, WindowConfig, WindowDelegate};use cacao::appkit::{App, AppDelegate};
struct BasicApp {window: Window<AppWindow>}
impl AppDelegate for BasicApp {
  fn did_finish_launching(&self) {App::set_menu(vec![
      Menu::new("", vec![MenuItem::Services,MenuItem::Separator,MenuItem::Hide,MenuItem::HideOthers,MenuItem::ShowAll,MenuItem::Separator,MenuItem::Quit,]),
      Menu::new("File", vec![MenuItem::CloseWindow]),]);
    App::activate();self.window.show();
  }
  fn should_terminate_after_last_window_closed(&self) -> bool {true}
}
#[derive(Default)] struct AppWindow {content:View,blue:View,red:View,green:View} //option avoids lack of default Button
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

impl WindowDelegate for AppWindow {
  const NAME: &'static str = "WindowDelegate";
  fn did_load(&mut self, win: Window) {
    win.set_title("AutoLayout Example");
    win.set_minimum_content_size(30., 30.);
    win.set_movable_by_background	(true	);

    let y=Button::new("Overwrite"	);
    let n=Button::new("Cancel"   	);
    y.set_bezel_style(BezelStyle::RegularSquare);
    n.set_bezel_style(BezelStyle::RegularSquare); // BUGS: RegularSquare, ShadowlessSquare,SmallSquare,TexturedSquare break become vertical 100% of the height
    // n.set_bezel_style(BezelStyle::Rounded); // OK even if button Y is untouched
    self.content.add_subview(&y);
    self.content.add_subview(&n);

    self.blue.set_background_color(Color::SystemBlue)	;self.blue.layer.set_corner_radius(16.)	;self.content.add_subview(&self.blue);
    self.red.set_background_color(Color::SystemRed)  	                                       	;self.content.add_subview(&self.red);
    self.content.add_subview(&self.green);
    win.set_content_view(&self.content);

    LayoutConstraint::activate(&[
      n	.top     	.constraint_equal_to(&self.content.top          	).offset( 16.),
      n	.bottom  	.constraint_equal_to(&self.content.bottom       	).offset(-16.),
      n	.leading 	.constraint_equal_to(&self.content.leading      	).offset( 16.),
      n	.width   	.constraint_equal_to_constant(200.              	),
      y	.top     	.constraint_equal_to(&self.content.top          	).offset(126.),
      y	.bottom  	.constraint_equal_to(&self.content.bottom       	).offset(-16.),
      y	.leading 	.constraint_greater_than_or_equal_to(&n.trailing	).offset(5.),
      y	.trailing	.constraint_equal_to(&self.content.trailing     	).offset(-46.),
      y	.width   	.constraint_equal_to(&n.width                   	),
    ]);
  }
}

fn main() {
  App::new("com.test.window", BasicApp {window: Window::with(WindowConfig::default(), AppWindow::default())}).run();
}

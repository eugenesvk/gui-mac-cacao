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
use cacao::events::EventType;
use std::sync::RwLock;
use cacao::appkit::menu::{Menu, MenuItem};use cacao::appkit::window::{Window, WindowConfig, WindowDelegate};use cacao::appkit::{App, AppDelegate};
use cacao::foundation::NSString;
struct BasicApp {window: Window<AppWindow>,key_monitor: RwLock<Option<EventMonitor>>}
impl BasicApp {
  /// Monitor for key presses, and dispatch if they match an action we're after
  pub fn start_monitoring(&self) {
    let mut lock = self.key_monitor.write().unwrap();
    *lock = Some(Event::local_monitor(EventMask::KeyDown | EventMask::FlagsChanged, |evt| { // EventMask::KeyUp |
      //use calculator::{dispatch, Msg};
      let kind = evt.kind();
      let ev_t:&str = match kind {
        EventType::FlagsChanged	=> "Δ in ⇧⎈⌥⌘",
        EventType::KeyDown     	=> "↓",
        EventType::KeyUp       	=> "↑",
        _                      	=> "?",
      };
      match evt.kind() {
         EventType::KeyDown
        |EventType::KeyUp	=> {
          let chars = evt.characters(); //characters associated with a key-up or key-down event
          let chars = evt.characters_ignoring_modifiers(); //characters associated with a key-up or key-down event w/o mods (except ⇧)
          let key_code = evt.key_code(); //virtual code for the key associated with the event.
          let mod_flag = evt.modifier_flags(); //virtual code for the key associated with the event.
          println!("{} {}𝚻{:?} vk={} mod_flag={}", chars, ev_t,kind, key_code, mod_flag);
          match chars.as_ref() {
            "y" => {press_y("letter y")},
            "c" => {press_n("letter c")},
            "s" => {press_n("letter s")},
            _ => return Some(evt),
          }
        },
        // use key code to diff ‹vs› in modifiers as key presses (not as part of modifier flags)
        EventType::FlagsChanged	=> {
          let key_code = evt.key_code(); //virtual code for the key associated with the event.
          let mod_flag = evt.modifier_flags(); //virtual code for the key associated with the event.
          println!("   {}𝚻{:?} vk={} mod_flag={:#}", ev_t,kind, key_code, mod_flag);
        }
        _	=> {//dbg!("  𝚻{:?} ev_t={} ev={:?}", kind, ev_t, evt);
          return None},
      }
      None
    }));
  }
}

impl AppDelegate for BasicApp {
  fn did_finish_launching(&self) {
    App::set_menu(vec![
      Menu::new("", vec![MenuItem::Services,MenuItem::Separator,MenuItem::Hide,MenuItem::HideOthers,MenuItem::ShowAll,MenuItem::Separator,MenuItem::Quit,]),
      Menu::new("File", vec![MenuItem::CloseWindow]),
    ]);
    App::activate();self.window.show();
    self.start_monitoring(); // Event Monitors need to be started after the App has been activated. We use an RwLock here, but it's possible this entire method can be &mut self and you wouldn't need these kinds of shenanigans.
  }
  fn should_terminate_after_last_window_closed(&self) -> bool {true}
}
use cacao::appkit::{Event, EventMask, EventMonitor};
#[derive(Default)] struct AppWindow {content:View, button:Option<Button>, button2:Option<Button>,} //option avoids lack of default Button
use cacao::{
  layout 	::{Layout,LayoutConstraint,},
  text   	::{Label,TextAlign,Font,AttributedString, AttributedString as RichStr },
  view   	::{View,ViewDelegate,ViewController,},
  switch 	::Switch,
  button 	::{Button,BezelStyle, BezelStyle as Border,ImagePosition,},
  control	::{Control,ControlSize,},
  color  	::{Color, Theme,},
  image  	::{Image,MacSystemIcon,SFSymbol},
  utils  	::os::OS_VERSION,
};
use cacao::appkit::FocusRingType;

use cacao::objc::{class, msg_send, sel, sel_impl};
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

// trait ImageExt {
//   fn os_icon(icon:objc2_app_kit::NSImageName) -> Self;
// }
// impl ImageExt for cacao::image::Image {
//   /// Returns a stock system icon, ⚠️NOT guaranteed to exist across all versions of macOS supported
//   #[cfg(target_os = "macos")]
//   pub fn os_icon(icon:objc2_app_kit::NSImageName) -> Self {
//     Image(unsafe {ShareId::from_ptr({
//       let icon = icon.to_id();
//       msg_send![class!(NSImage), imageNamed:icon]})
//     })
//   }
// }
use core::ops::Range;
fn press_y(s:&str) {println!("Y action from: {}",s)}
fn press_n(s:&str) {println!("N action from: {}",s)}
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

    // let mut y=Button::new("O̲verwrite"	);y.set_action(|_| {press_y("UI button")});y.set_key_equivalent("o"); //❗
    // let mut n=Button::new("S̲kip"  	);n.set_action(|_| {press_n("UI button")});n.set_key_equivalent("\r");

    let lbl = "S̲kip"; let lbl_u16 = lbl.encode_utf16(); let len = lbl_u16.count() as isize;
    let acc = "S̲"; let acc_len = acc.encode_utf16().count() as isize;
    let mut n=Button::new(lbl	);n.set_action(|_| {press_n("UI button")});n.set_key_equivalent("\r");
    let mut attr_str = RichStr::new(lbl);
    let font = Font::system(16.); attr_str.set_font(font, Range{start:0,end:len}); // make label bigger

    let accelerator = Range{start:0,end:acc_len}; //[start,end)
    attr_str.set_text_color(cacao::color::Color::rgb(150,255,150), accelerator.clone());
    let font = Font::bold_system(16.);
    attr_str.set_font(font, accelerator);
    n.set_attributed_text(attr_str);

    let lbl = "O̲verwrite"; let lbl_u16 = lbl.encode_utf16(); let len = lbl_u16.count() as isize;
    let acc = "O̲"; let acc_len = acc.encode_utf16().count() as isize;
    let mut y=Button::new(lbl	);y.set_action(|_| {press_n("UI button")});y.set_key_equivalent("o");
    let mut attr_str = RichStr::new(lbl);
    let font = Font::system(16.); attr_str.set_font(font, Range{start:0,end:len}); // make label bigger

    let accelerator = Range{start:0,end:acc_len}; //[start,end)
    attr_str.set_text_color(cacao::color::Color::rgb(200,0,0), accelerator.clone());
    let font = Font::bold_system(16.);
    attr_str.set_font(font, accelerator);
    y.set_attributed_text(attr_str);


    y.set_control_size(ControlSize::Large);
    n.set_control_size(ControlSize::Large);
    // y.set_alpha(0.1);
    // n.set_alpha(0.9);
    y.set_bezel_style(BezelStyle::Rounded);
    n.set_bezel_style(BezelStyle::Rounded); // RegularSquare, ShadowlessSquare,SmallSquare,TexturedSquare break become vertical 100% of the height
    y.set_focus_ring_type(FocusRingType::Exterior); // seems to have no effect
    n.set_focus_ring_type(FocusRingType::None); // already an highlighted button, don't need another indicator
    // y.set_text_color(Color::SystemRed  );
    // n.set_text_color(Color::SystemBlack);

    if let os_info::Version::Semantic(os_major,_,_) = OS_VERSION.version() {
      if *os_major >= 11 {//debug!("info major version={:?}", os_major);
        let icon = Image::symbol(SFSymbol::SquareAndArrowDownOnSquareFill, "Overwrite"); //SFSymbol min version 11, alt MacSystemIcon
        y.set_image(icon);
        y.set_image_position(ImagePosition::ImageLeft); // developer.apple.com/library/archive/documentation/Cocoa/Conceptual/Button/Tasks/SettingButtonImage.html
      }
    }
    self.content.add_subview(&y);
    self.content.add_subview(&n);

    win.set_content_view(&self.content);

    // let yl	= Label::new();yl.set_text("y")   	;self.content.add_subview(&yl	);
    let nl   	= Label::new();nl.set_text("↩¦c") 	;self.content.add_subview(&nl	);
    let yl   	= Label::new(); //yl.set_text("y")	;
    let mut attr_str = RichStr::new("🤦🏼‍♂️✗🦀✗sdasfsd");
    let alen = "🦀".len(); // 4, but ↓ is 2 symbols, check https://stackoverflow.com/questions/50409143/convert-utf-8-bytes-emoji-code-to-emoji-icon-as-a-text
    let alen2 = "🤦🏼‍♂️".len(); //17
    let ln3 = "🤦🏼‍♂️".encode_utf16().count(); //7
    // is this u8 vs u16?
    println!("a={} b={} c={}",alen,alen2, ln3);
    let green_range =Range{start:0,end:7}; //[start,end) 8 starts coloring ✗
    attr_str.set_text_color(cacao::color::Color::rgb(0,255,0),green_range);
    yl.set_attributed_text(attr_str);
    self.content.add_subview(&yl	);

    let hn:f64 = 20.0; let hy:f64 = hn; //20 seems to be the default large, but manually setting.height makes the buttons bug and have diff H
    LayoutConstraint::activate(&[
      n    	.top     	.constraint_equal_to(&self.content.top                	).offset( 46.),
      nl   	.top     	.constraint_equal_to(&n.center_y                      	).offset(hn/2.0 +2.),
      y    	.top     	.constraint_equal_to(&self.content.top                	).offset( 46.),
      yl   	.top     	.constraint_equal_to(&y.center_y                      	).offset(hy/2.0 +0.),
      n    	.bottom  	.constraint_equal_to(&self.content.bottom             	).offset(-16.),
      y    	.bottom  	.constraint_equal_to(&self.content.bottom             	).offset(-16.),
      n    	.leading 	.constraint_equal_to(&self.content.leading            	).offset( 16.),y	.leading	.constraint_greater_than_or_equal_to(&n.trailing	).offset(5.),
      nl   	.center_x	.constraint_equal_to(&n.center_x                      	),
      y    	.trailing	.constraint_equal_to(&self.content.trailing           	).offset(-46.),
      yl   	.center_x	.constraint_equal_to(&y.center_x                      	),
      n    	.width   	.constraint_equal_to_constant(200.                    	)             ,//n	.height	.constraint_equal_to_constant(hn	),
      y    	.width   	.constraint_equal_to(&n.width                         	)             ,//y	.height	.constraint_equal_to_constant(hy	),
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
  App::new("com.test.window", BasicApp {window:Window::with(WindowConfig::default(),AppWindow::default()), key_monitor:RwLock::new(None)}).run();
}

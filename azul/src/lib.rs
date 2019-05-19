//! Azul is a free, functional, immediate-mode GUI framework for rapid development
//! of desktop applications written in Rust, supported by the Mozilla WebRender
//! rendering engine, using a flexbox-based CSS / DOM model for layout and styling.
//!
//! # Concept
//!
//! Azul is largely based on the principle of immediate-mode GUI frameworks, which
//! is that the entire UI (in Azuls case the DOM) is reconstructed and re-rendered
//! on every frame (instead of having functions that mutate the UI state like
//! `button.setText()`). This method of constructing UIs has a performance overhead
//! over methods that retain the UI, therefore Azul only calls the [`Layout::layout()`]
//! function when its absolutely necessary - inside of a callback, you can return
//! whether it is necessary to redraw the screen or not (by returning
//! [`Redraw`] or [`DontRedraw`], respectively).
//!
//! In difference to other immediate-mode frameworks, Azul does not immediately
//! draw to the screen, but rather "draws" to a `Dom`. This has several advantages,
//! such as making it possible to layout code at runtime, [loading a `Dom` from
//! an XML file], recognizing state changes by diffing two frames, as well as being
//! able to reparent DOMs into almost any configuration to make components reusable
//! independent of the context they are in.
//!
//! # Development lifecycle
//!
//! A huge problem when working with GUI applications in Rust is managing the
//! compile time. Having to recompile your entire code when you just want to
//! shift an element a pixel to the right is not a good developer experience.
//! Azul has three main methods of combating compile time:
//!
//! - The [XML] system, which allows you to load DOMs at runtime [from a file]
//! - The [CSS] system, which allows you to [load and parse stylesheets]
//!
//! Due to Azuls stateless rendering architecutre, hot-reloading also preserves
//! the current application state. Once you are done layouting your applications
//! UI, you can [transpile the XML code to valid Rust source code] using [azulc],
//! the Azul-XML-to-Rust compiler.
//!
//! Please note that the compiler isn't perfect - the XML system is very limited,
//! and parsing XML has a certain performance overhead, since it's done on every frame.
//! That is fine for debug builds, but the XML system should not be used in release mode.
//!
//! When you are done with designing the callbacks of your widget, you may want to
//! package the widget up to autmatically react to certain events without having the
//! user of your widget write any code to hook up the callbacks - for this purpose,
//! Azul features a [two way data binding] system.
//!
//! # Custom drawing and embedding external applications
//!
//! Azul is mostly concerned with rendering text, images and rectangular boxes (divs).
//! Any other content can be drawn by drawing to an OpenGL texture (using a
//! [`GlTextureCallback`]) and handing the texture as an "image" to Azul. This is also how
//! components like a video player or other OpenGL-based visualizations can exist
//! outside of the core library and be "injected" into the UI.
//!
//! You can draw to an OpenGL texture and hand it to Azul in order to display it
//! in the UI - the texture doesn't have to come from Azul itself, you can inject
//! it from an external application.
//!
//! # Limitations
//!
//! There are a few limitations that should be noted:
//!
//! - There are no scrollbars yet. Creating scrollable frames can be done by
//!   [creating an `IFrameCallback`].
//! - Similarly, there is no clipping of overflowing content yet - clipping only
//!   works for `IFrameCallback`s.
//! - There is no support for CSS animations of any kind yet
//! - Changing dynamic variables will trigger an entire UI relayout and restyling
//!
//! # Hello world
//!
//! ```no_run
//! extern crate azul;
//!
//! use azul::prelude::*;
//!
//! struct MyDataModel { }
//!
//! impl Layout for MyDataModel {
//!     fn layout(&self, _: LayoutInfo<Self>) -> Dom<Self> {
//!         Dom::label("Hello World")
//!     }
//! }
//!
//! fn main() {
//!     let mut app = App::new(MyDataModel { }, AppConfig::default()).unwrap();
//!     let window = app.create_window(WindowCreateOptions::default(), css::native()).unwrap();
//!     app.run(window).unwrap();
//! }
//! ```
//!
//! Running this code should return a window similar to this:
//!
//! ![Opening a blank window](https://raw.githubusercontent.com/maps4print/azul/master/doc/azul_tutorial_empty_window.png)
//!
//! # Tutorials
//!
//! Explaining all concepts and examples is too much to be included in
//! this API reference. Please refer to the [wiki](https://github.com/maps4print/azul/wiki)
//! or use the links below to learn about how to use Azul.
//!
//! - [Getting Started](https://github.com/maps4print/azul/wiki/Getting-Started)
//! - [A simple counter](https://github.com/maps4print/azul/wiki/A-simple-counter)
//! - [Styling your app with CSS](https://github.com/maps4print/azul/wiki/Styling-your-application-with-CSS)
//! - [SVG drawing](https://github.com/maps4print/azul/wiki/SVG-drawing)
//! - [OpenGL drawing](https://github.com/maps4print/azul/wiki/OpenGL-drawing)
//! - [Timers, timers, tasks and async IO](https://github.com/maps4print/azul/wiki/Timers,-timers,-tasks-and-async-IO)
//! - [Two-way data binding](https://github.com/maps4print/azul/wiki/Two-way-data-binding)
//! - [Unit testing](https://github.com/maps4print/azul/wiki/Unit-testing)
//!
//! [`Layout::layout()`]: ../azul/traits/trait.Layout.html
//! [widgets]: ../azul/widgets/index.html
//! [loading a `Dom` from an XML file]: ../azul/dom/struct.Dom.html#method.from_file
//! [XML]: ../azul/xml/index.html
//! [`Redraw`]: ../azul/callbacks/constant.Redraw.html
//! [`DontRedraw`]: ../azul/callbacks/constant.DontRedraw.html
//! [`GlTextureCallback`]: ../azul/callbacks/struct.GlTextureCallback.html
//! [creating an `IFrameCallback`]: ../azul/dom/struct.Dom.html#method.iframe
//! [from a file]: ../azul/dom/struct.Dom.html#method.from_file
//! [CSS]: ../azul/css/index.html
//! [load and parse stylesheets]: ../azul/css/fn.from_str.html
//! [transpile the XML code to valid Rust source code]: https://github.com/maps4print/azul/wiki/XML-to-Rust-compilation
//! [azulc]: https://crates.io/crates/azulc
//! [two way data binding]: https://github.com/maps4print/azul/wiki/Two-way-data-binding

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/maps4print/azul/master/assets/images/azul_logo_full_min.svg.png",
    html_favicon_url = "https://raw.githubusercontent.com/maps4print/azul/master/assets/images/favicon.ico",
)]

#![allow(dead_code)]
#![deny(unused_must_use)]
#![deny(unreachable_patterns)]
#![deny(missing_copy_implementations)]
#![deny(clippy::all)]

#[cfg(feature = "logging")]
#[macro_use(warn, error)]
pub extern crate log;
#[macro_use(lazy_static)]
pub extern crate lazy_static;
#[cfg_attr(feature = "svg", macro_use(implement_vertex, uniform))]
pub extern crate glium;

#[cfg(feature = "serde_serialization")]
#[cfg_attr(feature = "serde_serialization", macro_use)]
extern crate serde;
#[cfg(feature = "serde_serialization")]
#[cfg_attr(feature = "serde_serialization", macro_use)]
extern crate serde_derive;

pub extern crate gleam;
pub extern crate euclid;
pub extern crate webrender;
pub extern crate app_units;
pub extern crate unicode_normalization;
pub extern crate tinyfiledialogs;
pub extern crate clipboard2;
pub extern crate font_loader;
pub extern crate xmlparser;
pub extern crate harfbuzz_sys;

#[cfg(feature = "svg")]
pub extern crate stb_truetype;
#[cfg(feature = "logging")]
pub extern crate fern;
#[cfg(feature = "logging")]
pub extern crate backtrace;
#[cfg(feature = "image_loading")]
pub extern crate image;
#[cfg(feature = "svg")]
pub extern crate lyon;
#[cfg(feature = "svg_parsing")]
pub extern crate usvg;
#[cfg(feature = "faster-hashing")]
pub extern crate twox_hash;

#[cfg(feature = "css_parser")]
extern crate azul_css;
extern crate azul_native_style;
extern crate azul_css_parser;

// Crate-internal macros
#[macro_use]
mod macros;

/// Manages application state (`App` / `AppState` / `AppResources`), wrapping resources and app state
pub mod app;
/// Async IO helpers / (`Task` / `Timer` / `Thread`)
pub mod async;
/// Type definitions for various types of callbacks, as well as focus and scroll handling
pub mod callbacks;
/// CSS type definitions / CSS parsing functions
#[cfg(any(feature = "css_parser", feature = "native_style"))]
pub mod css;
/// Bindings to the native file-chooser, color picker, etc. dialogs
pub mod dialogs;
/// DOM / HTML node handling
pub mod dom;
/// Re-exports of errors
pub mod error;
/// Handles text layout (modularized, can be used as a standalone module)
pub mod text_layout;
/// Main `Layout` trait definition + convenience traits for `Arc<Mutex<T>>`
pub mod traits;
/// Container for default widgets (`TextInput` / `Button` / `Label`, `TableView`, ...)
pub mod widgets;
/// Window state handling and window-related information
pub mod window;
/// XML-based DOM serialization and XML-to-Rust compiler implementation
pub mod xml;

/// UI Description & display list handling (webrender)
mod ui_description;
/// HarfBuzz text shaping utilities
mod text_shaping;
/// Converts the UI description (the styled HTML nodes)
/// to an actual display list (+ layout)
mod display_list;
/// Slab allocator for nodes, based on IDs (replaces kuchiki + markup5ever)
mod id_tree;
/// State handling for user interfaces
mod ui_state;
/// The compositor takes all textures (user-defined + the UI texture(s)) and draws them on
/// top of each other
mod compositor;
/// Default logger, can be turned off with `feature = "logging"`
#[cfg(feature = "logging")]
mod logging;
/// Flexbox-based UI solver
mod ui_solver;
/// DOM styling module
mod style;
/// DOM diffing
mod diff;
/// Checks that two-way bound values are on the stack
mod stack_checked_pointer;
/// Window state handling and diffing
mod window_state;
/// ImageId / FontId handling and caching
mod app_resources;

/// Font & image resource handling, lookup and caching
pub mod resources {
    // re-export everything *except* the AppResources (which are exported under the "app" module)
    pub use app_resources::{
        FontId, ImageId, LoadedFont, RawImage, FontReloadError, FontSource, ImageReloadError,
        ImageSource, RawImageFormat, CssFontId, CssImageId,
        TextCache, TextId,
    };
}

// Faster implementation of a HashMap (optional, disabled by default, turn on with --feature="faster-hashing")

#[cfg(feature = "faster-hashing")]
type FastHashMap<T, U> = ::std::collections::HashMap<T, U, ::std::hash::BuildHasherDefault<::twox_hash::XxHash>>;
#[cfg(feature = "faster-hashing")]
type FastHashSet<T> = ::std::collections::HashSet<T, ::std::hash::BuildHasherDefault<::twox_hash::XxHash>>;
#[cfg(not(feature = "faster-hashing"))]
type FastHashMap<T, U> = ::std::collections::HashMap<T, U>;
#[cfg(not(feature = "faster-hashing"))]
type FastHashSet<T> = ::std::collections::HashSet<T>;

/// Quick exports of common types
pub mod prelude {
    #[cfg(feature = "css_parser")]
    pub use azul_css::*;
    pub use app::{App, AppConfig, AppState, AppResources};
    pub use async::{Task, TerminateTimer, TimerId, Timer, DropCheck};
    pub use resources::{
        RawImageFormat, ImageId, FontId, FontSource, ImageSource,
        TextCache, TextId,
    };
    pub use callbacks::{
        Callback, TimerCallback, IFrameCallback, GlTextureCallback,
        UpdateScreen, Redraw, DontRedraw,
        CallbackInfo, FocusTarget, LayoutInfo, HidpiAdjustedBounds, Texture,
    };
    pub use dom::{
        Dom, DomHash, NodeType, NodeData, On, DomString, TabIndex,
        EventFilter, HoverEventFilter, FocusEventFilter, NotEventFilter, WindowEventFilter,
    };
    pub use traits::{Layout, Modify};
    pub use window::{
        MonitorIter, Window, WindowCreateOptions,
        WindowMonitorTarget, RendererType, ReadOnlyWindow
    };
    pub use window_state::{WindowState, KeyboardState, MouseState, DebugState, keymap, AcceleratorKey};
    pub use glium::glutin::{
        dpi::{LogicalPosition, LogicalSize, PhysicalPosition, PhysicalSize},
        VirtualKeyCode, ScanCode, Icon,
    };
    pub use stack_checked_pointer::StackCheckedPointer;
    pub use text_layout::{TextLayoutOptions, GlyphInstance};
    pub use xml::{XmlComponent, XmlComponentMap};

    #[cfg(any(feature = "css_parser", feature = "native_style"))]
    pub use css;
    #[cfg(feature = "logging")]
    pub use log::LevelFilter;
}

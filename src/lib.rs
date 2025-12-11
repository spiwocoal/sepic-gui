#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::SepicApp;

mod serialcomms;

mod tabs;
pub use tabs::MyTabViewer;

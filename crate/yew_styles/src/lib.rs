//! # Yew Styles
//!
//! Yew Styles is a style framework for yew
//!
//! ## Motivation
//!
//! The purpose of developing this project is first,
//! provide a style framework for yew because there isn't not many options currently,
//! also to create a layout system which is not far of the flexbox concept, and,
//! to take the rust benefits and implement a properties selected by enumeration
//! in the most of the cases which makes fast for developing applications and avoids the practice try and error
//!
//! ## How it works
//!
//! Each component is splited in two parts, the logical yew component and its sass module,
//! however, it is not necessary to worry about the sass module only it needs to be include in the project
//!
//! ### How install it
//!
//! 1. Install the sass module: `npm install yew-styles`
//! 2. Add the yew_style crate in Cargo.toml file: `yew_styles = "0.3.1"`
//! 3. Import the main.css file in you main javascript/typescript file project:
//! ```typescript
//!    import 'node_modules/yew-styles/main.css';
//! ```
//! 4. Ready to import and use in your project 🚀
//!
//! ## Development phase
//!
//! Yew style is in early phase, currently doesn't have enough components to cover all the requirements that could need a website/web application.
//! All contributions are appreciated.
#![recursion_limit = "512"]
pub mod assets;
mod components;
pub mod styles;
mod utils;

pub use components::{button, layouts, navbar};

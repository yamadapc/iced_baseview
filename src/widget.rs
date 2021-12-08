//! Display information and interactive controls in your application.
//!
//! # Re-exports
//! For convenience, the contents of this module are available at the root
//! module. Therefore, you can directly type:
//!
//! ```
//! use iced_baseview::{button, Button};
//! ```
//!
//! # Stateful widgets
//! Some widgets need to keep track of __local state__.
//!
//! These widgets have their own module with a `State` type. For instance, a
//! [`TextInput`] has some [`text_input::State`].
//!
//! [`TextInput`]: text_input/struct.TextInput.html
//! [`text_input::State`]: text_input/struct.State.html
mod platform {
    pub use crate::renderer::widget::{
        button, checkbox, container, pane_grid, pick_list, progress_bar, radio,
        rule, scrollable, slider, text_input, Column, Row, Space, Text,
    };

    #[cfg(any(
        feature = "with-wgpu-canvas",
        feature = "with-glow-canvas"
    ))]
    #[cfg_attr(
        docsrs,
        doc(cfg(any(
            feature = "with-wgpu-canvas",
            feature = "with-glow-canvas"
        )))
    )]
    pub use crate::renderer::widget::canvas;

    #[cfg_attr(docsrs, doc(cfg(feature = "with-wgpu-image")))]
    pub mod image {
        //! Display images in your user interface.
        pub use iced_native::image::{Data, Handle};
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "with-wgpu-svg")))]
    pub mod svg {
        //! Display vector graphics in your user interface.
        pub use iced_native::svg::{Data, Handle};
    }

    #[doc(no_inline)]
    pub use {
        button::Button, checkbox::Checkbox, container::Container,
        pane_grid::PaneGrid, pick_list::PickList, progress_bar::ProgressBar,
        radio::Radio, rule::Rule, scrollable::Scrollable, slider::Slider,
        text_input::TextInput,
    };

    #[cfg(any(
        feature = "with-wgpu-canvas",
        feature = "with-glow-canvas"
    ))]
    #[doc(no_inline)]
    pub use canvas::Canvas;
}

pub use platform::*;

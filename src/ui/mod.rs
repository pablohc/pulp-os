// widget toolkit for 1-bit e-paper displays
//
// font-independent primitives (Region, Alignment, stack fmt) live
// here in the kernel. font-dependent widgets (BitmapLabel, QuickMenu,
// ButtonFeedback) live in apps::widgets and are re-exported below
// for backward-compatible import paths.

pub mod stack_fmt;
pub mod statusbar;
mod widget;

pub use stack_fmt::{StackFmt, stack_fmt};
pub use statusbar::{
    BAR_HEIGHT, CONTENT_TOP, free_stack_bytes, paint_stack, stack_high_water_mark,
};
pub use widget::{Alignment, Region, wrap_next, wrap_prev};

pub use crate::board::{SCREEN_H, SCREEN_W};

// re-exports from apps::widgets (font-dependent, app-side)
pub use crate::apps::widgets::QuickMenu;
pub use crate::apps::widgets::bitmap_label::{BitmapDynLabel, BitmapLabel};
pub use crate::apps::widgets::button_feedback::{BUTTON_BAR_H, ButtonFeedback};
pub use crate::apps::widgets::quick_menu;

//! Miscellaneous widgets
//!
//! Small convenience widgets that don’t fit elsewhere (e.g. bullets, help
//! markers). See functions on `Ui` for details.
//!
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::as_conversions
)]
use crate::Ui;
use crate::sys;

bitflags::bitflags! {
    /// Flags for invisible buttons
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct ButtonFlags: i32 {
        /// No flags
        const NONE = 0;
        /// React on left mouse button
        const MOUSE_BUTTON_LEFT = sys::ImGuiButtonFlags_MouseButtonLeft as i32;
        /// React on right mouse button
        const MOUSE_BUTTON_RIGHT = sys::ImGuiButtonFlags_MouseButtonRight as i32;
        /// React on middle mouse button
        const MOUSE_BUTTON_MIDDLE = sys::ImGuiButtonFlags_MouseButtonMiddle as i32;
    }
}

/// Direction for arrow buttons (alias for Direction)
pub use crate::Direction as ArrowDirection;

impl Ui {
    /// Creates a bullet point
    #[doc(alias = "Bullet")]
    pub fn bullet(&self) {
        unsafe {
            sys::igBullet();
        }
    }

    /// Creates a bullet point with text
    #[doc(alias = "BulletText")]
    pub fn bullet_text(&self, text: impl AsRef<str>) {
        let text_ptr = self.scratch_txt(text);
        unsafe {
            sys::igBulletText(text_ptr);
        }
    }
}

impl Ui {
    /// Creates a small button
    #[doc(alias = "SmallButton")]
    pub fn small_button(&self, label: impl AsRef<str>) -> bool {
        let label_ptr = self.scratch_txt(label);
        unsafe { sys::igSmallButton_Str(label_ptr) }
    }

    /// Creates an invisible button
    #[doc(alias = "InvisibleButton")]
    pub fn invisible_button(&self, str_id: impl AsRef<str>, size: impl Into<[f32; 2]>) -> bool {
        self.invisible_button_flags(str_id, size, crate::widget::ButtonFlags::NONE)
    }

    /// Creates an invisible button with flags
    #[doc(alias = "InvisibleButton")]
    pub fn invisible_button_flags(
        &self,
        str_id: impl AsRef<str>,
        size: impl Into<[f32; 2]>,
        flags: crate::widget::ButtonFlags,
    ) -> bool {
        let id_ptr = self.scratch_txt(str_id);
        let size_vec: sys::ImVec2 = size.into().into();
        unsafe { sys::igInvisibleButton_Str(id_ptr, size_vec, flags.bits()) }
    }

    /// Creates an arrow button
    #[doc(alias = "ArrowButton")]
    pub fn arrow_button(&self, str_id: impl AsRef<str>, dir: crate::Direction) -> bool {
        let id_ptr = self.scratch_txt(str_id);
        unsafe { sys::igArrowButton_Str(id_ptr, dir as i32) }
    }
}

// ============================================================================
// Disabled scope (RAII)
// ============================================================================

/// Tracks a disabled scope begun with [`Ui::begin_disabled`] and ended on drop.
#[must_use]
pub struct DisabledToken<'ui> {
    _ui: &'ui Ui,
}

impl<'ui> DisabledToken<'ui> {
    fn new(ui: &'ui Ui) -> Self {
        DisabledToken { _ui: ui }
    }

    /// Ends the disabled scope explicitly.
    pub fn end(self) {
        // Drop will call EndDisabled
    }
}

impl<'ui> Drop for DisabledToken<'ui> {
    fn drop(&mut self) {
        unsafe { sys::igEndDisabled() }
    }
}

impl Ui {
    /// Begin a disabled scope for subsequent items.
    ///
    /// All following widgets will be disabled (grayed out and non-interactive)
    /// until the returned token is dropped.
    #[doc(alias = "BeginDisabled")]
    pub fn begin_disabled(&self) -> DisabledToken<'_> {
        unsafe { sys::igBeginDisabled(true) }
        DisabledToken::new(self)
    }

    /// Begin a conditionally disabled scope for subsequent items.
    ///
    /// If `disabled` is false, this still needs to be paired with the returned
    /// token being dropped to correctly balance the internal stack.
    #[doc(alias = "BeginDisabled")]
    pub fn begin_disabled_with_cond(&self, disabled: bool) -> DisabledToken<'_> {
        unsafe { sys::igBeginDisabled(disabled) }
        DisabledToken::new(self)
    }
}

// ============================================================================
// Button repeat (convenience over item flag)
// ============================================================================

impl Ui {
    /// Enable/disable repeating behavior for subsequent buttons.
    ///
    /// Internally uses `PushItemFlag(ImGuiItemFlags_ButtonRepeat, repeat)`.
    #[doc(alias = "PushButtonRepeat")]
    pub fn push_button_repeat(&self, repeat: bool) {
        unsafe { sys::igPushItemFlag(sys::ImGuiItemFlags_ButtonRepeat as i32, repeat) }
    }

    /// Pop the button repeat item flag.
    #[doc(alias = "PopButtonRepeat")]
    pub fn pop_button_repeat(&self) {
        unsafe { sys::igPopItemFlag() }
    }
}

// ============================================================================
// Item key ownership
// ============================================================================

impl Ui {
    /// Set the key owner for the last item, without flags.
    #[doc(alias = "SetItemKeyOwner")]
    pub fn set_item_key_owner(&self, key: crate::input::Key) {
        let k: sys::ImGuiKey = key as sys::ImGuiKey;
        unsafe { sys::igSetItemKeyOwner_Nil(k) }
    }

    /// Set the key owner for the last item with input flags.
    /// Pass a combination of `ImGuiInputFlags_*` from `dear_imgui_sys`.
    #[doc(alias = "SetItemKeyOwner")]
    pub fn set_item_key_owner_with_flags(
        &self,
        key: crate::input::Key,
        flags: sys::ImGuiInputFlags,
    ) {
        let k: sys::ImGuiKey = key as sys::ImGuiKey;
        unsafe { sys::igSetItemKeyOwner_InputFlags(k, flags) }
    }
}

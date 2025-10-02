//! Image plot implementation

use super::{Plot, PlotError, safe_cstring};
use crate::{ImageFlags, sys};

/// Plot an image in plot coordinates using an ImTextureID
pub struct ImagePlot<'a> {
    label: &'a str,
    tex_id: sys::ImTextureID,
    bounds_min: sys::ImPlotPoint,
    bounds_max: sys::ImPlotPoint,
    uv0: [f32; 2],
    uv1: [f32; 2],
    tint: [f32; 4],
    flags: ImageFlags,
}

impl<'a> ImagePlot<'a> {
    pub fn new(
        label: &'a str,
        tex_id: sys::ImTextureID,
        bounds_min: sys::ImPlotPoint,
        bounds_max: sys::ImPlotPoint,
    ) -> Self {
        Self {
            label,
            tex_id,
            bounds_min,
            bounds_max,
            uv0: [0.0, 0.0],
            uv1: [1.0, 1.0],
            tint: [1.0, 1.0, 1.0, 1.0],
            flags: ImageFlags::NONE,
        }
    }

    pub fn with_uv(mut self, uv0: [f32; 2], uv1: [f32; 2]) -> Self {
        self.uv0 = uv0;
        self.uv1 = uv1;
        self
    }
    pub fn with_tint(mut self, tint: [f32; 4]) -> Self {
        self.tint = tint;
        self
    }
    pub fn with_flags(mut self, flags: ImageFlags) -> Self {
        self.flags = flags;
        self
    }

    pub fn validate(&self) -> Result<(), PlotError> {
        Ok(())
    }
}

impl<'a> Plot for ImagePlot<'a> {
    fn plot(&self) {
        if self.validate().is_err() {
            return;
        }
        let label_c = safe_cstring(self.label);
        let uv0 = sys::ImVec2 {
            x: self.uv0[0],
            y: self.uv0[1],
        };
        let uv1 = sys::ImVec2 {
            x: self.uv1[0],
            y: self.uv1[1],
        };
        let tint = sys::ImVec4 {
            x: self.tint[0],
            y: self.tint[1],
            z: self.tint[2],
            w: self.tint[3],
        };
        // Construct ImTextureRef from ImTextureID
        let tex_ref = sys::ImTextureRef {
            _TexData: std::ptr::null_mut(),
            _TexID: self.tex_id,
        };
        unsafe {
            sys::ImPlot_PlotImage(
                label_c.as_ptr(),
                tex_ref,
                self.bounds_min,
                self.bounds_max,
                uv0,
                uv1,
                tint,
                self.flags.bits() as i32,
            )
        }
    }

    fn label(&self) -> &str {
        self.label
    }
}

/// Convenience methods on PlotUi
impl<'ui> crate::PlotUi<'ui> {
    pub fn plot_image(
        &self,
        label: &str,
        tex_id: sys::ImTextureID,
        bounds_min: sys::ImPlotPoint,
        bounds_max: sys::ImPlotPoint,
    ) -> Result<(), PlotError> {
        let plot = ImagePlot::new(label, tex_id, bounds_min, bounds_max);
        plot.validate()?;
        plot.plot();
        Ok(())
    }

    /// Plot an image using ImGui's TextureId wrapper (if available)
    #[allow(unused_variables)]
    pub fn plot_image_with_imgui_texture(
        &self,
        label: &str,
        texture: dear_imgui_rs::TextureId,
        bounds_min: sys::ImPlotPoint,
        bounds_max: sys::ImPlotPoint,
    ) -> Result<(), PlotError> {
        // dear_imgui_rs::TextureId is a transparent wrapper; cast to sys::ImTextureID via `id()` then transmute
        // This is a common interop pattern across imgui backends.
        let raw: sys::ImTextureID = unsafe { std::mem::transmute(texture.id()) };
        self.plot_image(label, raw, bounds_min, bounds_max)
    }
}

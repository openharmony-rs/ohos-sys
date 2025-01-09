// automatically generated by rust-bindgen 0.71.1

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::types::*;

#[allow(unused_imports)]
#[cfg(feature = "api-12")]
use crate::error_code::OH_Drawing_ErrorCode;

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl OH_Drawing_ScaleToFit {
    /// Scales in x and y to fill destination rect.
    pub const SCALE_TO_FIT_FILL: OH_Drawing_ScaleToFit = OH_Drawing_ScaleToFit(0);
}
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl OH_Drawing_ScaleToFit {
    /// Scales and aligns to left and top.
    pub const SCALE_TO_FIT_START: OH_Drawing_ScaleToFit = OH_Drawing_ScaleToFit(1);
}
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl OH_Drawing_ScaleToFit {
    /// Scales and aligns to center.
    pub const SCALE_TO_FIT_CENTER: OH_Drawing_ScaleToFit = OH_Drawing_ScaleToFit(2);
}
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl OH_Drawing_ScaleToFit {
    /// Scales and aligns to right and bottom.
    pub const SCALE_TO_FIT_END: OH_Drawing_ScaleToFit = OH_Drawing_ScaleToFit(3);
}
#[repr(transparent)]
/// Enumerates of scale to fit flags, how matrix is constructed to map one rect to another.
///
///
/// Available since API-level: 12
///
/// Version: 1.0
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_ScaleToFit(pub ::core::ffi::c_uint);
extern "C" {
    /// Creates an <b>OH_Drawing_Matrix</b> object.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    ///
    /// # Returns
    ///
    /// * Returns the pointer to the <b>OH_Drawing_Matrix</b> object created.
    ///
    /// Available since API-level: 11
    ///
    /// Version: 1.0
    #[cfg(feature = "api-11")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
    pub fn OH_Drawing_MatrixCreate() -> *mut OH_Drawing_Matrix;
    /// Creates an <b>OH_Drawing_Matrix</b> object with rotation. Sets matrix to
    /// rotate by degrees about a pivot point at (px, py).
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `OH_Drawing_Matrix` - Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    ///
    /// * `deg` - angle of axes relative to upright axes
    ///
    /// * `x` - pivot on x-axis.
    ///
    /// * `y` - pivot on y-axis.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_MatrixCreateRotation(deg: f32, x: f32, y: f32) -> *mut OH_Drawing_Matrix;
    /// Creates an <b>OH_Drawing_Matrix</b> object with scale. Sets matrix to scale
    /// by sx and sy, about a pivot point at (px, py).
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `OH_Drawing_Matrix` - Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    ///
    /// * `sx` - horizontal scale factor.
    ///
    /// * `sy` - vertical scale factor.
    ///
    /// * `px` - pivot on x-axis.
    ///
    /// * `py` - pivot on y-axis.
    ///
    /// # Returns
    ///
    /// * Returns the pointer to the <b>OH_Drawing_Matrix</b> object created.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_MatrixCreateScale(
        sx: f32,
        sy: f32,
        px: f32,
        py: f32,
    ) -> *mut OH_Drawing_Matrix;
    /// Creates an <b>OH_Drawing_Matrix</b> object with translation.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `OH_Drawing_Matrix` - Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    ///
    /// * `dx` - horizontal translation.
    ///
    /// * `dy` - vertical translation.
    ///
    /// # Returns
    ///
    /// * Returns the pointer to the <b>OH_Drawing_Matrix</b> object created.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_MatrixCreateTranslation(dx: f32, dy: f32) -> *mut OH_Drawing_Matrix;
    /// Sets the params for a matrix.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `OH_Drawing_Matrix` - Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    ///
    /// * `scaleX` - horizontal scale factor to store
    ///
    /// * `skewX` - horizontal skew factor to store
    ///
    /// * `transX` - horizontal translation to store
    ///
    /// * `skewY` - vertical skew factor to store
    ///
    /// * `scaleY` - vertical scale factor to store
    ///
    /// * `transY` - vertical translation to store
    ///
    /// * `persp0` - input x-axis values perspective factor to store
    ///
    /// * `persp1` - input y-axis values perspective factor to store
    ///
    /// * `persp2` - perspective scale factor to store
    ///
    /// Available since API-level: 11
    ///
    /// Version: 1.0
    #[cfg(feature = "api-11")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
    pub fn OH_Drawing_MatrixSetMatrix(
        arg1: *mut OH_Drawing_Matrix,
        scaleX: f32,
        skewX: f32,
        transX: f32,
        skewY: f32,
        scaleY: f32,
        transY: f32,
        persp0: f32,
        persp1: f32,
        persp2: f32,
    );
    /// Sets matrix to scale and translate src rect to dst rect.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `OH_Drawing_Matrix` - Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    ///
    /// * `src` - Indicates the pointer to an <b>OH_Drawing_Rect</b> object rect to map from.
    ///
    /// * `dst` - Indicates the pointer to an <b>OH_Drawing_Rect</b> object rect to map to.
    ///
    /// * `stf` - Scales to fit enum method.
    ///
    /// # Returns
    ///
    /// * Returns true if dst is empty, and sets matrix to:
    /// | 0 0 0 |
    /// | 0 0 0 |
    /// | 0 0 1 |
    ///
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_MatrixSetRectToRect(
        arg1: *mut OH_Drawing_Matrix,
        src: *const OH_Drawing_Rect,
        dst: *const OH_Drawing_Rect,
        stf: OH_Drawing_ScaleToFit,
    ) -> bool;
    /// Sets matrix to matrix multiplied by matrix constructed from rotating by degrees
    /// about pivot point(px, py), positive degrees rotates clockwise.
    /// Given:
    ///
    /// | A B C | | c -s dx |
    /// Matrix = | D E F |, R(degrees, px, py) = | s c dy |
    /// | G H I | | 0 0 1 |
    ///
    /// where:
    ///
    /// c = cos(degrees)
    /// s = sin(degrees)
    /// dx = s * py + (1 - c) * px
    /// dy = -s * px + (1 - c) * py
    ///
    /// sets Matrix to:
    ///
    /// | A B C | | c -s dx | | Ac+Bs -As+Bc A*dx+B*dy+C |
    /// Matrix * R(degrees, px, py) = | D E F | | s c dy | = | Dc+Es -Ds+Ec D*dx+E*dy+F |
    /// | G H I | | 0 0 1 | | Gc+Hs -Gs+Hc G*dx+H*dy+I |
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `OH_Drawing_Matrix` - Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    ///
    /// * `degree` - Indicates the angle of axes relative to upright axes.
    ///
    /// * `px` - Indicates the pivot on x-axis.
    ///
    /// * `py` - Indicates the pivot on y-axis.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_MatrixPreRotate(arg1: *mut OH_Drawing_Matrix, degree: f32, px: f32, py: f32);
    /// Sets matrix to forward scale by sx and sy, about a pivot point at (px, py).
    /// Given:
    ///
    /// | A B C | | sx 0 dx |
    /// Matrix =| D E F |, S(sx, sy, px, py) = | 0 sy dy |
    /// | G H I | | 0 0 1 |
    ///
    /// where:
    ///
    /// dx = px - sx * px
    /// dy = py - sy * py
    ///
    /// sets Matrix to:
    ///
    /// | A B C | | sx 0 dx | | A*sx B*sy A*dx+B*dy+C |
    /// Matrix * S(sx, sy, px, py) = | D E F | | 0 sy dy | = | D*sx E*sy D*dx+E*dy+F |
    /// | G H I | | 0 0 1 | | G*sx H*sy G*dx+H*dy+I |
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `OH_Drawing_Matrix` - Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    ///
    /// * `sx` - Horizontal scale factor.
    ///
    /// * `sy` - Vertical scale factor.
    ///
    /// * `px` - Pivot on x-axis.
    ///
    /// * `py` - Pivot on y-axis.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_MatrixPreScale(
        arg1: *mut OH_Drawing_Matrix,
        sx: f32,
        sy: f32,
        px: f32,
        py: f32,
    );
    /// Sets forward matrix to translate by dx and dy.
    /// Given:
    /// | A B C | | 1 0 dx |
    /// Matrix = | D E F |, T(dx, dy) = | 0 1 dy |
    /// | G H I | | 0 0 1 |
    /// sets Matrix to:
    /// | A B C | | 1 0 dx | | A B A*dx+B*dy+C |
    /// Matrix * T(dx, dy) = | D E F | | 0 1 dy | = | D E D*dx+E*dy+F |
    /// | G H I | | 0 0 1 | | G H G*dx+H*dy+I |
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `OH_Drawing_Matrix` - Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    ///
    /// * `dx` - Indicates the horizontal translation.
    ///
    /// * `dy` - Indicates the vertical translation.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_MatrixPreTranslate(arg1: *mut OH_Drawing_Matrix, dx: f32, dy: f32);
    /// Sets matrix to matrix constructed from rotating by degrees about pivot point(px, py),
    /// multiplied by matrix, positive degrees rotates clockwise.
    /// Given:
    ///
    /// | J K L | | c -s dx |
    /// Matrix = | M N O |, R(degrees, px, py) = | s c dy |
    /// | P Q R | | 0 0 1 |
    ///
    /// where:
    ///
    /// c = cos(degrees)
    /// s = sin(degrees)
    /// dx = s * py + (1 - c) * px
    /// dy = -s * px + (1 - c) * py
    ///
    /// sets Matrix to:
    ///
    /// |c -s dx| |J K L| |cJ-sM+dx*P cK-sN+dx*Q cL-sO+dx+R|
    /// R(degrees, px, py) * Matrix = |s c dy| |M N O| = |sJ+cM+dy*P sK+cN+dy*Q sL+cO+dy*R|
    /// |0 0 1| |P Q R| | P Q R|
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `OH_Drawing_Matrix` - Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    ///
    /// * `degree` - Indicates the angle of axes relative to upright axes.
    ///
    /// * `px` - Indicates the pivot on x-axis.
    ///
    /// * `py` - Indicates the pivot on y-axis.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_MatrixPostRotate(arg1: *mut OH_Drawing_Matrix, degree: f32, px: f32, py: f32);
    /// Sets matrix to backward scale by sx and sy, about a pivot point at (px, py).
    /// Given:
    /// | J K L | | sx 0 dx |
    /// Matrix = | M N O |, S(sx, sy, px, py) = | 0 sy dy |
    /// | P Q R | | 0 0 1 |
    /// where:
    /// dx = px - sx * px
    /// dy = py - sy * py
    /// sets Matrix to:
    /// | sx 0 dx | | J K L | | sx*J+dx*P sx*K+dx*Q sx*L+dx+R |
    /// S(sx, sy, px, py) * Matrix = | 0 sy dy | | M N O | = | sy*M+dy*P sy*N+dy*Q sy*O+dy*R |
    /// | 0 0 1 | | P Q R | | P Q R |
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `OH_Drawing_Matrix` - Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    ///
    /// * `sx` - Horizontal scale factor.
    ///
    /// * `sy` - Vertical scale factor.
    ///
    /// * `px` - Pivot on x-axis.
    ///
    /// * `py` - Pivot on y-axis.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_MatrixPostScale(
        arg1: *mut OH_Drawing_Matrix,
        sx: f32,
        sy: f32,
        px: f32,
        py: f32,
    );
    /// Sets backward matrix to translate by (dx, dy).
    /// Given:
    ///
    /// | J K L | | 1 0 dx |
    /// Matrix = | M N O |, T(dx, dy) = | 0 1 dy |
    /// | P Q R | | 0 0 1 |
    ///
    /// sets Matrix to:
    ///
    /// | 1 0 dx | | J K L | | J+dx*P K+dx*Q L+dx*R |
    /// T(dx, dy) * Matrix = | 0 1 dy | | M N O | = | M+dy*P N+dy*Q O+dy*R |
    /// | 0 0 1 | | P Q R | | P Q R |
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `OH_Drawing_Matrix` - Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    ///
    /// * `dx` - Indicates the horizontal translation.
    ///
    /// * `dy` - Indicates the vertical translation.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_MatrixPostTranslate(arg1: *mut OH_Drawing_Matrix, dx: f32, dy: f32);
    /// Reset matrix to identity, which has no effect on mapped point, sets matrix to:
    /// | 1 0 0 |
    /// | 0 1 0 |
    /// | 0 0 1 |
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `OH_Drawing_Matrix` - Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_MatrixReset(arg1: *mut OH_Drawing_Matrix);
    /// Sets matrix total to matrix a multiplied by matrix b.
    /// Given:
    /// | A B C | | J K L |
    /// a = | D E F |, b = | M N O |
    /// | G H I | | P Q R |
    /// sets Matrix total to:
    /// | A B C | | J K L | | AJ+BM+CP AK+BN+CQ AL+BO+CR |
    /// total = a * b = | D E F | * | M N O | = | DJ+EM+FP DK+EN+FQ DL+EO+FR |
    /// | G H I | | P Q R | | GJ+HM+IP GK+HN+IQ GL+HO+IR |
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `total` - Indicates the pointer to an <b>OH_Drawing_Matrix</b> object that a * b.
    ///
    /// * `a` - Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    ///
    /// * `b` - Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_MatrixConcat(
        total: *mut OH_Drawing_Matrix,
        a: *const OH_Drawing_Matrix,
        b: *const OH_Drawing_Matrix,
    );
    /// Gets nine matrix values contained by matrix into array.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `matrix` - Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    ///
    /// * `value` - Storages for nine matrix values.
    ///
    /// # Returns
    ///
    /// * Returns the error code.
    /// Returns [`OH_DRAWING_SUCCESS`] if the operation is successful.
    /// Returns [`OH_DRAWING_ERROR_INVALID_PARAMETER`] if matrix or value is nullptr.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_MatrixGetAll(
        matrix: *mut OH_Drawing_Matrix,
        value: *mut f32,
    ) -> OH_Drawing_ErrorCode;
    /// Get one matrix value. Index is between the range of 0-8.
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `OH_Drawing_Matrix` - Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    ///
    /// * `index` - one of 0-8.
    ///
    /// # Returns
    ///
    /// * Returns value corresponding to index.Returns 0 if out of range.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_MatrixGetValue(
        arg1: *mut OH_Drawing_Matrix,
        index: ::core::ffi::c_int,
    ) -> f32;
    /// Sets matrix to rotate by degrees about a pivot point at (px, py). The pivot point is unchanged
    /// when mapped with matrix. Positive degrees rotates clockwise.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `OH_Drawing_Matrix` - Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    ///
    /// * `degree` - Indicates the angle of axes relative to upright axes.
    ///
    /// * `px` - Indicates the pivot on x-axis.
    ///
    /// * `py` - Indicates the pivot on y-axis.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_MatrixRotate(arg1: *mut OH_Drawing_Matrix, degree: f32, px: f32, py: f32);
    /// Sets matrix to translate by (dx, dy)
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `OH_Drawing_Matrix` - Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    ///
    /// * `dx` - Indicates the horizontal translation.
    ///
    /// * `dy` - Indicates the vertical translation.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_MatrixTranslate(arg1: *mut OH_Drawing_Matrix, dx: f32, dy: f32);
    /// Sets matrix to scale by sx and sy, about a pivot point at (px, py).
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `OH_Drawing_Matrix` - Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    ///
    /// * `sx` - Indicates the horizontal scale factor.
    ///
    /// * `sy` - Indicates the vertical scale factor.
    ///
    /// * `px` - Indicates the pivot on x-axis.
    ///
    /// * `py` - Indicates the pivot on y-axis.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_MatrixScale(arg1: *mut OH_Drawing_Matrix, sx: f32, sy: f32, px: f32, py: f32);
    /// Sets inverse to reciprocal matrix, returning true if matrix can be inverted.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `OH_Drawing_Matrix` - Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    ///
    /// * `inverse` - Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    ///
    /// # Returns
    ///
    /// * Returns true if the matrix is not nullptr and can be inverted;
    /// returns false if the matrix is nullptr or cannot be inverted.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_MatrixInvert(
        arg1: *mut OH_Drawing_Matrix,
        inverse: *mut OH_Drawing_Matrix,
    ) -> bool;
    /// Sets the params of matrix to map src to dst.
    /// Count must greater than or equal to zero, and less than or equal to four.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `OH_Drawing_Matrix` - Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    ///
    /// * `src` - Points to map from.
    ///
    /// * `dst` - Points to map to.
    ///
    /// * `count` - Number of point in src and dst.
    ///
    /// # Returns
    ///
    /// * Returns true if matrix is constructed successfully.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_MatrixSetPolyToPoly(
        arg1: *mut OH_Drawing_Matrix,
        src: *const OH_Drawing_Point2D,
        dst: *const OH_Drawing_Point2D,
        count: u32,
    ) -> bool;
    /// Maps the src point array to the dst point array by matrix transformation.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `OH_Drawing_Matrix` - Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    ///
    /// * `src` - Points to map from.
    ///
    /// * `dst` - Points to map to.
    ///
    /// * `count` - Number of point in src and dst.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_MatrixMapPoints(
        arg1: *const OH_Drawing_Matrix,
        src: *const OH_Drawing_Point2D,
        dst: *mut OH_Drawing_Point2D,
        count: ::core::ffi::c_int,
    );
    /// Sets dst to bounds of src corners mapped by matrix transformation.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `OH_Drawing_Matrix` - Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    ///
    /// * `src` - Rect to map from.
    ///
    /// * `dst` - Rect to map to.
    ///
    /// # Returns
    ///
    /// * Returns true if the mapped src is equal to the dst; returns false is not equal.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_MatrixMapRect(
        arg1: *const OH_Drawing_Matrix,
        src: *const OH_Drawing_Rect,
        dst: *mut OH_Drawing_Rect,
    ) -> bool;
    /// Returns true if the first matrix equals the second matrix.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `OH_Drawing_Matrix` - Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    ///
    /// * `other` - Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    ///
    /// # Returns
    ///
    /// * Returns true if the two matrices are equal; returns false if not equal.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_MatrixIsEqual(
        arg1: *mut OH_Drawing_Matrix,
        other: *mut OH_Drawing_Matrix,
    ) -> bool;
    /// Returns true if matrix is identity.
    /// Identity matrix is : | 1 0 0 |
    /// | 0 1 0 |
    /// | 0 0 1 |
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `OH_Drawing_Matrix` - Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    ///
    /// # Returns
    ///
    /// * Returns true if matrix is identity; returns false if not identity.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_MatrixIsIdentity(arg1: *mut OH_Drawing_Matrix) -> bool;
    /// Destroys an <b>OH_Drawing_Matrix</b> object and reclaims the memory occupied by the object.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `OH_Drawing_Matrix` - Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    ///
    /// Available since API-level: 11
    ///
    /// Version: 1.0
    #[cfg(feature = "api-11")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
    pub fn OH_Drawing_MatrixDestroy(arg1: *mut OH_Drawing_Matrix);
}

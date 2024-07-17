#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::drawing::types::{OH_Drawing_Matrix, OH_Drawing_Point2D, OH_Drawing_Rect};

impl OH_Drawing_ScaleToFit {
    /// Scales in x and y to fill destination rect.
    pub const SCALE_TO_FIT_FILL: OH_Drawing_ScaleToFit = OH_Drawing_ScaleToFit(0);
}
impl OH_Drawing_ScaleToFit {
    /// Scales and aligns to left and top.
    pub const SCALE_TO_FIT_START: OH_Drawing_ScaleToFit = OH_Drawing_ScaleToFit(1);
}
impl OH_Drawing_ScaleToFit {
    /// Scales and aligns to center.
    pub const SCALE_TO_FIT_CENTER: OH_Drawing_ScaleToFit = OH_Drawing_ScaleToFit(2);
}
impl OH_Drawing_ScaleToFit {
    /// Scales and aligns to right and bottom.
    pub const SCALE_TO_FIT_END: OH_Drawing_ScaleToFit = OH_Drawing_ScaleToFit(3);
}
#[repr(transparent)]
/** @brief Enumerates of scale to fit flags, how matrix is constructed to map one rect to another.

@since 12
@version 1.0*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_ScaleToFit(pub ::core::ffi::c_uint);

extern "C" {
    /** @brief Creates an <b>OH_Drawing_Matrix</b> object with rotation. Sets matrix to
    rotate by degrees about a pivot point at (px, py).

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Matrix Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    @param deg  angle of axes relative to upright axes
    @param x  pivot on x-axis.
    @param y  pivot on y-axis.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_MatrixCreateRotation(deg: f32, x: f32, y: f32) -> *mut OH_Drawing_Matrix;
    /** @brief Creates an <b>OH_Drawing_Matrix</b> object with scale. Sets matrix to scale
    by sx and sy, about a pivot point at (px, py).

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Matrix Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    @param sx  horizontal scale factor.
    @param sy  vertical scale factor.
    @param px  pivot on x-axis.
    @param py  pivot on y-axis.
    @return Returns the pointer to the <b>OH_Drawing_Matrix</b> object created.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_MatrixCreateScale(
        sx: f32,
        sy: f32,
        px: f32,
        py: f32,
    ) -> *mut OH_Drawing_Matrix;
    /** @brief Creates an <b>OH_Drawing_Matrix</b> object with translation.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Matrix Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    @param dx  horizontal translation.
    @param dy  vertical translation.
    @return Returns the pointer to the <b>OH_Drawing_Matrix</b> object created.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_MatrixCreateTranslation(dx: f32, dy: f32) -> *mut OH_Drawing_Matrix;
    /** @brief Sets matrix to scale and translate src rect to dst rect.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Matrix Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    @param src Indicates the pointer to an <b>OH_Drawing_Rect</b> object rect to map from.
    @param dst Indicates the pointer to an <b>OH_Drawing_Rect</b> object rect to map to.
    @param stf Scales to fit enum method.
    @return Returns true if dst is empty, and sets matrix to:
            | 0 0 0 |
            | 0 0 0 |
            | 0 0 1 |

    @since 12
    @version 1.0*/
    pub fn OH_Drawing_MatrixSetRectToRect(
        arg1: *mut OH_Drawing_Matrix,
        src: *const OH_Drawing_Rect,
        dst: *const OH_Drawing_Rect,
        stf: OH_Drawing_ScaleToFit,
    ) -> bool;
    /** @brief Sets matrix to matrix multiplied by matrix constructed from rotating by degrees
    about pivot point(px, py), positive degrees rotates clockwise.
           Given:

                        | A B C |                        | c -s dx |
               Matrix = | D E F |,  R(degrees, px, py) = | s  c dy |
                        | G H I |                        | 0  0  1 |

           where:

               c  = cos(degrees)
               s  = sin(degrees)
               dx =  s * py + (1 - c) * px
               dy = -s * px + (1 - c) * py

           sets Matrix to:

                                             | A B C | | c -s dx |   | Ac+Bs -As+Bc A*dx+B*dy+C |
               Matrix * R(degrees, px, py) = | D E F | | s  c dy | = | Dc+Es -Ds+Ec D*dx+E*dy+F |
                                             | G H I | | 0  0  1 |   | Gc+Hs -Gs+Hc G*dx+H*dy+I |

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Matrix Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    @param degree Indicates the angle of axes relative to upright axes.
    @param px Indicates the pivot on x-axis.
    @param py Indicates the pivot on y-axis.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_MatrixPreRotate(arg1: *mut OH_Drawing_Matrix, degree: f32, px: f32, py: f32);
    /** @brief Sets matrix to forward scale by sx and sy, about a pivot point at (px, py).
           Given:

                       | A B C |                       | sx  0 dx |
               Matrix =| D E F |,  S(sx, sy, px, py) = |  0 sy dy |
                       | G H I |                       |  0  0  1 |

           where:

               dx = px - sx * px
               dy = py - sy * py

           sets Matrix to:

                                            | A B C | | sx  0 dx |   | A*sx B*sy A*dx+B*dy+C |
               Matrix * S(sx, sy, px, py) = | D E F | |  0 sy dy | = | D*sx E*sy D*dx+E*dy+F |
                                            | G H I | |  0  0  1 |   | G*sx H*sy G*dx+H*dy+I |

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Matrix Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    @param sx Horizontal scale factor.
    @param sy Vertical scale factor.
    @param px Pivot on x-axis.
    @param py Pivot on y-axis.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_MatrixPreScale(
        arg1: *mut OH_Drawing_Matrix,
        sx: f32,
        sy: f32,
        px: f32,
        py: f32,
    );
    /** @brief Sets forward matrix to translate by dx and dy.
           Given:
                        | A B C |               | 1 0 dx |
               Matrix = | D E F |,  T(dx, dy) = | 0 1 dy |
                        | G H I |               | 0 0  1 |
           sets Matrix to:
                                    | A B C | | 1 0 dx |   | A B A*dx+B*dy+C |
               Matrix * T(dx, dy) = | D E F | | 0 1 dy | = | D E D*dx+E*dy+F |
                                    | G H I | | 0 0  1 |   | G H G*dx+H*dy+I |

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Matrix Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    @param dx Indicates the horizontal translation.
    @param dy Indicates the vertical translation.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_MatrixPreTranslate(arg1: *mut OH_Drawing_Matrix, dx: f32, dy: f32);
    /** @brief Sets matrix to matrix constructed from rotating by degrees about pivot point(px, py),
    multiplied by matrix, positive degrees rotates clockwise.
           Given:

                        | J K L |                        | c -s dx |
               Matrix = | M N O |,  R(degrees, px, py) = | s  c dy |
                        | P Q R |                        | 0  0  1 |

           where:

               c  = cos(degrees)
               s  = sin(degrees)
               dx =  s * py + (1 - c) * px
               dy = -s * px + (1 - c) * py

           sets Matrix to:

                                             |c -s dx| |J K L|   |cJ-sM+dx*P cK-sN+dx*Q cL-sO+dx+R|
               R(degrees, px, py) * Matrix = |s  c dy| |M N O| = |sJ+cM+dy*P sK+cN+dy*Q sL+cO+dy*R|
                                             |0  0  1| |P Q R|   |         P          Q          R|

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Matrix Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    @param degree Indicates the angle of axes relative to upright axes.
    @param px Indicates the pivot on x-axis.
    @param py Indicates the pivot on y-axis.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_MatrixPostRotate(arg1: *mut OH_Drawing_Matrix, degree: f32, px: f32, py: f32);
    /** @brief Sets matrix to backward scale by sx and sy, about a pivot point at (px, py).
           Given:
                        | J K L |                       | sx  0 dx |
               Matrix = | M N O |,  S(sx, sy, px, py) = |  0 sy dy |
                        | P Q R |                       |  0  0  1 |
           where:
               dx = px - sx * px
               dy = py - sy * py
           sets Matrix to:
                                            | sx  0 dx | | J K L |   | sx*J+dx*P sx*K+dx*Q sx*L+dx+R |
               S(sx, sy, px, py) * Matrix = |  0 sy dy | | M N O | = | sy*M+dy*P sy*N+dy*Q sy*O+dy*R |
                                            |  0  0  1 | | P Q R |   |         P         Q         R |

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Matrix Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    @param sx Horizontal scale factor.
    @param sy Vertical scale factor.
    @param px Pivot on x-axis.
    @param py Pivot on y-axis.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_MatrixPostScale(
        arg1: *mut OH_Drawing_Matrix,
        sx: f32,
        sy: f32,
        px: f32,
        py: f32,
    );
    /** @brief Sets backward matrix to translate by (dx, dy).
           Given:

                        | J K L |               | 1 0 dx |
               Matrix = | M N O |,  T(dx, dy) = | 0 1 dy |
                        | P Q R |               | 0 0  1 |

           sets Matrix to:

                                    | 1 0 dx | | J K L |   | J+dx*P K+dx*Q L+dx*R |
               T(dx, dy) * Matrix = | 0 1 dy | | M N O | = | M+dy*P N+dy*Q O+dy*R |
                                    | 0 0  1 | | P Q R |   |      P      Q      R |

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Matrix Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    @param dx Indicates the horizontal translation.
    @param dy Indicates the vertical translation.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_MatrixPostTranslate(arg1: *mut OH_Drawing_Matrix, dx: f32, dy: f32);
    /** @brief Reset matrix to identity, which has no effect on mapped point, sets matrix to:
           | 1 0 0 |
           | 0 1 0 |
           | 0 0 1 |

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Matrix Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_MatrixReset(arg1: *mut OH_Drawing_Matrix);
    /** @brief Sets matrix total to matrix a multiplied by matrix b.
          Given:
                       | A B C |          | J K L |
                   a = | D E F |,     b = | M N O |
                       | G H I |          | P Q R |
          sets Matrix total to:
                               | A B C |   | J K L |   | AJ+BM+CP AK+BN+CQ AL+BO+CR |
              total = a * b =  | D E F | * | M N O | = | DJ+EM+FP DK+EN+FQ DL+EO+FR |
                               | G H I |   | P Q R |   | GJ+HM+IP GK+HN+IQ GL+HO+IR |
    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param total Indicates the pointer to an <b>OH_Drawing_Matrix</b> object that a * b.
    @param a Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    @param b Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_MatrixConcat(
        total: *mut OH_Drawing_Matrix,
        a: *const OH_Drawing_Matrix,
        b: *const OH_Drawing_Matrix,
    );
    /** @brief Get one matrix value. Index is between the range of 0-8.
    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Matrix Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    @param index one of 0-8.
    @return Returns value corresponding to index.Returns 0 if out of range.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_MatrixGetValue(
        arg1: *mut OH_Drawing_Matrix,
        index: ::core::ffi::c_int,
    ) -> f32;
    /** @brief Sets matrix to rotate by degrees about a pivot point at (px, py). The pivot point is unchanged
    when mapped with matrix. Positive degrees rotates clockwise.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Matrix Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    @param degree Indicates the angle of axes relative to upright axes.
    @param px Indicates the pivot on x-axis.
    @param py Indicates the pivot on y-axis.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_MatrixRotate(arg1: *mut OH_Drawing_Matrix, degree: f32, px: f32, py: f32);
    /** @brief Sets matrix to translate by (dx, dy)

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Matrix Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    @param dx Indicates the horizontal translation.
    @param dy Indicates the vertical translation.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_MatrixTranslate(arg1: *mut OH_Drawing_Matrix, dx: f32, dy: f32);
    /** @brief Sets matrix to scale by sx and sy, about a pivot point at (px, py).

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Matrix Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    @param sx Indicates the horizontal scale factor.
    @param sy Indicates the vertical scale factor.
    @param px Indicates the pivot on x-axis.
    @param py Indicates the pivot on y-axis.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_MatrixScale(arg1: *mut OH_Drawing_Matrix, sx: f32, sy: f32, px: f32, py: f32);
    /** @brief Sets inverse to reciprocal matrix, returning true if matrix can be inverted.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Matrix Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    @param inverse Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    @return Returns true if the matrix is not nullptr and can be inverted;
            returns false if the matrix is nullptr or cannot be inverted.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_MatrixInvert(
        arg1: *mut OH_Drawing_Matrix,
        inverse: *mut OH_Drawing_Matrix,
    ) -> bool;
    /** @brief Sets the params of matrix to map src to dst.
    Count must greater than or equal to zero, and less than or equal to four.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Matrix Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    @param src Points to map from.
    @param dst Points to map to.
    @param count Number of point in src and dst.
    @return Returns true if matrix is constructed successfully.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_MatrixSetPolyToPoly(
        arg1: *mut OH_Drawing_Matrix,
        src: *const OH_Drawing_Point2D,
        dst: *const OH_Drawing_Point2D,
        count: u32,
    ) -> bool;
    /** @brief Maps the src point array to the dst point array by matrix transformation.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Matrix Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    @param src Points to map from.
    @param dst Points to map to.
    @param count Number of point in src and dst.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_MatrixMapPoints(
        arg1: *const OH_Drawing_Matrix,
        src: *const OH_Drawing_Point2D,
        dst: *mut OH_Drawing_Point2D,
        count: ::core::ffi::c_int,
    );
    /** @brief Sets dst to bounds of src corners mapped by matrix transformation.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Matrix Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    @param src Rect to map from.
    @param dst Rect to map to.
    @return Returns true if the mapped src is equal to the dst; returns false is not equal.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_MatrixMapRect(
        arg1: *const OH_Drawing_Matrix,
        src: *const OH_Drawing_Rect,
        dst: *mut OH_Drawing_Rect,
    ) -> bool;
    /** @brief Returns true if the first matrix equals the second matrix.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Matrix Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    @param other Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    @return Returns true if the two matrices are equal; returns false if not equal.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_MatrixIsEqual(
        arg1: *mut OH_Drawing_Matrix,
        other: *mut OH_Drawing_Matrix,
    ) -> bool;
    /** @brief Returns true if matrix is identity.
    Identity matrix is :  | 1 0 0 |
                          | 0 1 0 |
                          | 0 0 1 |

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Matrix Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    @return Returns true if matrix is identity; returns false if not identity.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_MatrixIsIdentity(arg1: *mut OH_Drawing_Matrix) -> bool;

}

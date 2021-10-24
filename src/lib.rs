use gles31::{
    self as gl,
    types::{GLbitfield, GLenum, GLint, GLuint},
};

pub unsafe fn blit_default_framebuffer_from(
    read_framebuffer: GLuint,
    src_x0: GLint,
    src_y0: GLint,
    src_x1: GLint,
    src_y1: GLint,
    dst_x0: GLint,
    dst_y0: GLint,
    dst_x1: GLint,
    dst_y1: GLint,
    mask: GLbitfield,
    filter: GLenum,
) {
    // prepare render graphics
    gl::Viewport(dst_x0, dst_y0, dst_x1, dst_y1);

    glBlitNamedFramebufferSIM(
        read_framebuffer,
        0,
        src_x0,
        src_y0,
        src_x1,
        src_y1,
        dst_x0,
        dst_y0,
        dst_x1,
        dst_y1,
        mask,
        filter,
    );

    gl::BindFramebuffer(gl::READ_FRAMEBUFFER, 0);
}

/// `BlitNamedFramebuffer`関数のsim。バインドレスではないので使用後の状態に気をつけること。
#[allow(non_snake_case)]
#[inline(always)]
pub unsafe fn glBlitNamedFramebufferSIM(
    read_framebuffer: GLuint,
    draw_framebuffer: GLuint,
    src_x0: GLint,
    src_y0: GLint,
    src_x1: GLint,
    src_y1: GLint,
    dst_x0: GLint,
    dst_y0: GLint,
    dst_x1: GLint,
    dst_y1: GLint,
    mask: GLbitfield,
    filter: GLenum,
) {
    gl::BindFramebuffer(gl::READ_FRAMEBUFFER, read_framebuffer);
    gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, draw_framebuffer);
    gl::BlitFramebuffer(
        src_x0, src_y0, src_x1, src_y1, dst_x0, dst_y0, dst_x1, dst_y1, mask, filter,
    );
}

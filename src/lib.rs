use ::gles31;

pub unsafe fn blit_default_framebuffer_from(
    fbo: gles31::types::GLuint,
    src_x0: gles31::types::GLint,
    src_y0: gles31::types::GLint,
    src_x1: gles31::types::GLint,
    src_y1: gles31::types::GLint,
    dst_x0: gles31::types::GLint,
    dst_y0: gles31::types::GLint,
    dst_x1: gles31::types::GLint,
    dst_y1: gles31::types::GLint,
    mask: gles31::types::GLbitfield,
    filter: gles31::types::GLenum,
) {
    use gles31 as gl;

    // prepare render graphics
    gl::Viewport(dst_x0, dst_y0, dst_x1, dst_y1);

    gl::BindFramebuffer(gl::READ_FRAMEBUFFER, fbo);
    gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, 0);
    gl::BlitFramebuffer(
        src_x0, src_y0, src_x1, src_y1, dst_x0, dst_y0, dst_x1, dst_y1, mask, filter,
    );

    gl::BindFramebuffer(gl::READ_FRAMEBUFFER, 0);
}

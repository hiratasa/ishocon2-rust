use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext};

pub fn plus1(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h.param(0).unwrap().value().as_i64().unwrap();

    out.write(&(param + 1).to_string())?;
    Ok(())
}

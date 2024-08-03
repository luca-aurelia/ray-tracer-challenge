use palette::Oklcha;

pub fn oklcha(
    normalized_lightness: f32,
    normalized_chroma: f32,
    normalized_hue: f32,
    normalized_alpha: f32,
) -> Oklcha {
    // The palette docs say that chroma is unbounded, but ChatGPT says that in practice
    // a range of 0.0 to 1.5 represents an extended upper bound.
    Oklcha::new(
        normalized_lightness,
        normalized_chroma * 1.5,
        normalized_hue * 360.0,
        normalized_alpha,
    )
}

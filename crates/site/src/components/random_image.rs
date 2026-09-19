use crate::components::markdown::Picture;
use crate::markdown::CustomComponent;
use crate::markdown::Image;
#[cfg(feature = "ssr")]
use crate::markdown::{PageTracer, Params};
use leptos::prelude::*;
use leptos_meta::Html;
use rand::prelude::IteratorRandom;

#[derive(Debug)]
pub struct RandomImage {
    images: Vec<Image>,
}

impl CustomComponent for RandomImage {
    #[cfg(feature = "ssr")]
    fn try_from_params(
        mut params: Params,
        body: &str,
        tracer: &mut PageTracer,
    ) -> crate::Result<Self> {
        let Some(alt) = params.take("alt") else {
            return Err(crate::Error::InvalidMarkdown(
                "random_image: alt is missing",
            ));
        };
        params.finish("random_image")?;
        let number = tracer.img();
        let images = body
            .lines()
            .map(str::trim)
            .filter(|dest_url| !dest_url.is_empty())
            .map(|dest_url| {
                use crate::markdown::content_dir;

                let source = content_dir().join("assets").join(dest_url);
                let (lqip, set, width, height, gradient) = Image::preprocess(&source, number)?;
                Ok(Image {
                    path: format!("/assets/{dest_url}"),
                    alt: alt.clone(),
                    title: alt.clone(),
                    caption: vec![],
                    lqip,
                    set,
                    width,
                    height,
                    number,
                    gradient,
                })
            })
            .collect::<crate::Result<Vec<_>>>()?;
        if images.is_empty() {
            return Err(crate::Error::InvalidMarkdown(
                "at least one image must be provided",
            ));
        }
        Ok(Self { images })
    }

    fn page_image(&self) -> Option<&Image> {
        self.images.first()
    }

    fn view(&self) -> AnyView {
        let Some(image) = self.images.iter().choose(&mut rand::rng()).cloned() else {
            return ().into_any();
        };
        let background = image
            .gradient
            .as_deref()
            .map(crate::components::markdown::gradient_style)
            .unwrap_or_default();
        view! {
            <Html {..} style=background />
            <Picture image block=true />
        }
        .into_any()
    }
}

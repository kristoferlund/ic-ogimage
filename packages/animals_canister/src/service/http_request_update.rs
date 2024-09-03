use serde_json::json;

use crate::{
    certified_data::{render_index_html, render_ogimage_svg, update_root_hash},
    http::{default_headers, http_error, HttpRequest, HttpResponse},
    ANIMALS, ASSETS,
};

#[ic_cdk::update]
async fn http_request_update(req: HttpRequest) -> HttpResponse {
    let parts: Vec<&str> = req.url.split('/').collect();
    let animal_id = parts[1];

    // Only requests for animal detail pages are supported.
    // The URL should be in the form of "/<animal_id>".
    let animal_id = match animal_id.parse::<u32>() {
        Ok(val) => val,
        Err(_) => return http_error("Invalid URL."),
    };

    let animal = match ANIMALS.with_borrow(|animals| animals.get(&animal_id).cloned()) {
        Some(animal) => animal,
        None => return http_error("Animal not found."),
    };

    let rendered_index_asset = render_index_html(
        format!("/{}", animal_id),
        json!({
            "ogimage": format!("{}/ogimage.png", animal_id),
            "title": format!("{} {} - IC OG Image Example", animal.emoji, animal.name),
            "description": "Generate OG images for IC projects."
        }),
    );

    ASSETS.with_borrow_mut(|certified_assets| {
        certified_assets.certify_asset(rendered_index_asset.clone(), &default_headers());

        let rendered_ogimage_asset = render_ogimage_svg(json!({
            "id": animal_id,
            "emoji": animal.emoji,
            "name": animal.name
        }));
        certified_assets.certify_asset(rendered_ogimage_asset, &default_headers());
    });

    update_root_hash();

    HttpResponse {
        status_code: 200,
        body: rendered_index_asset.content,
        headers: default_headers(),
        upgrade: None,
    }
}

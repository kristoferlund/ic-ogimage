use crate::{http::default_headers, ASSETS, SIGNATURES};
use asset_util::{collect_assets, Asset, CertifiedAssets, ContentEncoding, ContentType};
use canister_sig_util::signature_map::LABEL_SIG;
use handlebars::Handlebars;
use ic_cdk::api::set_certified_data;
use ic_certification::{fork_hash, labeled_hash};
use include_dir::{include_dir, Dir};
use serde_json::{json, Value};

pub static ASSET_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../animals_frontend/dist");

pub fn update_root_hash() {
    SIGNATURES.with_borrow(|sigs| {
        ASSETS.with_borrow(|assets| {
            let prefixed_root_hash = fork_hash(
                &assets.root_hash(),
                &labeled_hash(LABEL_SIG, &sigs.root_hash()),
            );
            set_certified_data(&prefixed_root_hash[..]);
        })
    })
}

pub fn render_index_html(url_path: String, mut data: Value) -> Asset {
    let handlebars = Handlebars::new();
    let canister_id = ic_cdk::api::id().to_string();
    data.as_object_mut()
        .unwrap()
        .insert("canister_id".to_string(), json!(canister_id));
    let index_file = ASSET_DIR.get_file("index.html").unwrap();
    let index_content_rendered = handlebars
        .render_template(index_file.contents_utf8().unwrap(), &data)
        .unwrap();

    Asset {
        url_path,
        content: index_content_rendered.clone().into_bytes(),
        encoding: ContentEncoding::Identity,
        content_type: ContentType::HTML,
    }
}

pub fn render_ogimage_svg(data: Value) -> Asset {
    let handlebars = Handlebars::new();
    let ogimage_template = include_str!("ogimage_template.svg");
    let ogimage_content_rendered = handlebars.render_template(ogimage_template, &data).unwrap();

    Asset {
        url_path: format!("/{}/ogimage.svg", data["id"].as_u64().unwrap()),
        content: ogimage_content_rendered.into_bytes(),
        encoding: ContentEncoding::Identity,
        content_type: ContentType::SVG,
    }
}

pub fn init_assets() {
    let assets = collect_assets(&ASSET_DIR, None);

    ASSETS.with_borrow_mut(|certified_assets| {
        *certified_assets = CertifiedAssets::certify_assets(assets, &default_headers());

        let rendered_index_asset = render_index_html(
            "/".to_string(),
            json!({
                "ogimage": "ogimage.svg",
                "title": "IC OG Image Example",
                "description": "Generate OG images for IC projects."
            }),
        );

        certified_assets.certify_asset(rendered_index_asset, &default_headers());

        let ogimage_default = include_str!("ogimage_default.svg");
        let ogimage_asset = Asset {
            url_path: "/ogimage.svg".to_string(),
            content: ogimage_default.into(),
            encoding: ContentEncoding::Identity,
            content_type: ContentType::SVG,
        };
        certified_assets.certify_asset(ogimage_asset, &default_headers());
    });

    update_root_hash()
}
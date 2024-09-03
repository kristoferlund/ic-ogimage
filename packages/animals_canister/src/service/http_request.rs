use crate::{
    http::{default_headers, HttpRequest, HttpResponse},
    ASSETS, SIGNATURES,
};
use canister_sig_util::signature_map::LABEL_SIG;
use ic_certification::{labeled_hash, pruned};

#[ic_cdk::query]
pub fn http_request(req: HttpRequest) -> HttpResponse {
    let sigs_root_hash =
        SIGNATURES.with_borrow(|sigs| pruned(labeled_hash(LABEL_SIG, &sigs.root_hash())));

    if let Some(asset) = ASSETS.with_borrow(|assets| {
        assets.get_certified_asset(
            &req.url,
            req.certificate_version,
            Some(sigs_root_hash.clone()),
        )
    }) {
        let mut headers = default_headers();
        headers.extend(asset.headers);
        return HttpResponse {
            status_code: 200,
            body: asset.content,
            headers,
            upgrade: None,
        };
    }

    // If asset is not found, return 404 and upgrade the connection to an update call
    HttpResponse {
        status_code: 404,
        headers: default_headers(),
        body: vec![],
        upgrade: Some(true),
    }
}

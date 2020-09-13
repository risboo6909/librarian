use strip_markdown::strip_markdown;

fn remove_md_tags(body: &str) -> String {
    let stripped = strip_markdown(body);
    stripped
}

pub(crate) fn post_proc(uri: &str, body: &str) -> String {
    if uri.ends_with(".md") {
        return remove_md_tags(body);
    }

    body.to_owned()
}

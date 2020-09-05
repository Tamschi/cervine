#[test]
fn changelog() {
	// This will become less useful with patches, so I'm on the lookout for a crate that lets me test major, minor and revision independently.
	version_sync::assert_contains_regex!("CHANGELOG.md", "^## {version}$");
}

#[test]
fn html_root_url() {
	version_sync::assert_html_root_url_updated!("src/lib.rs");
}

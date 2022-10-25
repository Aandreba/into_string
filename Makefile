check:
	cargo check --no-default-features
	cargo check --no-default-features --features std
	cargo check --no-default-features --features max
	cargo check --no-default-features --features std,max

doc:
	cargo rustdoc --open --all-features -- --cfg docsrs
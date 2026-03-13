deploy:
	cargo lambda build --release --output-format zip --arm64
	serverless deploy

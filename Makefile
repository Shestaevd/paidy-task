.PHONY: build
build:
	docker build --network=host -t paidy-restaurant-image:0.1 .
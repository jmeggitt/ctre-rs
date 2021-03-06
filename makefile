.PHONY: reload-phoenix build-phoenix all

reload-phoenix:
	git submodule sync
	git submodule update --init --recursive

build-phoenix: reload-phoenix
	cd Phoenix-api; rm -rf arm
	cd Phoenix-api; ./gradlew CTRE_PhoenixSharedLibrary

all: build-phoenix
	cp Phoenix-api/arm/cpp/build/libs/cTRE_Phoenix/shared/* ctre-sys/lib/
	cargo build

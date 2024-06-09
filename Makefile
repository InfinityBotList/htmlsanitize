CDN_PATH := /silverpelt/cdn/ibl

all:
	cargo build --release

restartwebserver:
	cargo sqlx prepare
	make all
	make restartwebserver_nobuild

restartwebserver_nobuild:
	systemctl stop htmlsanitize
	sleep 3 # Give time for it to stop
	cp -v target/release/htmlsanitize htmlsanitize
	systemctl start htmlsanitize
ts:
	rm -rvf $(CDN_PATH)/dev/bindings/htmlsanitize
	cargo test
	cp -rf bindings/.generated $(CDN_PATH)/dev/bindings/htmlsanitize

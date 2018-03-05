
DIR=eg-amp-rust.lv2
TARGET=target/release/examples/libeg_amp_rust.so
PREFIX=/usr/local

.PHONY: all clean install

all: $(DIR)/libeg_amp_rust.so $(DIR)/eg-amp-rust.ttl $(DIR)/manifest.ttl

clean:
	rm -r $(DIR)

install:
	install -d /usr/local/lib/lv2/eg-amp-rust.lv2
	install $(DIR)/libeg_amp_rust.so $(DIR)/eg-amp-rust.ttl $(DIR)/manifest.ttl /usr/local/lib/lv2/eg-amp-rust.lv2

$(DIR)/libeg_amp_rust.so: $TARGET
	mkdir -p eg-amp-rust.lv2
	cp $(TARGET) $(DIR)/

$TARGET:
	cargo build --release --examples

$(DIR)/eg-amp-rust.ttl: examples/eg-amp-rust.ttl
	cp examples/eg-amp-rust.ttl $(DIR)/eg-amp-rust.ttl

$(DIR)/manifest.ttl: examples/manifest.ttl
	cp examples/manifest.ttl $(DIR)/manifest.ttl

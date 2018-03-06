
NAME=eg-amp.lv2
TARGET=target/release/examples/libeg_amp.so
PREFIX=/usr/local

.PHONY: all clean install

all: $(NAME)/libeg_amp.so $(NAME)/eg-amp.ttl $(NAME)/manifest.ttl

clean:
	rm -r $(NAME)

install:
	install -d /usr/local/lib/lv2/eg-amp.lv2
	install $(NAME)/libeg_amp.so $(NAME)/eg-amp.ttl $(NAME)/manifest.ttl /usr/local/lib/lv2/eg-amp.lv2

$(NAME)/libeg_amp.so: $(TARGET)
	mkdir -p eg-amp.lv2
	cp $(TARGET) $(NAME)/

$(TARGET): examples/eg-amp/eg_amp.rs
	cargo build --release --examples

$(NAME)/eg-amp.ttl: examples/eg-amp/eg-amp.ttl
	cp examples/eg-amp/eg-amp.ttl $(NAME)/eg-amp.ttl

$(NAME)/manifest.ttl: examples/eg-amp/manifest.ttl
	cp examples/eg-amp/manifest.ttl $(NAME)/manifest.ttl

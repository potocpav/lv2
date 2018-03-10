
AMP_NAME=eg-amp.lv2
AMP_TARGET=target/debug/examples/libeg_amp.so

SAMPLER_NAME=eg-sampler.lv2
SAMPLER_UI_TARGET=target/debug/examples/libsampler_ui.so
SAMPLER_TARGET=target/debug/examples/libsampler.so
PREFIX=/usr/local

.PHONY: all clean install

all: $(AMP_NAME)/libeg_amp.so $(AMP_NAME)/eg_amp.ttl $(AMP_NAME)/manifest.ttl $(SAMPLER_NAME)/libsampler.so $(SAMPLER_NAME)/libsampler_ui.so $(SAMPLER_NAME)/sampler.ttl $(SAMPLER_NAME)/manifest.ttl

clean:
	rm -r $(AMP_NAME)
	rm -r $(SAMPLER_NAME)

install:
	install -d /usr/local/lib/lv2/$(AMP_NAME)
	install $(AMP_NAME)/libeg_amp.so $(AMP_NAME)/eg_amp.ttl $(AMP_NAME)/manifest.ttl /usr/local/lib/lv2/$(AMP_NAME)
	install -d /usr/local/lib/lv2/$(SAMPLER_NAME)
	install $(SAMPLER_NAME)/libsampler.so $(SAMPLER_NAME)/libsampler_ui.so $(SAMPLER_NAME)/sampler.ttl $(SAMPLER_NAME)/manifest.ttl /usr/local/lib/lv2/$(SAMPLER_NAME)

$(AMP_NAME)/libeg_amp.so: $(AMP_TARGET)
	mkdir -p eg-amp.lv2
	cp $(AMP_TARGET) $(AMP_NAME)/

$(AMP_TARGET): examples/eg-amp/eg_amp.rs
	cargo build --debug --example eg_amp

$(AMP_NAME)/eg_amp.ttl: examples/eg-amp/eg_amp.ttl
	cp examples/eg-amp/eg_amp.ttl $(AMP_NAME)/eg_amp.ttl

$(AMP_NAME)/manifest.ttl: examples/eg-amp/manifest.ttl
	cp examples/eg-amp/manifest.ttl $(AMP_NAME)/manifest.ttl


$(SAMPLER_NAME)/libsampler.so: $(SAMPLER_TARGET)
	mkdir -p eg-sampler.lv2
	cp $(SAMPLER_TARGET) $(SAMPLER_NAME)/
$(SAMPLER_NAME)/libsampler_ui.so: $(SAMPLER_UI_TARGET)
	mkdir -p eg-sampler.lv2
	cp $(SAMPLER_UI_TARGET) $(SAMPLER_NAME)/

$(SAMPLER_TARGET): examples/eg-sampler/sampler.rs
	cargo build --debug --example sampler
$(SAMPLER_UI_TARGET): examples/eg-sampler/sampler_ui.rs
	cargo build --debug --example sampler_ui

$(SAMPLER_NAME)/sampler.ttl: examples/eg-sampler/sampler.ttl
	cp examples/eg-sampler/sampler.ttl $(SAMPLER_NAME)/sampler.ttl

$(SAMPLER_NAME)/manifest.ttl: examples/eg-sampler/manifest.ttl
	cp examples/eg-sampler/manifest.ttl $(SAMPLER_NAME)/manifest.ttl

ifeq ($(strip $(LIBTRANSISTOR_HOME)),)
$(error "Please set LIBTRANSISTOR_HOME in your environment. export LIBTRANSISTOR_HOME=<path to libtransistor>")
endif

PROGRAM := oxidgb
OBJ := target/aarch64-unknown-switch/release/liboxidgb_switch.a

include $(LIBTRANSISTOR_HOME)/libtransistor.mk

all: $(PROGRAM).nso $(PROGRAM).nro

target/aarch64-unknown-switch/release/liboxidgb_switch.a:
	xargo build --target aarch64-unknown-switch --release

$(PROGRAM).nso.so: ${OBJ} $(LIBTRANSITOR_NSO_LIB) $(LIBTRANSISTOR_COMMON_LIBS)
	$(LD) $(LD_FLAGS) -lm -o $@ ${OBJ} $(LIBTRANSISTOR_NSO_LDFLAGS)

$(PROGRAM).nro.so: ${OBJ} $(LIBTRANSITOR_NRO_LIB) $(LIBTRANSISTOR_COMMON_LIBS)
	$(LD) $(LD_FLAGS) -lm -o $@ ${OBJ} $(LIBTRANSISTOR_NRO_LDFLAGS)

clean:
	cargo clean
	rm -rf *.o *.nso *.nro *.so
RUSTC = rustc
CLANG = clang
GFORTH = gforth

TARGET = vanilla

all: $(TARGET)

src/helper.o: src/helper.c
	$(CLANG) -c src/helper.c -o src/helper.o

$(TARGET): src/vanilla.rs src/helper.o
	$(RUSTC) src/vanilla.rs -o $(TARGET) src/helper.o

run-forth:
	$(GFORTH) src/true_false.fs -e "bye"

clean:
	rm -f src/*.o $(TARGET)

.PHONY: all clean run-forth

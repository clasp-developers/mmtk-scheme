
all:
	(make -C scheme all)
	(make -C mmtk all)
	mkdir -p build/rust-objects
	(cd build/rust-objects; ar x ../../mmtk/target/debug/libmmtkscheme.a)
	llvm-ar-16 rcs build/libmmtkscheme.a build/rust-objects/*.o scheme/clayer.o
	llvm-ranlib-16 build/libmmtkscheme.a

clean:
	rm -rf build/ *~
	(cd scheme; rm -f *.o *~)
	(cd mmtk; make clean)


set -e
DIR=${PWD##*/src/}
TARGET=lib${DIR}.a
rm -fr ${TARGET} *.o
for i in *.c; 
do 
	gcc -c -fPIC -O2 -g \
		-Wall \
		-Wdeclaration-after-statement \
		-Wredundant-decls \
		-D_REENTRANT \
		$( for i in `ls -d ../*/`; do echo "-I${i}"; done   | xargs )  \
		-I/usr/include/SDL2 \
		$i -o ${i%%.c}.o; 
done
ar crus ${TARGET} *.o
set +e

set -e 
find . -iname "*.o" -delete  -or -iname "*.a" -delete

for i in `ls -d */`; 
do 
	cd ${i} && \
	bash -x ../build.sh && \
	cd ..; 
done
set +e

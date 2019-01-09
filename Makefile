.PHONY: clean

TARGET-DIR=l-systems-material
BOOK=${TARGET-DIR}/book
ARCHIVE=${TARGET-DIR}.tar.gz

${ARCHIVE}: ${BOOK}
	tar cvfz $@ ${TARGET-DIR}

${BOOK}: ${TARGET-DIR}
	cp -r docs $@

${TARGET-DIR}:
	mkdir -p $@

clean:
	rm -rf ${ARCHIVE} ${TARGET-DIR}

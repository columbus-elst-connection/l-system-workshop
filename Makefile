.PHONY: clean

TARGET-DIR=l-systems-material
BOOK-SOURCE=book
BOOK-RESULT=docs
BOOK-TARGET=${TARGET-DIR}/${BOOK-SOURCE}
CODE-RESULT=code
CODE-TARGET=${TARGET-DIR}/${CODE-RESULT}
ARCHIVE=${TARGET-DIR}.tar.gz
VENDOR-RESULT=${CODE-RESULT}/vendor

${ARCHIVE}: ${BOOK-TARGET} ${CODE-TARGET}
	tar cvfz $@ ${TARGET-DIR}

${BOOK-TARGET}: ${BOOK-RESULT} ${TARGET-DIR}
	cp -r $< $@

${BOOK-RESULT}:
	cd ${BOOK-SOURCE} && mdbook build

${CODE-TARGET}: ${CODE-RESULT} ${TARGET-DIR} ${VENDOR-RESULT}
	find code -type d -name target | xargs rm -rf
	cp -r $< $@

${TARGET-DIR}:
	mkdir -p $@

${VENDOR-RESULT}:
	cd code && cargo vendor

clean:
	rm -rf ${ARCHIVE} ${TARGET-DIR} ${BOOK-RESULT} ${VENDOR-RESULT}

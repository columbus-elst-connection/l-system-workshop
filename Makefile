.PHONY: clean

TARGET-DIR=l-systems-material
BOOK-SOURCE=book
BOOK-RESULT=docs
BOOK-TARGET=${TARGET-DIR}/book
ARCHIVE=${TARGET-DIR}.tar.gz

${ARCHIVE}: ${BOOK-TARGET}
	tar cvfz $@ ${TARGET-DIR}

${BOOK-TARGET}: ${BOOK-RESULT} ${TARGET-DIR}
	cp -r $< $@

${BOOK-RESULT}: 
	cd ${BOOK-SOURCE} && mdbook build

${TARGET-DIR}:
	mkdir -p $@

clean:
	rm -rf ${ARCHIVE} ${TARGET-DIR} ${BOOK-RESULT}

SUnit_FILES = \

$(SUnit_FILES):
$(srcdir)/packages/sunit/stamp-classes: $(SUnit_FILES)
	touch $(srcdir)/packages/sunit/stamp-classes

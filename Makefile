DOCKER=docker

out/debian/striptags_0.1.0_amd64.deb: out
	$(DOCKER) run --rm -it -v $(CURDIR):/src:ro -v $(CURDIR)/out:/out -e CARGO_TARGET_DIR=/out striptags cargo deb

out:
	mkdir out

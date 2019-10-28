DOCKER=docker
VERSION=0.1.1-2

out/debian/striptags_${VERSION}_amd64.deb: out
	$(DOCKER) run --rm -it -v $(CURDIR):/src:ro -v $(CURDIR)/out:/out -e CARGO_TARGET_DIR=/out striptags cargo deb -- --locked

out:
	mkdir out

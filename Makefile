MAKEFLAGS += --no-print-directory

.PHONY: setup setup-hooks check check-fmt check-clippy test prettier markdownlint ci-check lint fmt build clean install

setup: setup-hooks
	@echo "Development environment ready."

setup-hooks:
	@command -v prek >/dev/null 2>&1 || { \
		echo "prek is not installed. Install it first:"; \
		echo "  cargo install prek"; \
		exit 1; \
	}
	prek install
	@echo "Git hooks installed via prek."

check: check-fmt check-clippy

check-fmt:
	cargo fmt --check --all

check-clippy:
	cargo clippy --locked --all-targets --all-features -- -D warnings

test:
	cargo test --locked --workspace --all-targets --all-features

prettier:
	prek run --all-files --verbose prettier

markdownlint:
	prek run --all-files --verbose markdownlint-cli2

ci-check:
	@echo "━━━ 🖊️ Prettier ━━━"
	$(MAKE) prettier
	@echo ""
	@echo "━━━ 📝 Markdownlint ━━━"
	$(MAKE) markdownlint
	@echo ""
	@echo "━━━ 🎨 Rustfmt ━━━"
	$(MAKE) check-fmt
	@echo ""
	@echo "━━━ 📎 Clippy ━━━"
	$(MAKE) check-clippy
	@echo ""
	@echo "━━━ 🧪 Tests ━━━"
	$(MAKE) test
	@echo ""
	@echo "━━━ ✅ CI Check Passed ━━━"

lint: fmt prettier markdownlint
	cargo clippy --fix --locked --workspace --all-targets --all-features --allow-dirty --allow-staged -- -D warnings

fmt:
	cargo fmt --all

build:
	cargo build --locked --release

clean:
	cargo clean

install:
	cargo install --locked --path .

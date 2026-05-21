# Type generation via type-crafter (npx).
# See CLAUDE.md for details.

STOREFRONT_SPEC := types/storefront/index.yaml
STOREFRONT_OUT  := src/storefront/generated/types

.PHONY: gen-storefront clean-storefront

## gen-storefront: Regenerate storefront types from YAML specs.
gen-storefront:
	npx type-crafter@latest generate rust $(STOREFRONT_SPEC) $(STOREFRONT_OUT) SingleFile SingleFile
	sed -i '' '/^pub mod mod;$$/d' $(STOREFRONT_OUT)/mod.rs
	cargo fmt

## clean-storefront: Remove all generated storefront type files.
clean-storefront:
	rm -rf $(STOREFRONT_OUT)
	mkdir -p $(STOREFRONT_OUT)

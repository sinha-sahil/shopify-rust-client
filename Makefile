# Type generation via type-crafter (npx).
# See CLAUDE.md for details.

STOREFRONT_SPEC := types/storefront/index.yaml
STOREFRONT_OUT  := src/storefront/generated/types

ADMIN_SPEC := types/admin/index.yaml
ADMIN_OUT  := src/admin/generated/types

.PHONY: gen-storefront clean-storefront gen-admin clean-admin

## gen-storefront: Regenerate storefront types from YAML specs.
gen-storefront:
	npx type-crafter@latest generate rust $(STOREFRONT_SPEC) $(STOREFRONT_OUT) SingleFile SingleFile
	sed -i '' '/^pub mod mod;$$/d' $(STOREFRONT_OUT)/mod.rs
	cargo fmt

## clean-storefront: Remove all generated storefront type files.
clean-storefront:
	rm -rf $(STOREFRONT_OUT)
	mkdir -p $(STOREFRONT_OUT)

## gen-admin: Regenerate admin types from YAML specs.
gen-admin:
	npx type-crafter@latest generate rust $(ADMIN_SPEC) $(ADMIN_OUT) SingleFile SingleFile
	sed -i '' '/^pub mod mod;$$/d' $(ADMIN_OUT)/mod.rs
	cargo fmt

## clean-admin: Remove all generated admin type files.
clean-admin:
	rm -rf $(ADMIN_OUT)
	mkdir -p $(ADMIN_OUT)

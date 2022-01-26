debug:
	cd web && npx webpack --progress --mode development

release:
	cd web && npx webpack --progress --mode production

setup:
	cd web && npm i

crate:
	wasm-pack build --target web --out-dir web/src/pkg

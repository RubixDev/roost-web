GRAMMAR_FILE = web/src/roost-grammar.ts
CONFIG_FILE = web/src/roost-config.ts

debug: pull
	cd web && npx webpack --progress --mode development

release: pull
	cd web && npx webpack --progress --mode production

pull:
	cargo update
	cd web && npm i
	curl https://raw.githubusercontent.com/RubixDev/vscode-roost/master/syntaxes/roost.tmLanguage.json > $(GRAMMAR_FILE)
	sed -i '1i export default JSON.stringify(' $(GRAMMAR_FILE)
	echo ")" >> $(GRAMMAR_FILE)
	curl https://raw.githubusercontent.com/RubixDev/vscode-roost/master/language-configuration.json > $(CONFIG_FILE)
	sed -i '1i export default' $(CONFIG_FILE)

crate:
	wasm-pack build --target web --out-dir web/src/pkg

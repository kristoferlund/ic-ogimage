build-frontend:
	dfx generate animals_canister
	npm i
	npm run build

build-canister:
	cargo build

deploy: build-frontend build-canister
	dfx deploy

deploy-ic: build-frontend build-canister
	dfx deploy --ic

clean:
	rm -rf .dfx
	rm -rf node_modules
	rm -rf packages/animals_frontend/dist
	rm -rf packages/animals_frontend/node_modules
	rm -rf packages/animals_frontend/src/declarations
	rm -rf target
	rm -f .env
	cargo clean

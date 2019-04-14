#!/usr/bin/env node

// Pinched from: https://raw.githubusercontent.com/IronCoreLabs/recrypt-node-binding/master/publish.js

const fs = require("fs");
const cp = require("child_process");

const exec = async (...args) => {
  console.log("$", ...args);
  const e = cp.exec(...args);
  e.stdout.pipe(process.stdout);
  e.stderr.pipe(process.stdout);
  return new Promise((res, rej) => {
    e.on("close", res);
    e.on("error", rej);
  });
};

try {
  run();
} catch (err) {
  console.log("publish.js FAILED");
  console.log(err);
}

async function run() {
  // Cleanup the previous build, if it exists
  await exec("rm -rf ./dist");
  await exec("rm -rf ./bin-package");
  await exec("rm -rf ./build");

  // Cleanup any previous Rust builds, update deps, and compile
  await exec("yarn install --ignore-scripts");
  await exec("yarn run clean");
  await exec("cd ./native && cargo update");
  await exec("yarn run compile");

  // Copy over package distribution files
  await exec("mkdir dist");
  await exec("cp README.md package.json LICENSE ./dist");

  // Add a NPM install script to the package.json that we push
  // to NPM so that when consumers pull it down it
  // runs the expected node-pre-gyp step.
  const npmPackageJson = require("./dist/package.json");
  npmPackageJson.scripts.install = "node-pre-gyp install --fallback-to-build";
  fs.writeFileSync(
    "./dist/package.json",
    JSON.stringify(npmPackageJson, null, 2)
  );

  // Copy over compiled binaries
  await exec("mkdir bin-package");
  await exec("cp ./native/index.node ./bin-package");
  await exec("./node_modules/.bin/node-pre-gyp package");
  const files = cp
    .execSync("find ./build -name *.tar.gz")
    .toString()
    .split("\n")
    .join(" ");
  await exec(`cp ${files} ./bin-package`);

  // Do publish
  if (process.argv.slice(2).indexOf("--publish") !== -1) {
    await exec("cd dist && npm publish --access public");
  } else {
    console.log("Skipping publishing to npm...");
  }

  console.log("publish.js COMPLETE");
}

const prettier = "prettier -w";
const eslint = "eslint --fix";
const rustfmt = "rustfmt";

module.exports = {
  "*.{js,mjs,cjs,ts,vue}": [eslint, prettier],
  "*.{md,html,css,scss,json,yml,yaml}": prettier,
  "*.rs": rustfmt,
};

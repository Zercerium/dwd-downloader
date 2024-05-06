module.exports = {
  root: true,
  env: {
    browser: true,
    node: true,
  },
  parser: "vue-eslint-parser",
  parserOptions: {
    parser: "@typescript-eslint/parser",
  },
  extends: ["@nuxt/eslint-config", "plugin:prettier/recommended"],
  plugins: [],
  rules: {
    "vue/prop-name-casing": ["warn", "snake_case"],
    "prettier/prettier": ["error", { endOfLine: "auto" }],
    "@typescript-eslint/naming-convention": [
      "error",
      {
        selector: "variable",
        format: ["snake_case"],
        leadingUnderscore: "allow",
      },
    ],
  },
};

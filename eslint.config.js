import js from '@eslint/js';
import { includeIgnoreFile } from '@eslint/compat';
import svelte from 'eslint-plugin-svelte';
import globals from 'globals';
import { fileURLToPath } from 'node:url';
import ts from 'typescript-eslint';
import stylisticJs from '@stylistic/eslint-plugin-js'
const gitignorePath = fileURLToPath(new URL("./.gitignore", import.meta.url));

export default ts.config(
    includeIgnoreFile(gitignorePath),
    js.configs.recommended,
    ...ts.configs.recommended,
    ...svelte.configs["flat/recommended"],
    {
        languageOptions: {
            globals: {
                ...globals.browser,
                ...globals.node
            }
        }
    },
    {
        files: ["**/*.svelte"],

        languageOptions: {
            parserOptions: {
                parser: ts.parser
            }
        },

        plugins: {
            '@stylistic/js': stylisticJs
        },

        rules: {
            "quotes": ["warn", "double"],
            "semi": ["warn", "always"],
            "camelcase": ["warn", { "properties": "always" }],
            "no-unused-vars": "off",
            '@typescript-eslint/no-unused-vars': 'warn',
            "@stylistic/js/object-curly-spacing": ["warn", "always"],
            "@stylistic/js/eol-last": ["warn", "always"],
        }
    }
);

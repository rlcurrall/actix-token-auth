module.exports = {
    extends: ['plugin:vue/vue3-recommended', 'prettier', 'prettier/vue'],
    plugins: ['prettier'],
    rules: { 'prettier/prettier': ['error'], 'vue/html-indent': 'off' },
}

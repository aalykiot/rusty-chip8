module.exports = {
  mount: {
    src: '/dist',
    public: '/',
  },
  devOptions: {
    tailwindConfig: './tailwind.config.js',
  },
  plugins: [
    '@snowpack/plugin-postcss',
    [
      'snowpack-plugin-wasm-pack',
      {
        projectPath: './wasm',
      },
    ],
  ],
};

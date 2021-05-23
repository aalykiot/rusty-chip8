module.exports = {
  mount: {
    src: '/dist',
    public: '/',
  },
  plugins: [
    [
      'snowpack-plugin-wasm-pack',
      {
        projectPath: './wasm',
      },
    ],
  ],
};

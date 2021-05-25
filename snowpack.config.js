module.exports = {
  mount: {
    src: '/dist',
    public: '/',
  },
  exclude: [
    '**/*.rs',
    '**/*.lock',
    '**/*.toml',
    '**/*.md',
    '**/chip8/target/**',
  ],
  devOptions: {
    tailwindConfig: './tailwind.config.js',
  },
  plugins: [
    '@snowpack/plugin-postcss',
    [
      'snowpack-plugin-wasm-pack',
      {
        projectPath: './src/chip8',
      },
    ],
  ],
};

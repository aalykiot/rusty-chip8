module.exports = {
  purge: ['./public/**/*.html'],
  theme: {
    extend: {
      spacing: {
        640: '40rem',
        320: '20rem',
      },
    },
  },
  plugins: [require('@tailwindcss/forms')],
};

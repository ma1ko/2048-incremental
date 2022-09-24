

module.exports = {
  content: ["./src/*"],
  theme: {
    extend: {
	    gridTemplateRows: {
		'7': 'repeat(7, minmax(0, 1fr))',
		'8': 'repeat(8, minmax(0, 1fr))',
		'9': 'repeat(9, minmax(0, 1fr))',
		'10': 'repeat(10, minmax(0, 1fr))',

	    }
    },
  },
  plugins: [],
}



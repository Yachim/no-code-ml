/** @type {import('tailwindcss').Config} */
module.exports = {
	content: [
		"./index.html",
		"./src/**/*.{js,ts,jsx,tsx,svelte}",
	],
	theme: {
		extend: {
			colors: {
				bg: "#191a1e",
				headerBg: "#212226",
				border: "#2c2d30",
				text: "#d9d9d9"
			}
		},
		transitionDuration: {
			DEFAULT: "300ms"
		}
	},
	plugins: [],
}

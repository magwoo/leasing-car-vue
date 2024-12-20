import type { Config } from "tailwindcss";

export default {
  content: ["./src/**/*.{vue,ts}", "./index.html"],
  theme: {
    extend: {
      colors: {
        primary: "#FF9514",
        accent: "#111111",
        neutral: "#F3F3F4",
      },
      fontFamily: {
        montserrat: "Montserrat",
      },
    },
  },
  plugins: [],
} satisfies Config;

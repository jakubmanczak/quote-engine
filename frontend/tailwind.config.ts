import type { Config } from "tailwindcss";

export default {
  content: [
    "./src/pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/components/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/app/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      colors: {
        background: "var(--background)",
        foreground: "var(--foreground)",
        middleground: "var(--middleground)",
        "half-transparent": "var(--half-transparent)",
      },
      fontFamily: {
        fancy: ["var(--font-markazi)", "serif"],
      },
    },
  },
  plugins: [],
} satisfies Config;

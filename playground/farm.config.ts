import { defineConfig } from "@farmfe/core";

export default defineConfig({
  compilation: {
    input: {
      index: "./index.html",
    },
    persistentCache: false,
    progress: false,
  },
  plugins: [
    ["@farmfe/plugin-react", { runtime: "automatic" }],
    [
      "farm-plugin-compression",
      { level: "best", exclude: ["svg", "html", "map"] },
    ],
  ],
});

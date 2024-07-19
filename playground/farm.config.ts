import { defineConfig } from "@farmfe/core";
import react from "@farmfe/plugin-react";
import compression from "farm-plugin-compression";

export default defineConfig({
  compilation: {
    input: {
      index: "./index.html",
    },
    persistentCache: false,
    progress: false,
  },
  plugins: [
    react({ runtime: "automatic" }),
    compression({ level: "best", exclude: ["svg", "html", "map"] }),
  ],
});

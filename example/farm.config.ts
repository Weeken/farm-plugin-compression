import { defineConfig } from "@farmfe/core";
import vue from "@vitejs/plugin-vue";
// import { compression } from "vite-plugin-compression2";

export default defineConfig({
  // compilation: {
  //   output: {
  //     filename: "assets/[name].[hash].[ext]",
  //     assetsFilename: "static/[resourceName].[ext]",
  //   },
  // },
  plugins: [["farm-plugin-compression", { level: "best" }]],
  vitePlugins: [vue()],
});

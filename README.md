## farm-plugin-compression

a rust plugin use [flate2](https://crates.io/crates/flate2) for [farm](https://github.com/farm-fe/farm) to compress resources.

### Install

```bash
pnpm add -D farm-plugin-compression
```

### Usage

```ts
import { defineConfig } from "@farmfe/core";
import vue from "@vitejs/plugin-vue";

export default defineConfig({
  plugins: [
    [
      "farm-plugin-compression",
      { level: "best" }, // level: "best" | "none" | "default" | "fast"
    ],
  ],
});
```

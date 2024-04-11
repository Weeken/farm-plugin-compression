## farm-plugin-compression

A rust plugin for [farm](https://github.com/farm-fe/farm) to compress resources.

### Install

```bash
pnpm add -D farm-plugin-compression
```

### Usage

```ts
import { defineConfig } from "@farmfe/core";

export default defineConfig({
  plugins: [
    [
      "farm-plugin-compression",
      { level: "best" }, // level: "best" | "none" | "default" | "fast"
    ],
  ],
});
```

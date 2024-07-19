## farm-plugin-compression

A rust plugin for [farm](https://github.com/farm-fe/farm) to compress resources.

### Install

```bash
pnpm add -D farm-plugin-compression
```

> [!NOTE]
> "@farmfe/core": "^1.2.0" --> "farm-plugin-compression": "0.0.4"  
> "@farmfe/core": "^1.3.0" --> "farm-plugin-compression": "^0.1.x"

### Usage

```ts
import { defineConfig } from "@farmfe/core";
import compression from "farm-plugin-compression";

export default defineConfig({
  plugins: [
    [
      "farm-plugin-compression",
      {
        level: "best", // level: "best" | "none" | "default" | "fast"
        exclude: ["jpg", "png"], // resource type like jpg, png, and others generate in dist
      },
    ],
    // or
    compression({ level: "best", exclude: ["jpg", "png"] }),
  ],
});
```

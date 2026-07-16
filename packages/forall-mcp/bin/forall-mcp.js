#!/usr/bin/env node
import { main } from "../src/index.js";

main().catch((error) => {
  console.error(error instanceof Error ? error.message : error);
  process.exit(1);
});

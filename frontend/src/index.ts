import { serve } from "bun";
import index from "./index.html";

const server = serve({
  routes: {
    "/*": index,
    "/api/*": async (req) => {
      const target = new URL(req.url);
      target.hostname = "localhost";
      target.port = "3000";
      return fetch(target.toString(), req);
    },
  },
  development: process.env.NODE_ENV !== "production" && {
    hmr: true,
    console: true,
  },
});

console.log(`🚀 ODIN frontend running at ${server.url}`);

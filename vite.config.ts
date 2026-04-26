import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import path from "path";

export default defineConfig({
    plugins: [
        react({
            jsxRuntime: "classic",
            jsxImportSource: undefined,
            jsxFactory: "React.createElement",
            jsxFragment: "React.Fragment",
        }),
    ],
    clearScreen: false,
    server: {
        port: 1420,
        strictPort: true,
        watch: {
            ignored: ["**/src-tauri/**"],
        },
    },
    build: {
        outDir: "dist",
        emptyOutDir: true,
        sourcemap: true,
        rollupOptions: {
            input: {
                index: path.resolve(__dirname, "src/frontend/index.html"),
            },
        },
    },
    resolve: {
        alias: {
            "@": path.resolve(__dirname, "src/frontend"),
        },
    },
});

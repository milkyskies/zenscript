export interface FloeOptions {
    /** Path to the floe binary. Defaults to "floe". */
    compiler?: string;
}
/**
 * Vite plugin for Floe.
 *
 * Transforms `.fl` files to TypeScript in the build pipeline.
 * Uses the `floe` compiler binary for compilation.
 *
 * @example
 * ```ts
 * import { defineConfig } from "vite"
 * import floe from "@floeorg/vite-plugin"
 *
 * export default defineConfig({
 *   plugins: [floe()],
 * })
 * ```
 */
export default function floe(options?: FloeOptions): {
    name: string;
    enforce: "pre";
    config(config: {
        resolve?: {
            extensions?: string[];
        };
    }): {
        resolve: {
            extensions: string[];
        };
        esbuild: {
            include: RegExp;
            loader: "tsx";
        };
    };
    transform(this: {
        error(msg: string): never;
    }, code: string, id: string): {
        code: string;
        map: string | null;
        moduleType: string;
    } | null;
    handleHotUpdate({ file, server }: {
        file: string;
        server: {
            moduleGraph: {
                getModulesByFile(file: string): Set<any> | undefined;
            };
        };
    }): any[] | undefined;
};

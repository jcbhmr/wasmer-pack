import {bindings, commands} from "@wasmer/wabt";
import {
    WASM_FEATURE_SIMD,
    WASM_FEATURE_BULK_MEMORY,
} from "@wasmer/wabt/src/bindings/wabt/wabt.js";
// @ts-ignore
import { WASI, init as initWasi } from "@wasmer/wasi";
import {} from "jasmine";

describe("Generated WASI bindings", () => {
    beforeAll(() => initWasi());

    it("can use the wabt library", async () => {
        const wabt = await bindings.wabt();

        const result = wabt.wat2wasm(
            "(module)",
            WASM_FEATURE_SIMD | WASM_FEATURE_BULK_MEMORY
        );

        expect(result.tag).toBe("ok");
    });

    it("can invoke the wat2wasm executable", async () => {
        const wasi = new WASI({
            args: ["wat2wasm", "--help"],
        });

        try {
            const exitStatus = await commands.wat2wasm({ wasi });

            expect(exitStatus).toEqual({ code: 123 });
        } catch {
            // For some reason, non-zero exit codes trigger an exception. Ignore
            // it...
        }

        const stdout = wasi.getStdoutString();
        expect(stdout).toContain("usage: wat2wasm [options] filename");
        expect(stdout).toContain(
            "read a file in the wasm text format, check it for errors, and"
        );
        const stderr = wasi.getStderrString();
        expect(stderr).toEqual("");
    });
});

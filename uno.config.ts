// uno.config.ts
import {
    defineConfig,
    presetAttributify,
    presetIcons,
    presetTypography,
    presetUno,
    presetWebFonts,
    transformerDirectives,
    transformerVariantGroup,
} from "unocss";

export default defineConfig({
    shortcuts: {
        // shortcuts to multiple utilities
        'btn': 'py-2 px-4 font-semibold rounded-lg shadow-md',
        'btn-green': 'text-white bg-green-500 hover:bg-green-700',
        // single utility alias
        'red': 'text-red-100',

        "flex-center": "flex justify-center items-center",
        "flex-x-center": "flex justify-center",
        "flex-y-center": "flex items-center",
        "wh-full": "w-full h-full",
        "flex-x-between": "flex items-center justify-between",
        "flex-x-end": "flex items-center justify-end",
        "absolute-lt": "absolute left-0 top-0",
        "absolute-rt": "absolute right-0 top-0 ",
        "fixed-lt": "fixed left-0 top-0",
    },
    theme: {
        colors: {
            primary: "var(--el-color-primary)",
            primary_dark: "var(--el-color-primary-light-5)",
        },
    },
    presets: [
        presetUno(),
        presetAttributify(),
        presetIcons(),
        presetTypography(),
        presetWebFonts({
            fonts: {
                // ...
            },
        }),
    ],
    transformers: [transformerDirectives(), transformerVariantGroup()],
});

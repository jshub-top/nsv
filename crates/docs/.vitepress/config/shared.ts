import { InlineLinkPreviewElementTransform } from "@nolebase/vitepress-plugin-inline-link-preview/markdown-it";
import Mark from "markdown-it-mark";
import { defineConfig } from "vitepress";

import { search as zhSearch } from "./zh";

export const shared = defineConfig({
    title: "Nsv",

    rewrites: {
        "en/:rest*": ":rest*",
    },

    lastUpdated: true,
    cleanUrls: true,
    metaChunk: true,

    markdown: {
        lineNumbers: false,
        config(md) {
            md.use(InlineLinkPreviewElementTransform);
            md.use(Mark);
        },
    },

    sitemap: {
        hostname: "https://nsv.jshub.top",
        transformItems(items) {
            return items.filter((item) => !item.url.includes("migration"));
        },
    },

    head: [
        ["link", { rel: "icon", type: "image/svg+xml", href: "/logo-n.svg" }],
        ["meta", { name: "theme-color", content: "#8c6bef" }],
        ["meta", { property: "og:type", content: "website" }],
        ["meta", { property: "og:locale", content: "en" }],
        [
            "meta",
            {
                property: "og:title",
                content: "Aide | Conquer Any Code in VSCode",
            },
        ],
        ["meta", { property: "og:site_name", content: "Aide" }],
        [
            "meta",
            {
                property: "og:image",
                content: "https://nsv.jshub.top/logo.nsv",
            },
        ],
        ["meta", { property: "og:url", content: "https://nsv.jshub.top/" }],
    ],

    themeConfig: {
        logo: { src: "/logo.svg", width: 50, height: 24 },

        socialLinks: [
            { icon: "github", link: "https://github.com/jshub-top/nsv" },
        ],

        search: {
            provider: "local",
        },
    },

    vite: {
        optimizeDeps: {
            exclude: [
                "@nolebase/vitepress-plugin-inline-link-preview/markdown-it",
            ],
        },
        ssr: {
            noExternal: ["@nolebase/*"],
        },
    },
});

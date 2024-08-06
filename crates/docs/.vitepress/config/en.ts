import { createRequire } from "module";
import type { DefaultTheme } from "vitepress";
import { defineConfig } from "vitepress";

const require = createRequire(import.meta.url);

export const en = defineConfig({
    lang: "en-US",
    description:
        "Conquer Any Code in VSCode: One-Click Comments, Conversions, UI-to-Code, and AI Batch Processing! 💪",
    themeConfig: {
        footer: {
            message: "Released under the MIT License.",
            copyright: "Copyright © 2024-present Jshub.Top",
        },
        nav: nav(),
        sidebar: sidebar(),
        editLink: {
            pattern:
                "https://github.com/nicepkg/aide/edit/master/website/:path",
            text: "Edit this page on GitHub",
        },
    },
});

function nav(): DefaultTheme.NavItem[] {
    return [
        {
            text: "Guide",
            link: "/guide/getting-started/",
            activeMatch: "/guide/",
        },
        {
            text: "Development",
            items: [
                {
                    text: "Install" + " ",
                    link: "https://marketplace.visualstudio.com/items?itemName=nicepkg.aide-pro",
                },
                {
                    text: "Changelog",
                    link: "https://github.com/nicepkg/aide/blob/master/CHANGELOG.md",
                },
                {
                    text: "Contributing",
                    link: "https://github.com/nicepkg/aide/blob/master/CONTRIBUTING.md",
                },
            ],
        },
    ];
}

function sidebar(): DefaultTheme.Sidebar {
    return {
        "/guide/": [
            {
                text: "🚀&nbsp;&nbsp; Getting Started",
                collapsed: false,
                base: "/guide/getting-started",
                items: [
                    { text: "Introduction", link: "/" },
                    { text: "Installation", link: "/installation" },
                    {
                        text: "Customize Shortcuts",
                        link: "/customize-shortcuts",
                    },
                    {
                        text: "Customize Configuration",
                        link: "/customize-configuration",
                    },
                    { text: "FAQ", link: "/faq" },
                ],
            },
        ],
    };
}

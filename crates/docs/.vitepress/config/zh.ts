import { createRequire } from "module";
import { defineConfig, DefaultTheme } from "vitepress";

const require = createRequire(import.meta.url);

export const zh = defineConfig({
    lang: "zh-Hans",
    description: "命令行修改node版本. 💪",
    themeConfig: {
        footer: {
            message: "基于 MIT 许可发布",
            copyright: `Copyright © 2024-present Jshub.Top`,
        },
        nav: nav(),
        sidebar: sidebar(),
        editLink: {
            pattern:
                "https://github.com/jshub-top/nsv/edit/main/crates/docs/:path",
            text: "在 GitHub 上编辑此页面",
        },
        docFooter: {
            prev: "上一页",
            next: "下一页",
        },
        outline: {
            label: "页面导航",
        },

        lastUpdated: {
            text: "最后更新于",
            formatOptions: {
                dateStyle: "short",
                timeStyle: "medium",
            },
        },

        langMenuLabel: "多语言",
        returnToTopLabel: "回到顶部",
        sidebarMenuLabel: "菜单",
        darkModeSwitchLabel: "主题",
        lightModeSwitchTitle: "切换到浅色模式",
        darkModeSwitchTitle: "切换到深色模式",
    },
});

function nav(): DefaultTheme.NavItem[] {
    return [
        {
            text: "指南",
            link: "/zh/guide/intro/",
            activeMatch: "/zh/guide/",
        },
        {
            text: "开发",
            items: [
                {
                    text: "安装" + " ",
                    link: "https://marketplace.visualstudio.com/items?itemName=nicepkg.aide-pro",
                },
                {
                    text: "更新日志",
                    link: "https://github.com/nicepkg/aide/blob/master/CHANGELOG.md",
                },
                {
                    text: "贡献指南",
                    link: "https://github.com/nicepkg/aide/blob/master/CONTRIBUTING.md",
                },
            ],
        },
    ];
}

// 🌱 🚀
function sidebar(): DefaultTheme.Sidebar {
    return {
        "/zh/guide/": [
            {
                text: "简介",
                collapsed: false,
                base: "/zh/guide/intro",
                items: [
                    { text: "简介", link: "/" },
                    {
                        text: "快速开始",
                        link: "/getting-started",
                    },
                    {
                        text: "自定义快捷键",
                        link: "/customize-shortcuts",
                    },
                    {
                        text: "自定义配置",
                        link: "/customize-configuration",
                    },
                    { text: "常见问题解答", link: "/faq" },
                ],
            },
        ],
    };
}

export const search: DefaultTheme.AlgoliaSearchOptions["locales"] = {
    zh: {
        placeholder: "搜索文档",
        translations: {
            button: {
                buttonText: "搜索文档",
                buttonAriaLabel: "搜索文档",
            },
            modal: {
                searchBox: {
                    resetButtonTitle: "清除查询条件",
                    resetButtonAriaLabel: "清除查询条件",
                    cancelButtonText: "取消",
                    cancelButtonAriaLabel: "取消",
                },
                startScreen: {
                    recentSearchesTitle: "搜索历史",
                    noRecentSearchesText: "没有搜索历史",
                    saveRecentSearchButtonTitle: "保存至搜索历史",
                    removeRecentSearchButtonTitle: "从搜索历史中移除",
                    favoriteSearchesTitle: "收藏",
                    removeFavoriteSearchButtonTitle: "从收藏中移除",
                },
                errorScreen: {
                    titleText: "无法获取结果",
                    helpText: "你可能需要检查你的网络连接",
                },
                footer: {
                    selectText: "选择",
                    navigateText: "切换",
                    closeText: "关闭",
                    searchByText: "搜索提供者",
                },
                noResultsScreen: {
                    noResultsText: "无法找到相关结果",
                    suggestedQueryText: "你可以尝试查询",
                    reportMissingResultsText: "你认为该查询应该有结果？",
                    reportMissingResultsLinkText: "点击反馈",
                },
            },
        },
    },
};

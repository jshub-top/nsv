import { defineConfigWithTheme, defineConfig, DefaultTheme } from 'vitepress'
import type { Config as ThemeConfig } from '@vue/theme'
import baseConfig from '@vue/theme/config'




const nav: DefaultTheme.Config["nav"] = [
    {
        text: '指导',
        activeMatch: `^/(guide)/`,
        items: [
            {
                text: "快速开始",
                link: "/guide/start",
            },
            {
                text: "cli 命令",
                link: "/guide/cli/use",
            }
        ]
    },
    {
        text: "version-0.0.1",
        activeMatch: `^/(version)/`,
        link: "/version/",
    }
]

const sidebar: DefaultTheme.Config["sidebar"] = [
    {
        text: '介绍',
        collapsed: true,
        items: [
            { text: '为什么选择nsv', link: '/guide/what-is-nsv' },
            { text: '简介', link: '/guide/intro' },
            { text: '开始', link: '/guide/start' }
        ]
    },
    {
        text: "cli",
        collapsed: true,
        items: [
            { text: 'use', link: '/guide/cli/use' },
            { text: 'local', link: '/guide/cli/local' },
            { text: 'discern', link: '/guide/cli/discern' }
        ]
    }
]

// https://vitepress.dev/reference/site-config
export default defineConfigWithTheme<DefaultTheme.Config>({
    extends: baseConfig,
    lang: "zh-CN",
    title: " ",
    titleTemplate: "nsv",
    description: "nsv.js -- 一个node版本管理工具",
    srcDir: "src",
    lastUpdated: true,
    cleanUrls: true,



    head: [
        [
            "link",
            { rel: "icon", href: "/logo1.svg" }
        ]
    ],



    themeConfig: {
        nav,
        sidebar,
        outline: "deep",
        logo: "/logo.svg",
        lastUpdated: {
            text: 'Updated at',
            formatOptions: {
                dateStyle: 'full',
                timeStyle: 'medium'
            }
        },


        editLink: {
            pattern: 'https://github.com/1739616529/nsv/tree/main/docs/:path',
            text: '在Github上修改此页'
        },

        socialLinks: [
            { icon: 'github', link: 'https://github.com/1739616529/nsv' }
        ],


        search: {
            provider: 'local',
            options: {
                translations: {
                    button: {
                        buttonText: '搜索'
                    },
                    modal: {
                        footer: {
                            selectText: '选择',
                            navigateText: '切换',
                            closeText: '关闭',
                        },
                    }
                },
            }
        }
    },
    vite: {
        define: {
            __VUE_OPTIONS_API__: false
        },
        optimizeDeps: {
            include: ['gsap', 'dynamics.js'],
            exclude: ['@vue/repl']
        },
        // @ts-ignore
        ssr: {
            external: ['@vue/repl']
        },
        server: {
            host: true,
            port: 7530,
            fs: {
                // for when developing with locally linked theme
                allow: ['../..']
            }
        },
        build: {
            minify: 'terser',
            chunkSizeWarningLimit: Infinity
        },
        json: {
            stringify: true
        },
        css: {
            devSourcemap: true
        }
    }
})

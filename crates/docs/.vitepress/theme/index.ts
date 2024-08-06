import { NolebaseInlineLinkPreviewPlugin } from "@nolebase/vitepress-plugin-inline-link-preview/client";
import DefaultTheme from "vitepress/theme";
import type { App } from "vue";

import "@nolebase/vitepress-plugin-inline-link-preview/client/style.css";

import "./style.css";

export default {
    extends: DefaultTheme,
    enhanceApp({ app }: { app: App }) {
        app.use(NolebaseInlineLinkPreviewPlugin);
    },
};

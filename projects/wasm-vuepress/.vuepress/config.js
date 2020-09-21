const locale_cn = {
    selectText: '选择语言',
    label: '简体中文',
    editLinkText: '在 GitHub 上编辑此页',
    serviceWorker: {
        updatePopup: {
            message: "发现新内容可用.",
            buttonText: "刷新"
        }
    },
    sidebar: {
        "/appendix/": [
            {
                title: '附录',
                collapsable: true,
                children: [
                    ['', '附录A'],
                    ['other', '附录B'],
                ]
            },
        ]
    }
}

module.exports = {
    dest: '.build',
    head: [
        ['link', { rel: 'shortcut icon', type: "image/x-icon", href: './favicon.png' }]
    ],
    themeConfig: {
        repo: 'GalAster/crypto-moe',
        editLinks: true,
        docsDir: 'projects/wasm-vuepress',
        markdown: {
            lineNumbers: true
        },
        locales: {
            '/': locale_cn,
        },
    },
    markdown: {
        config: md => {
        }
    },
    plugins: {
        mathjax: {
            target: 'chtml',
            presets: [],
        },
        '@vuepress/pwa': {
            // serviceWorker: true,
            // updatePopup: true,
            popupComponent: 'PWAUpdate',
        }
    }
};

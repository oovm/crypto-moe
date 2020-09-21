const sidebar = [
    {
        title: 'Trivial',
        path: '/trivial/',
        collapsable: false,
        children: [
            ['/trivial/caesar', '凯撒密码'],
            ['/trivial/morse', '莫斯密码'],
        ]
    },
    {
        title: 'Bizarre',
        path: '/bizarre/',
        collapsable: false,
        children: [
            ['/bizarre/brainfuck.md', 'Brainfuck']
            ['/bizarre/marysue.md', 'Marysue'],
        ]
    },
]



module.exports = {
    dest: 'docs/.build',
    head: [
        ['link', { rel: 'shortcut icon', type: "image/x-icon", href: './favicon.png' }],
        ['script',
            {
                type: '"text/javascript" async',
                src: 'https://unpkg.com/wasm-crypto-moe/wasm_crypto_moe.js'
            }
        ],
    ],
    themeConfig: {
        repo: 'GalAster/crypto-moe',
        editLinks: true,
        docsDir: 'projects/wasm-vuepress/docs',
        selectText: '选择语言',
        label: '简体中文',
        editLinkText: '在 GitHub 上编辑此页',
        markdown: {
            lineNumbers: true
        },
        sidebar: sidebar
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
            serviceWorker: true,
            updatePopup: true,
            popupComponent: 'PWAUpdate',
            generateSWConfig: {
                importWorkboxFrom: 'local'
            }
        }
    }
};

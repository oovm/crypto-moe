```html
<script type="module">
    import init, { marysue_encode, marysue_decode } from '/assets/js/wasm-crypto-moe'
    async function run() {
        await init("https://unpkg.com/wasm-crypto-moe/wasm_crypto_moe_bg.wasm");
        window.marysue_encode = marysue_encode;
        window.marysue_decode = marysue_decode;
    }
    run();
</script>
```


这是一个示例界面

<textarea placeholder="输入待编解码内容，支持中文" name="text" id="content" class="layui-textarea" style="height:250px;"></textarea>

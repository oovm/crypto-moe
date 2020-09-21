<script type="module">
    import init, { result, result_tex } from "https://cdn.jsdelivr.net/npm/sm_playground@0.0.5/sm_playground.js"
    async function run() {
        await init();
        window.forward = result;
        window.forward_tex = result_tex;
    }
    run();
</script>



这是一个示例界面

<div class="layui-row">
    <form class="layui-form" action="">
        <div class="layui-form-item layui-form-text">
            <pre style="margin-bottom:-20px;">
    <textarea placeholder="输入待编解码内容，支持中文" name="text" id="content" class="layui-textarea" style="height:250px;">
    </textarea>
    </pre>
        </div>
        <div class="layui-form-item" style="text-align:center;margin-bottom:-5px;">
            <div class="layui-inline">
                <div class="layui-input-block">
                    <button class="layui-btn" lay-submit="" id="aa_jiami">加&nbsp;&nbsp;密</button>
                </div>
            </div>
            <div class="layui-inline">
                <div class="layui-input-block">
                    <button class="layui-btn" lay-submit="" id="aa_jiemi">解&nbsp;&nbsp;密</button>
                </div>
            </div>
        </div>
    </form>
</div>
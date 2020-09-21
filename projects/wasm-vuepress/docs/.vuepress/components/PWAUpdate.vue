<template>
    <SWUpdatePopup v-slot="{ enabled, reload, message, buttonText }">
        <div v-if="enabled" class="pwa-update-popup">
            {{ message }}
            <br />
            <button @click="reload">{{ buttonText }}</button>
        </div>
    </SWUpdatePopup>
</template>

<script>
import SWUpdatePopup from "@vuepress/plugin-pwa/lib/SWUpdatePopup.vue";

export default {
    components: { SWUpdatePopup }
};
</script>

<style>
.pwa-update-popup {
    text-align: right;
    position: fixed;
    bottom: 20px;
    right: 20px;
    background-color: #fff;
    font-size: 20px;
    padding: 10px;
    border: 5px solid #3eaf7c;
}

.pwa-update-popup button {
    border: 1px solid #fefefe;
}
</style>

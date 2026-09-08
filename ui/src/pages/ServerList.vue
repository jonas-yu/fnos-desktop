<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const name = ref("");
const url = ref("");
const error = ref("");
const busy = ref(false);
const opened = ref<string | null>(null);

async function openNas() {
  error.value = "";
  opened.value = null;
  if (!url.value.trim()) {
    error.value = "请填写 NAS 地址";
    return;
  }
  busy.value = true;
  try {
    const label = await invoke<string>("open_nas", { url: url.value.trim() });
    opened.value = label;
  } catch (e) {
    error.value = String(e);
  } finally {
    busy.value = false;
  }
}
</script>

<template>
  <div class="page">
    <header class="hero">
      <div class="logo">飞牛桌面客户端</div>
      <p class="sub">自研 · 可审计 · 登录态隔离</p>
    </header>

    <!-- M1 空态引导 + 添加表单雏形 -->
    <section class="card add-form">
      <h2>添加 NAS</h2>
      <label class="field">
        <span>名称（可选）</span>
        <input v-model="name" placeholder="例如：家里 NAS" />
      </label>
      <label class="field">
        <span>地址</span>
        <input
          v-model="url"
          placeholder="https://192.168.1.16:5667 或 https://域名"
          @keyup.enter="openNas"
        />
      </label>
      <p v-if="error" class="error">{{ error }}</p>
      <p v-if="opened" class="ok">已打开（窗口标签 {{ opened }}）</p>
      <button class="primary" :disabled="busy" @click="openNas">
        {{ busy ? "打开中…" : "打开" }}
      </button>
      <p class="hint">
        地址支持直接粘贴；多台 NAS 的登录态各自独立保存（WebView 分区隔离）。
      </p>
    </section>
  </div>
</template>

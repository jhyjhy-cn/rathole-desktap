<script setup lang="ts">
import PageHeader from '../components/PageHeader.vue'
</script>

<template>
  <div class="about">
    <PageHeader>
      <template #header></template>
    </PageHeader>

    <div class="about-content">
    <img src="/vite.svg" class="logo" />
    <h1>Rathole Desktop</h1>
    <p>{{ $t('common.version') }} 0.0.1</p>
    <p>{{ $t('about.description') }}</p>
    
    <div class="links">
        <a href="https://github.com/rapiz1/rathole" target="_blank">{{ $t('about.core') }}</a>
        <a href="https://github.com/luckjiawei/frpc-desktop" target="_blank">{{ $t('about.inspired') }}</a>
    </div>
    </div>
  </div>
</template>

<style scoped>
.about {
    height: 100%;
    display: flex;
    flex-direction: column;
}

.about-content {
    flex: 1;
    text-align: center;
    padding: 20px;
    display: flex;
    flex-direction: column;
    align-items: center;
}

.logo {
    width: 100px;
    height: 100px;
    margin-bottom: 20px;
}
.links {
    margin-top: 30px;
    display: flex;
    gap: 20px;
    justify-content: center;
}
a {
    color: var(--el-color-primary);
    text-decoration: none;
}
</style>

<template>
  <div class="about">
    <img src="/vite.svg" class="logo" />
    <h1>Rathole Desktop</h1>
    <p>{{ $t('common.version') }} 0.0.1</p>
    <p>{{ $t('about.description') }}</p>
    
    <div class="links">
        <a href="https://github.com/rapiz1/rathole" target="_blank">{{ $t('about.core') }}</a>
        <a href="https://github.com/luckjiawei/frpc-desktop" target="_blank">{{ $t('about.inspired') }}</a>
    </div>
  </div>
</template>

<style scoped>
.about {
    text-align: center;
    padding-top: 50px;
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

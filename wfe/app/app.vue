<script setup>
useHead({
  meta: [
    { name: 'viewport', content: 'width=device-width, initial-scale=1' }
  ],
  link: [
    { rel: 'icon', href: '/favicon.ico' }
  ],
  htmlAttrs: {
    lang: 'en'
  }
})

const title = 'Rock Paper Scissors - Rusty Nuxt'
const description = 'A retro-styled Rock Paper Scissors game with WebSocket real-time gameplay.'

useSeoMeta({
  title,
  description,
  ogTitle: title,
  ogDescription: description,
})
</script>

<template>
  <UApp>
    <div class="retro-background">
      <UMain>
        <NuxtPage :transition="{
          name: 'page',
          mode: 'out-in'
        }" />
      </UMain>
    </div>
  </UApp>
</template>

<style scoped>
.retro-background {
  min-height: 100vh;
  background: 
    linear-gradient(0deg, transparent 24%, rgba(255, 0, 255, 0.05) 25%, rgba(255, 0, 255, 0.05) 26%, transparent 27%, transparent 74%, rgba(255, 0, 255, 0.05) 75%, rgba(255, 0, 255, 0.05) 76%, transparent 77%, transparent),
    linear-gradient(90deg, transparent 24%, rgba(0, 255, 255, 0.05) 25%, rgba(0, 255, 255, 0.05) 26%, transparent 27%, transparent 74%, rgba(0, 255, 255, 0.05) 75%, rgba(0, 255, 255, 0.05) 76%, transparent 77%, transparent);
  background-size: 50px 50px;
  background-color: #0a0a0a;
  position: relative;
}

.retro-background::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: 
    repeating-linear-gradient(
      0deg,
      rgba(0, 0, 0, 0.15),
      rgba(0, 0, 0, 0.15) 1px,
      transparent 1px,
      transparent 2px
    );
  pointer-events: none;
  animation: scanlines 8s linear infinite;
}

.retro-background::after {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: radial-gradient(ellipse at center, transparent 0%, rgba(0, 0, 0, 0.5) 100%);
  pointer-events: none;
}

@keyframes scanlines {
  0% {
    transform: translateY(0);
  }
  100% {
    transform: translateY(2px);
  }
}
</style>

<style>
/* Page transitions */
.page-enter-active,
.page-leave-active {
  transition: all 0.5s cubic-bezier(0.4, 0, 0.2, 1);
}

.page-enter-from {
  opacity: 0;
  transform: scale(0.95) translateY(20px);
}

.page-leave-to {
  opacity: 0;
  transform: scale(1.05) translateY(-20px);
}

.page-enter-to,
.page-leave-from {
  opacity: 1;
  transform: scale(1) translateY(0);
}
</style>


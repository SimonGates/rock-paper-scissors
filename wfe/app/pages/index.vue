<template>
  <div class="min-h-screen flex items-center justify-center">
    <div class="text-center space-y-12">
      <!-- Title -->
      <div class="space-y-4">
        <h1 class="text-7xl font-display font-bold text-green-400 tracking-wider animate-pulse-slow">
          ROCK PAPER SCISSORS
        </h1>
        <p class="text-xl text-zinc-500 font-mono tracking-widest">
          <span class="text-orange-600">RUSTY</span> <span class="text-green-600">NUXT</span> EDITION
        </p>
      </div>

      <!-- Play Button -->
      <div>
        <NuxtLink to="/game">
          <button
            class="group relative px-16 py-8 text-3xl font-bold font-mono tracking-widest bg-zinc-900 border-4 border-green-500 text-green-400 shadow-2xl shadow-green-500/50 transition-all duration-300 hover:scale-110 hover:shadow-green-500/80 hover:bg-green-500 hover:text-zinc-900 active:scale-95">
            <span class="relative z-10">PLAY GAME</span>
            <div
              class="absolute inset-0 border-4 border-green-400 translate-x-2 translate-y-2 -z-10 group-hover:translate-x-3 group-hover:translate-y-3 transition-transform duration-300">
            </div>
          </button>
        </NuxtLink>
      </div>

      <!-- Instructions -->
      <div class="text-zinc-600 text-sm font-mono space-y-2">
        <p>Click to start your journey</p>
        <p class="text-xs">Beat the computer and rack up points!</p>
      </div>
    </div>
  </div>
</template>

<style scoped>
@keyframes pulse-slow {

  0%,
  100% {
    opacity: 1;
  }

  50% {
    opacity: 0.8;
  }
}

.animate-pulse-slow {
  animation: pulse-slow 3s ease-in-out infinite;
}
</style>

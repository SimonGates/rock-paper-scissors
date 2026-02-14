<template>
  <div>
    <UPageHero title="Rock Paper Scissors" description="Rusty Nuxt"
      :ui="{ header: 'font-display', container: 'mb-0 lg:pb-0 ' }" />

    <UPageSection class="mt-0">
      <div class="flex items-center justify-center text-green-400">
        <div class="grid md:grid-cols-3 gap-8 text-xs md:text-sm font-mono">

          <!-- Rock -->
          <div class="shadow-2xl shadow-black/50 p-6 border-4 border-dashed border-zinc-700 border-5px">
            <h2 class="text-center mb-4 text-zinc-300 tracking-widest">ROCK</h2>
            <pre class="leading-tight text-green-400">
              <Rock />
            </pre>
          </div>

          <!-- Paper -->
          <div class="shadow-2xl shadow-black/50 p-6 border-4 border-dashed border-zinc-700 border-5px">
            <h2 class="text-center mb-4 text-zinc-300 tracking-widest">PAPER</h2>
            <pre class="leading-tight text-blue-400">
              <Paper />
            </pre>
          </div>

          <!-- Scissors -->
          <div class="shadow-2xl shadow-black/50 p-6 border-4 border-dashed border-zinc-700 border-5px">
            <h2 class="text-center mb-4 text-zinc-300 tracking-widest">SCISSORS</h2>
            <pre class="leading-tight text-pink-400">
              <Scissors />
            </pre>
          </div>

        </div>
      </div>
    </UPageSection>
  </div>
</template>

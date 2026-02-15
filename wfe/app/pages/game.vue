<template>
  <div>
    <UPageHero title="Rock Paper Scissors" description="Rusty Nuxt"
      :ui="{ header: 'font-display', container: 'mb-0 lg:pb-0 ' }" />

    <!-- Audio Controls -->
    <div class="flex justify-center gap-4 -mt-4 mb-8">
      <button @click="toggleMusic"
        class="p-3 bg-zinc-900 border-2 border-zinc-700 shadow-lg hover:border-zinc-500 transition-all duration-200"
        :class="{ 'border-green-500': isMusicEnabled, 'border-red-500': !isMusicEnabled }">
        <Icon :name="isMusicEnabled ? 'heroicons:musical-note' : 'heroicons:musical-note-slash'" class="w-6 h-6"
          :class="isMusicEnabled ? 'text-green-400' : 'text-red-400'" />
      </button>

      <button @click="toggleSfx"
        class="p-3 bg-zinc-900 border-2 border-zinc-700 shadow-lg hover:border-zinc-500 transition-all duration-200"
        :class="{ 'border-green-500': isSfxEnabled, 'border-red-500': !isSfxEnabled }">
        <Icon :name="isSfxEnabled ? 'heroicons:speaker-wave' : 'heroicons:speaker-x-mark'" class="w-6 h-6"
          :class="isSfxEnabled ? 'text-green-400' : 'text-red-400'" />
      </button>
    </div>

    <UPageSection class="mt-0">
      <!-- Info Row: Rules, Score, Countdown -->
      <div class="grid md:grid-cols-3 gap-6 mb-8">
        <!-- Rules -->
        <div class="px-6 py-4 bg-zinc-900 border-4 border-zinc-700 shadow-2xl shadow-black/50">
          <div class="text-zinc-500 text-xs font-mono space-y-2">
            <div class="text-zinc-400 text-sm tracking-widest mb-3">RULES</div>
            <div>Win: +3 points</div>
            <div>Lose: -1 point</div>
            <div>Draw: No change</div>
            <div class="pt-2 text-zinc-600">Rock beats Scissors</div>
            <div class="text-zinc-600">Paper beats Rock</div>
            <div class="text-zinc-600">Scissors beats Paper</div>
          </div>
        </div>

        <!-- Score Display -->
        <div class="text-center flex px-8 py-4 bg-zinc-900 border-4 border-zinc-700 shadow-2xl shadow-black/50">
          <div class="m-auto">
            <div class="text-zinc-400 text-sm tracking-widest mb-2">SCORE</div>
            <div class="text-5xl font-bold font-mono" :class="{
              'text-green-400': score > 0,
              'text-red-400': score < 0,
              'text-zinc-300': score === 0
            }">
              <span v-if="!showCountdown">
                {{ score }}
              </span>
              <span v-else>
                ---
              </span>
            </div>
          </div>
        </div>

        <!-- Countdown/Result Display -->
        <div class="text-center flex px-12 py-4 bg-zinc-900 border-4 border-dashed shadow-2xl shadow-black/50" :class="{
          'border-zinc-700': !showCountdown && !lastResult,
          'border-yellow-500': showCountdown,
          'border-green-500': lastResult === 'Win',
          'border-red-500': lastResult === 'Lose',
          'border-blue-500': lastResult === 'Draw'
        }">
          <div class="m-auto">
            <div class="text-zinc-400 text-sm tracking-widest mb-2">RESULT</div>
            <div v-if="showCountdown" class="text-6xl font-bold font-mono text-yellow-400 animate-pulse">
              {{ countdown }}
            </div>
            <div v-else-if="lastResult" class="text-5xl font-bold font-mono tracking-widest" :class="{
              'text-green-400': lastResult === 'Win',
              'text-red-400': lastResult === 'Lose',
              'text-blue-400': lastResult === 'Draw'
            }">
              {{ lastResult.toUpperCase() }}
            </div>
            <div v-else class="text-4xl font-bold font-mono text-zinc-700">
              ---
            </div>
          </div>
        </div>
      </div>

      <!-- Error Display -->
      <div v-if="error" class="text-center mb-8">
        <div class="inline-block px-8 py-4 bg-red-900/30 border-4 border-red-700 shadow-2xl shadow-black/50">
          <div class="text-red-400 text-sm font-mono">{{ error }}</div>
        </div>
      </div>

      <div class="flex items-center justify-center text-green-400">
        <div class="grid md:grid-cols-3 w-full gap-8 text-xs md:text-sm font-mono">

          <!-- Rock -->
          <button @click="handleChoice('Rock')" :disabled="isWaitingForResult || inFlight"
            class="shadow-2xl shadow-black/50 p-6  border-4 border-dashed cursor-pointer border-zinc-700 border-5px transition-all duration-200 hover:scale-105 hover:border-green-500 disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:scale-100">
            <h2 class="text-center mb-4 text-zinc-300 tracking-widest">ROCK</h2>
            <pre class="leading-tight text-green-400">
              <Rock />
            </pre>
          </button>

          <!-- Paper -->
          <button @click="handleChoice('Paper')" :disabled="isWaitingForResult || inFlight"
            class="shadow-2xl shadow-black/50 p-6 border-4 border-dashed cursor-pointer border-zinc-700 border-5px transition-all duration-200 hover:scale-105 hover:border-blue-500 disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:scale-100">
            <h2 class="text-center mb-4 text-zinc-300 tracking-widest">PAPER</h2>
            <pre class="leading-tight text-blue-400">
              <Paper />
            </pre>
          </button>

          <!-- Scissors -->
          <button @click="handleChoice('Scissors')" :disabled="isWaitingForResult || inFlight"
            class="shadow-2xl shadow-black/50 p-6 border-4 border-dashed cursor-pointer border-zinc-700 border-5px transition-all duration-200 hover:scale-105 hover:border-pink-500 disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:scale-100">
            <h2 class="text-center mb-4 text-zinc-300 tracking-widest">SCISSORS</h2>
            <pre class="leading-tight text-pink-400">
              <Scissors />
            </pre>
          </button>

        </div>
      </div>
    </UPageSection>
  </div>
</template>

<script setup lang="ts">
type Choice = 'Rock' | 'Paper' | 'Scissors'

const { play, score, lastResult, isWaitingForResult, error } = useRockPaperScissors()
const { isMusicEnabled, isSfxEnabled, toggleMusic, toggleSfx, playChoice, playWin, playLose, playDraw, initAudio } = useGameAudio()

const countdown = ref<number | null>(null)
const showCountdown = ref(false)
const inFlight = ref(false)

// Initialize audio when game page loads
onMounted(() => {
  initAudio()
})

const handleChoice = async (choice: Choice) => {
  // Play choice sound
  playChoice()

  // Start countdown
  inFlight.value = true;
  showCountdown.value = true
  countdown.value = 3

  // Send the choice to the server
  await play(choice)

  // Run countdown
  for (let i = 3; i > 0; i--) {
    countdown.value = i
    await new Promise(resolve => setTimeout(resolve, 1000))
  }

  showCountdown.value = false
  if (lastResult.value === 'Win') {
    playWin()
  }
  else if (lastResult.value === 'Draw') {
    playDraw()
  }
  else if (lastResult.value === 'Lose') {
    playLose()
  }
  // Show result for 2 seconds before clearing
  setTimeout(() => {
    lastResult.value = null
    inFlight.value = false
  }, 2000)
}
</script>

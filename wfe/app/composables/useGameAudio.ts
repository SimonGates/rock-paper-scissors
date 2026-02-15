import { ref, onMounted, onUnmounted } from 'vue'

export const useGameAudio = () => {
  const isMusicEnabled = ref(true)
  const isSfxEnabled = ref(true)
  
  const bgMusic = ref<HTMLAudioElement | null>(null)
  const choiceSound = ref<HTMLAudioElement | null>(null)
  const winSound = ref<HTMLAudioElement | null>(null)
  const loseSound = ref<HTMLAudioElement | null>(null)
  const isInitialized = ref(false)

  const initAudio = () => {
    if (isInitialized.value) return

    // Initialize audio elements
    bgMusic.value = new Audio('/assets/sounds/bg-loop.mp3')
    bgMusic.value.loop = true
    bgMusic.value.volume = 0.3

    choiceSound.value = new Audio('/assets/sounds/choice.mp3')
    choiceSound.value.volume = 0.5

    winSound.value = new Audio('/assets/sounds/winner.mp3')
    winSound.value.volume = 0.6

    loseSound.value = new Audio('/assets/sounds/lose.mp3')
    loseSound.value.volume = 0.6

    isInitialized.value = true

    // Start background music if enabled
    if (isMusicEnabled.value) {
      playBackgroundMusic()
    }
  }

  onUnmounted(() => {
    // Clean up audio elements
    if (bgMusic.value) {
      bgMusic.value.pause()
      bgMusic.value = null
    }
    choiceSound.value = null
    winSound.value = null
    loseSound.value = null
    isInitialized.value = false
  })

  const playBackgroundMusic = () => {
    if (bgMusic.value && isMusicEnabled.value) {
      bgMusic.value.play().catch(err => {
        console.log('Background music autoplay prevented:', err)
      })
    }
  }

  const toggleMusic = () => {
    isMusicEnabled.value = !isMusicEnabled.value
    
    if (bgMusic.value) {
      if (isMusicEnabled.value) {
        bgMusic.value.play().catch(err => {
          console.log('Background music play failed:', err)
        })
      } else {
        bgMusic.value.pause()
      }
    }
  }

  const toggleSfx = () => {
    isSfxEnabled.value = !isSfxEnabled.value
  }

  const playChoice = () => {
    if (isSfxEnabled.value && choiceSound.value) {
      choiceSound.value.currentTime = 0
      choiceSound.value.play().catch(err => {
        console.log('Choice sound play failed:', err)
      })
    }
  }

  const playWin = () => {
    if (isSfxEnabled.value && winSound.value) {
      winSound.value.currentTime = 0
      winSound.value.play().catch(err => {
        console.log('Win sound play failed:', err)
      })
    }
  }

  const playLose = () => {
    if (isSfxEnabled.value && loseSound.value) {
      loseSound.value.currentTime = 0
      loseSound.value.play().catch(err => {
        console.log('Lose sound play failed:', err)
      })
    }
  }

  return {
    isMusicEnabled,
    isSfxEnabled,
    toggleMusic,
    toggleSfx,
    playChoice,
    playWin,
    playLose,
    initAudio
  }
}


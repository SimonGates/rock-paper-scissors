import { ref, onUnmounted } from 'vue'

type Choice = 'Rock' | 'Paper' | 'Scissors'
type TurnResult = 'Win' | 'Lose' | 'Draw'

interface GameResponse {
  Result?: {
    turn_result: TurnResult
    game: {
      score: number
    }
  }
  Error?: string
}

export const useRockPaperScissors = () => {
  const ws = ref<WebSocket | null>(null)
  const isConnected = ref(false)
  const score = ref(0)
  const lastResult = ref<TurnResult | null>(null)
  const isWaitingForResult = ref(false)
  const error = ref<string | null>(null)

  const connect = () => {
    return new Promise<void>((resolve, reject) => {
      if (ws.value && isConnected.value) {
        resolve()
        return
      }

      try {
        ws.value = new WebSocket('ws://localhost:6767')

        ws.value.onopen = () => {
          console.log('Connected to game server')
          isConnected.value = true
          resolve()
        }

        ws.value.onmessage = (event) => {
          console.log('Received:', event.data)
          
          // Skip the welcome message
          if (event.data === 'Hi!') {
            return
          }

          try {
            const data: GameResponse = JSON.parse(event.data)
            
            if (data.Result) {
              lastResult.value = data.Result.turn_result
              score.value = data.Result.game.score
              isWaitingForResult.value = false
            } else if (data.Error) {
              error.value = data.Error
              isWaitingForResult.value = false
            }
          } catch (e) {
            console.error('Failed to parse message:', e)
          }
        }

        ws.value.onerror = (err) => {
          console.error('WebSocket error:', err)
          error.value = 'Connection error'
          reject(err)
        }

        ws.value.onclose = () => {
          console.log('Disconnected from server')
          isConnected.value = false
        }
      } catch (err) {
        console.error('Failed to create WebSocket:', err)
        error.value = 'Failed to connect'
        reject(err)
      }
    })
  }

  const play = async (choice: Choice) => {
    if (!isConnected.value) {
      await connect()
    }

    if (!ws.value || !isConnected.value) {
      error.value = 'Not connected to server'
      return
    }

    isWaitingForResult.value = true
    lastResult.value = null
    error.value = null

    const payload = { Payload: choice }
    ws.value.send(JSON.stringify(payload))
  }

  const disconnect = () => {
    if (ws.value) {
      ws.value.close()
      ws.value = null
      isConnected.value = false
    }
  }

  // Cleanup on unmount
  onUnmounted(() => {
    disconnect()
  })

  return {
    isConnected,
    score,
    lastResult,
    isWaitingForResult,
    error,
    play,
    disconnect
  }
}

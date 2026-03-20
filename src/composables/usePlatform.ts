import { ref, computed } from 'vue'

const userAgent = navigator.userAgent || ''
const isMobileDevice = ref(
  /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(userAgent),
)

export function usePlatform() {
  const isMobile = computed(() => isMobileDevice.value)
  const isDesktop = computed(() => !isMobileDevice.value)
  const isAndroid = computed(() => /Android/i.test(userAgent))
  const isIOS = computed(() => /iPhone|iPad|iPod/i.test(userAgent))

  return { isMobile, isDesktop, isAndroid, isIOS }
}

import { ref, watch, type Ref } from 'vue'

/**
 * 防抖函数
 * @param fn 要防抖的函数
 * @param delay 延迟时间（毫秒）
 */
export function debounce<T extends (...args: Parameters<T>) => ReturnType<T>>(
  fn: T,
  delay: number,
): (...args: Parameters<T>) => void {
  let timeoutId: ReturnType<typeof setTimeout> | null = null

  return function (this: ThisParameterType<T>, ...args: Parameters<T>) {
    if (timeoutId) {
      clearTimeout(timeoutId)
    }

    timeoutId = setTimeout(() => {
      fn.apply(this, args)
      timeoutId = null
    }, delay)
  }
}

/**
 * 可取消的防抖函数
 */
export function debounceCancelable<T extends (...args: Parameters<T>) => ReturnType<T>>(
  fn: T,
  delay: number,
): {
  call: (...args: Parameters<T>) => void
  cancel: () => void
} {
  let timeoutId: ReturnType<typeof setTimeout> | null = null

  return {
    call: function (...args: Parameters<T>) {
      if (timeoutId) {
        clearTimeout(timeoutId)
      }

      timeoutId = setTimeout(() => {
        fn(...args)
        timeoutId = null
      }, delay)
    },
    cancel: function () {
      if (timeoutId) {
        clearTimeout(timeoutId)
        timeoutId = null
      }
    },
  }
}

/**
 * 响应式防抖 - 用于watch
 * @param source 要监听的响应式数据
 * @param callback 回调函数
 * @param delay 延迟时间（毫秒）
 */
export function useDebouncedWatch<T>(
  source: Ref<T>,
  callback: (value: T, oldValue: T) => void,
  delay: number,
) {
  const debouncedCallback = debounce(callback, delay)

  return watch(source, debouncedCallback)
}

/**
 * 防抖的响应式值
 * @param source 源响应式数据
 * @param delay 延迟时间（毫秒）
 */
export function useDebouncedRef<T>(source: Ref<T>, delay: number): Ref<T> {
  const debouncedValue = ref(source.value) as Ref<T>
  let timeoutId: ReturnType<typeof setTimeout> | null = null

  watch(source, (newValue) => {
    if (timeoutId) {
      clearTimeout(timeoutId)
    }

    timeoutId = setTimeout(() => {
      debouncedValue.value = newValue
      timeoutId = null
    }, delay)
  })

  return debouncedValue
}

/**
 * 节流函数 - 在指定时间内只执行一次
 * @param fn 要节流的函数
 * @param delay 延迟时间（毫秒）
 */
export function throttle<T extends (...args: Parameters<T>) => ReturnType<T>>(
  fn: T,
  delay: number,
): (...args: Parameters<T>) => void {
  let lastTime = 0
  let timeoutId: ReturnType<typeof setTimeout> | null = null

  return function (this: ThisParameterType<T>, ...args: Parameters<T>) {
    const now = Date.now()

    if (now - lastTime >= delay) {
      lastTime = now
      fn.apply(this, args)
    } else {
      // 确保最后一次调用也会执行
      if (timeoutId) {
        clearTimeout(timeoutId)
      }
      timeoutId = setTimeout(
        () => {
          lastTime = Date.now()
          fn.apply(this, args)
          timeoutId = null
        },
        delay - (now - lastTime),
      )
    }
  }
}

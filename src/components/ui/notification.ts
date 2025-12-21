import { ref } from 'vue'

interface NotificationOptions {
  title?: string
  message: string
  type?: 'success' | 'error' | 'warning' | 'info'
  duration?: number
}

interface NotificationInstance {
  close: () => void
}

const notifications = ref<{ id: number; options: NotificationOptions }[]>([])
let notificationId = 0
let containerEl: HTMLDivElement | null = null

function ensureContainer() {
  if (!containerEl) {
    containerEl = document.createElement('div')
    containerEl.id = 'notification-container'
    containerEl.className = 'fixed top-4 right-4 z-50 flex flex-col gap-2'
    document.body.appendChild(containerEl)
  }
  return containerEl
}

function createNotificationElement(
  id: number,
  options: NotificationOptions,
): { el: HTMLDivElement; close: () => void } {
  const el = document.createElement('div')
  el.className = `
    flex items-center gap-3 px-4 py-3 rounded-lg shadow-lg
    bg-white border border-gray-200
    transform transition-all duration-300 ease-out
    translate-x-0 opacity-100
    min-w-[280px] max-w-[400px]
  `

  const iconClass =
    options.type === 'error'
      ? 'text-red-500'
      : options.type === 'warning'
        ? 'text-yellow-500'
        : options.type === 'info'
          ? 'text-blue-500'
          : 'text-green-500'

  const iconSvg =
    options.type === 'error'
      ? `<svg class="w-5 h-5 ${iconClass}" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>`
      : options.type === 'warning'
        ? `<svg class="w-5 h-5 ${iconClass}" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"></path></svg>`
        : `<svg class="w-5 h-5 ${iconClass}" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>`

  el.innerHTML = `
    <div class="flex-shrink-0">${iconSvg}</div>
    <div class="flex-1">
      ${options.title ? `<div class="font-medium text-gray-900">${options.title}</div>` : ''}
      <div class="text-sm text-gray-600">${options.message}</div>
    </div>
    <button class="flex-shrink-0 text-gray-400 hover:text-gray-600 transition-colors">
      <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
      </svg>
    </button>
  `

  const close = () => {
    el.style.transform = 'translateX(100%)'
    el.style.opacity = '0'
    setTimeout(() => {
      el.remove()
      notifications.value = notifications.value.filter((n) => n.id !== id)
    }, 300)
  }

  el.querySelector('button')?.addEventListener('click', close)

  return { el, close }
}

export function showNotification(options: NotificationOptions): NotificationInstance {
  const container = ensureContainer()
  const id = ++notificationId

  const { el, close } = createNotificationElement(id, options)
  container.appendChild(el)

  notifications.value.push({ id, options })

  // Auto close if duration is set and not 0
  if (options.duration !== 0) {
    const duration = options.duration ?? 3000
    setTimeout(close, duration)
  }

  return { close }
}

export const Notification = showNotification

import i18n from '../../i18n'

interface NotificationOptions {
  title?: string
  message: string
  detail?: string
  type?: 'success' | 'error' | 'warning' | 'info'
  duration?: number
}

interface NotificationInstance {
  close: () => void
}

let notificationId = 0
let containerEl: HTMLDivElement | null = null

function ensureContainer() {
  if (!containerEl) {
    containerEl = document.createElement('div')
    containerEl.id = 'notification-container'
    containerEl.className = 'fixed top-4 right-4 z-[9999] flex flex-col gap-2'
    containerEl.setAttribute('aria-live', 'polite')
    document.body.appendChild(containerEl)
  }
  return containerEl
}

function createSvgIcon(type: NotificationOptions['type']): SVGSVGElement {
  const svg = document.createElementNS('http://www.w3.org/2000/svg', 'svg')
  svg.setAttribute('class', 'w-5 h-5')
  svg.setAttribute('fill', 'none')
  svg.setAttribute('stroke', 'currentColor')
  svg.setAttribute('viewBox', '0 0 24 24')

  const colorClass =
    type === 'error'
      ? 'text-red-500'
      : type === 'warning'
        ? 'text-yellow-500'
        : type === 'info'
          ? 'text-blue-500'
          : 'text-green-500'
  svg.classList.add(...colorClass.split(' '))

  const path = document.createElementNS('http://www.w3.org/2000/svg', 'path')
  path.setAttribute('stroke-linecap', 'round')
  path.setAttribute('stroke-linejoin', 'round')
  path.setAttribute('stroke-width', '2')

  if (type === 'error') {
    path.setAttribute('d', 'M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z')
  } else if (type === 'warning') {
    path.setAttribute(
      'd',
      'M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z',
    )
  } else {
    path.setAttribute('d', 'M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z')
  }

  svg.appendChild(path)
  return svg
}

function createNotificationElement(
  _id: number,
  options: NotificationOptions,
): { el: HTMLDivElement; close: () => void } {
  const el = document.createElement('div')
  el.className = `
    flex ${options.detail ? 'items-start' : 'items-center'} gap-3 px-4 py-3 rounded-lg shadow-lg
    bg-surface border border-edge
    transform transition-all duration-300 ease-out
    translate-x-0 opacity-100
    min-w-[280px] max-w-[400px]
  `

  const iconContainer = document.createElement('div')
  iconContainer.className = 'flex-shrink-0'
  iconContainer.appendChild(createSvgIcon(options.type))
  el.appendChild(iconContainer)

  const contentContainer = document.createElement('div')
  contentContainer.className = 'flex-1'

  if (options.title) {
    const titleEl = document.createElement('div')
    titleEl.className = 'font-medium text-content'
    titleEl.textContent = options.title
    contentContainer.appendChild(titleEl)
  }

  const messageEl = document.createElement('div')
  messageEl.className = 'text-sm text-content-secondary'
  messageEl.textContent = options.message
  contentContainer.appendChild(messageEl)

  if (options.detail) {
    const detailToggle = document.createElement('button')
    detailToggle.className =
      'mt-1 text-xs text-content-tertiary hover:text-content-secondary transition-colors underline cursor-pointer'
    const showDetailText = i18n.global.t('notification.showDetail')
    const hideDetailText = i18n.global.t('notification.hideDetail')
    detailToggle.textContent = showDetailText

    const detailEl = document.createElement('div')
    detailEl.className =
      'mt-1 text-xs text-content-tertiary bg-surface-secondary rounded px-2 py-1.5 max-h-[120px] overflow-auto whitespace-pre-wrap break-all hidden'
    detailEl.textContent = options.detail

    detailToggle.addEventListener('click', () => {
      const isHidden = detailEl.classList.contains('hidden')
      detailEl.classList.toggle('hidden')
      detailToggle.textContent = isHidden ? hideDetailText : showDetailText
    })

    contentContainer.appendChild(detailToggle)
    contentContainer.appendChild(detailEl)
  }

  el.appendChild(contentContainer)

  const closeBtn = document.createElement('button')
  closeBtn.className = 'flex-shrink-0 text-gray-400 hover:text-gray-600 transition-colors'

  const closeSvg = document.createElementNS('http://www.w3.org/2000/svg', 'svg')
  closeSvg.setAttribute('class', 'w-4 h-4')
  closeSvg.setAttribute('fill', 'none')
  closeSvg.setAttribute('stroke', 'currentColor')
  closeSvg.setAttribute('viewBox', '0 0 24 24')

  const closePath = document.createElementNS('http://www.w3.org/2000/svg', 'path')
  closePath.setAttribute('stroke-linecap', 'round')
  closePath.setAttribute('stroke-linejoin', 'round')
  closePath.setAttribute('stroke-width', '2')
  closePath.setAttribute('d', 'M6 18L18 6M6 6l12 12')
  closeSvg.appendChild(closePath)
  closeBtn.appendChild(closeSvg)
  el.appendChild(closeBtn)

  let autoCloseTimer: ReturnType<typeof setTimeout> | null = null

  const close = () => {
    if (autoCloseTimer) {
      clearTimeout(autoCloseTimer)
      autoCloseTimer = null
    }
    el.style.transform = 'translateX(100%)'
    el.style.opacity = '0'
    setTimeout(() => {
      el.remove()
    }, 300)
  }

  // Auto close timer: error/warning stay until manually closed, others auto-close
  const isSticky = options.type === 'error' || options.type === 'warning'
  const defaultDuration = isSticky ? 0 : 3000
  const duration = options.duration ?? defaultDuration
  if (duration !== 0) {
    autoCloseTimer = setTimeout(close, duration)
  }

  closeBtn.addEventListener('click', close)

  return { el, close }
}

export function showNotification(options: NotificationOptions): NotificationInstance {
  const container = ensureContainer()
  const id = ++notificationId

  const { el, close } = createNotificationElement(id, options)
  container.appendChild(el)

  return { close }
}

export const Notification = showNotification

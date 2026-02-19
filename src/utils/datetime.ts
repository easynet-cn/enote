import { useI18n } from 'vue-i18n'

/**
 * 格式化相对时间（例如：3分钟前、2小时前等）
 */
export function formatRelativeTime(date: string | Date): string {
  const { t } = useI18n()
  const now = new Date()
  const targetDate = typeof date === 'string' ? new Date(date) : date
  const diffMs = now.getTime() - targetDate.getTime()
  const diffSeconds = Math.floor(diffMs / 1000)
  const diffMinutes = Math.floor(diffSeconds / 60)
  const diffHours = Math.floor(diffMinutes / 60)
  const diffDays = Math.floor(diffHours / 24)
  const diffWeeks = Math.floor(diffDays / 7)
  const diffMonths = Math.floor(diffDays / 30)
  const diffYears = Math.floor(diffDays / 365)

  if (diffSeconds < 60) {
    return t('datetime.justNow')
  } else if (diffMinutes < 60) {
    return t('datetime.minutesAgo', { n: diffMinutes })
  } else if (diffHours < 24) {
    return t('datetime.hoursAgo', { n: diffHours })
  } else if (diffDays === 1) {
    return t('datetime.yesterday')
  } else if (diffDays < 7) {
    return t('datetime.daysAgo', { n: diffDays })
  } else if (diffWeeks < 4) {
    return t('datetime.weeksAgo', { n: diffWeeks })
  } else if (diffMonths < 12) {
    return t('datetime.monthsAgo', { n: diffMonths })
  } else {
    return t('datetime.yearsAgo', { n: diffYears })
  }
}

/**
 * 格式化日期时间
 */
export function formatDateTime(date: string | Date, format: 'full' | 'short' = 'full'): string {
  const targetDate = typeof date === 'string' ? new Date(date) : date
  const { locale } = useI18n()

  const options: Intl.DateTimeFormatOptions =
    format === 'full'
      ? {
          year: 'numeric',
          month: 'long',
          day: 'numeric',
          hour: '2-digit',
          minute: '2-digit',
        }
      : {
          year: 'numeric',
          month: 'short',
          day: 'numeric',
        }

  return targetDate.toLocaleDateString(locale.value, options)
}

/**
 * 格式化日期
 */
export function formatDate(date: string | Date): string {
  return formatDateTime(date, 'short')
}

/**
 * 格式化时间
 */
export function formatTime(date: string | Date): string {
  const targetDate = typeof date === 'string' ? new Date(date) : date
  const { locale } = useI18n()

  return targetDate.toLocaleTimeString(locale.value, {
    hour: '2-digit',
    minute: '2-digit',
  })
}

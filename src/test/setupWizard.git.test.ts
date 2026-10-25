import { describe, it, expect, vi, beforeEach } from 'vitest'
import { mount, flushPromises } from '@vue/test-utils'
import { nextTick } from 'vue'
import type { ProfileConfig } from '../types'

/** 仅用于测试的组件实例形状（暴露了响应式 form） */
type WizardVm = { form: ProfileConfig }

// Mock Tauri path APIs used by the wizard's applyTypeDefaults()
vi.mock('@tauri-apps/api/path', () => ({
  appDataDir: vi.fn().mockResolvedValue('/tmp/fake-app-data'),
  join: (...parts: string[]) => parts.join('/'),
}))

// Mock Tauri dialog plugin (only used on button click, not on render)
vi.mock('@tauri-apps/plugin-dialog', () => ({
  open: vi.fn().mockResolvedValue('/tmp/fake-repo'),
  save: vi.fn().mockResolvedValue('/tmp/fake.db'),
}))

// Mock Tauri OS plugin (i18n queries system locale at module load)
vi.mock('@tauri-apps/plugin-os', () => ({
  locale: vi.fn().mockResolvedValue('zh-CN'),
  platform: vi.fn().mockResolvedValue('macos'),
}))

// Mock the API module so no real IPC is attempted
vi.mock('../api/note', () => ({
  profileApi: {
    generateEncryptionKey: vi.fn().mockResolvedValue('fake-key'),
    saveProfile: vi.fn().mockResolvedValue(undefined),
    reconnectProfile: vi.fn().mockResolvedValue(undefined),
    gitAvailable: vi.fn().mockResolvedValue(true),
    testGitConnection: vi.fn().mockResolvedValue(true),
  },
  noteApi: {},
}))

import i18n from '../i18n'
import { profileApi } from '../api/note'
import SetupWizard from '../components/SetupWizard.vue'

describe('SetupWizard git flow reproduction', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('selects git backend and advances to step 1 without crashing', async () => {
    const wrapper = mount(SetupWizard, {
      global: {
        plugins: [i18n],
      },
    })

    // Step 0: click the "Git 仓库" backend card (the clickable card has class cursor-pointer)
    const cards = wrapper.findAll('.cursor-pointer')
    const gitCard = cards.find((el) => el.text().includes('Git 仓库'))
    expect(gitCard, 'Git option card should render on step 0').toBeTruthy()
    await gitCard!.trigger('click')
    await nextTick()
    // Sanity: git should now be the selected backend
    expect(wrapper.html()).toContain('Git 仓库')

    // Click "下一步" (Next) to go to step 1 (connection config -> git form)
    const nextBtn = wrapper.findAll('button').find((el) => el.text().trim() === '下一步')
    expect(nextBtn, 'Next button should be present').toBeTruthy()
    await nextBtn!.trigger('click')
    await flushPromises()
    await nextTick()

    // After advancing, the git repo path form (step 1) should render without throwing
    expect(
      wrapper.html().includes('仓库路径'),
      'Step 1 git form should render without throwing',
    ).toBeTruthy()
  })

  // Helper: drive the wizard to the git connection step (step 1)
  async function gotoGitStep(wrapper: ReturnType<typeof mount>) {
    const cards = wrapper.findAll('.cursor-pointer')
    const gitCard = cards.find((el) => el.text().includes('Git 仓库'))
    await gitCard!.trigger('click')
    await nextTick()
    const nextBtn = wrapper.findAll('button').find((el) => el.text().trim() === '下一步')
    await nextBtn!.trigger('click')
    await flushPromises()
    await nextTick()
  }

  it('auto-switches remote auth to HTTPS when an https URL is entered', async () => {
    const wrapper = mount(SetupWizard, { global: { plugins: [i18n] } })
    await gotoGitStep(wrapper)

    const urlInput = wrapper
      .findAll('input')
      .find((el) => (el.attributes('placeholder') || '').includes('github.com'))
    expect(urlInput, 'Git remote URL input should be present').toBeTruthy()

    await urlInput!.setValue('https://github.com/user/notes.git')
    await urlInput!.trigger('blur')
    await nextTick()

    // Blur should infer 'https' from the https:// scheme
    expect((wrapper.vm as unknown as WizardVm).form.git?.remoteAuth).toBe('https')
    // The HTTPS auth segment should now be the active (highlighted) option
    const httpsBtn = wrapper.findAll('button').find((el) => el.text().trim() === 'HTTPS')
    expect(httpsBtn!.classes().join(' ')).toContain('text-indigo-600')
  })

  it('auto-switches remote auth to SSH when a git@ URL is entered', async () => {
    const wrapper = mount(SetupWizard, { global: { plugins: [i18n] } })
    await gotoGitStep(wrapper)

    const urlInput = wrapper
      .findAll('input')
      .find((el) => (el.attributes('placeholder') || '').includes('github.com'))
    await urlInput!.setValue('git@github.com:user/notes.git')
    await urlInput!.trigger('blur')
    await nextTick()

    expect((wrapper.vm as unknown as WizardVm).form.git?.remoteAuth).toBe('ssh')
  })

  it('shows a persistent inline result after clicking 测试连接', async () => {
    const wrapper = mount(SetupWizard, { global: { plugins: [i18n] } })
    await gotoGitStep(wrapper)

    const urlInput = wrapper
      .findAll('input')
      .find((el) => (el.attributes('placeholder') || '').includes('github.com'))
    await urlInput!.setValue('https://github.com/user/notes.git')
    await urlInput!.trigger('blur')
    await nextTick()

    const testBtn = wrapper.findAll('button').find((el) => el.text().trim() === '测试连接')
    expect(testBtn, 'Test connection button should be visible').toBeTruthy()
    await testBtn!.trigger('click')
    await flushPromises()
    await nextTick()

    // Inline result (inside the component, not the transient toast) must show success
    expect(
      wrapper.html().includes('连接成功'),
      'Inline test result should show success message',
    ).toBeTruthy()
  })

  it('emits complete only AFTER reconnectProfile resolves (DB_NOT_CONNECTED race)', async () => {
    // Deferred reconnect so we can assert the ordering before it resolves.
    // Regression for: first-setup entered main mode before the backend had set
    // note_store, so every data load returned DB_NOT_CONNECTED (a pile of errors).
    let resolveReconnect!: () => void
    const reconnectPending = new Promise<void>((res) => {
      resolveReconnect = res
    })
    vi.mocked(profileApi.reconnectProfile).mockReturnValue(
      reconnectPending as unknown as Promise<void>,
    )

    const wrapper = mount(SetupWizard, { global: { plugins: [i18n] } })

    // Step 0: pick the local database backend
    const dbCard = wrapper.findAll('.cursor-pointer').find((el) => el.text().includes('数据库'))
    expect(dbCard, 'database backend card should render').toBeTruthy()
    await dbCard!.trigger('click')
    await nextTick()

    // Advance step 0 -> 1 -> 2 (finish/save)
    for (let i = 0; i < 2; i++) {
      const nextBtn = wrapper.findAll('button').find((el) => el.text().trim() === '下一步')
      expect(nextBtn, `Next button should be present at round ${i}`).toBeTruthy()
      await nextBtn!.trigger('click')
      await flushPromises()
      await nextTick()
    }

    // Step 2: click "保存并连接"
    const saveBtn = wrapper.findAll('button').find((el) => el.text().trim() === '保存并连接')
    expect(saveBtn, 'save-and-connect button should be present').toBeTruthy()
    await saveBtn!.trigger('click')
    await flushPromises()
    await nextTick()

    // While reconnect is still pending, complete must NOT be emitted
    expect(
      wrapper.emitted('complete'),
      'complete must wait until reconnectProfile resolves',
    ).toBeFalsy()

    // Now resolve the reconnect
    resolveReconnect()
    await flushPromises()
    await nextTick()

    expect(
      wrapper.emitted('complete'),
      'complete should fire after reconnectProfile resolves',
    ).toBeTruthy()
  })
})

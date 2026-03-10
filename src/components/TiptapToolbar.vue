<template>
  <div class="tiptap-toolbar-wrapper" role="toolbar" :aria-label="t('aria.toolbar')">
    <!-- 左侧固定区域：Content Type -->
    <div class="toolbar-fixed toolbar-fixed-left">
      <!-- 新建笔记时的模式选择 -->
      <div v-if="isNewNote" class="toolbar-section">
        <Tooltip :content="t('editor.toolbarTooltip.contentType')" placement="bottom">
          <select
            :value="contentType"
            @change="
              emit('update:content-type', Number(($event.target as HTMLSelectElement).value))
            "
            class="h-8 px-2 text-sm border border-edge rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent font-medium"
            :class="
              isMarkdownMode
                ? 'bg-slate-800 text-white border-edge-dark'
                : 'bg-indigo-50 text-indigo-700 border-indigo-300'
            "
          >
            <option :value="ContentType.Html">{{ t('editor.contentType.richText') }}</option>
            <option :value="ContentType.Markdown">{{ t('editor.contentType.markdown') }}</option>
          </select>
        </Tooltip>
      </div>

      <!-- 模式标识（已保存的笔记） -->
      <div v-else class="toolbar-section">
        <span
          class="h-8 px-3 text-sm rounded-lg flex items-center font-medium"
          :class="isMarkdownMode ? 'bg-slate-800 text-white' : 'bg-indigo-50 text-indigo-700'"
        >
          {{ isMarkdownMode ? 'Markdown' : t('editor.contentType.richText') }}
        </span>
      </div>
    </div>

    <!-- 左箭头 -->
    <button
      class="scroll-btn scroll-btn-left"
      @click="scrollLeft"
      :disabled="!canScrollLeft"
      :class="{ hidden: !canScrollLeft }"
      aria-label="Scroll left"
    >
      <ChevronLeft class="w-5 h-5" />
    </button>

    <!-- 中间可滚动区域：编辑工具 -->
    <div class="tiptap-toolbar" ref="toolbarRef" @scroll="updateScrollState">
      <!-- 富文本模式工具栏（始终显示，非编辑模式禁用） -->
      <template v-if="!isMarkdownMode && editor">
        <!-- 标题和字体 -->
        <div class="toolbar-section">
          <Tooltip :content="t('editor.toolbarTooltip.headingLevel')" placement="bottom">
            <select
              v-model="headingLevel"
              @change="setHeading"
              :disabled="!editMode"
              class="h-8 px-2 text-sm border border-edge rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent disabled:opacity-50 disabled:cursor-not-allowed"
            >
              <option value="0">{{ t('editor.headingOptions.normal') }}</option>
              <option value="1">{{ t('editor.headingOptions.h1') }}</option>
              <option value="2">{{ t('editor.headingOptions.h2') }}</option>
              <option value="3">{{ t('editor.headingOptions.h3') }}</option>
              <option value="4">{{ t('editor.headingOptions.h4') }}</option>
              <option value="5">{{ t('editor.headingOptions.h5') }}</option>
              <option value="6">{{ t('editor.headingOptions.h6') }}</option>
            </select>
          </Tooltip>

          <Tooltip :content="t('editor.toolbarTooltip.fontFamily')" placement="bottom">
            <select
              v-model="fontFamily"
              @change="setFontFamily"
              :disabled="!editMode"
              class="h-8 px-2 text-sm border border-edge rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent ml-1 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              <option value="">{{ t('editor.fontOptions.default') }}</option>
              <optgroup :label="t('editor.fontOptions.sansSerif')">
                <option value="Arial, sans-serif">Arial</option>
                <option value="Helvetica, sans-serif">Helvetica</option>
                <option value="Verdana, sans-serif">Verdana</option>
                <option value="Tahoma, sans-serif">Tahoma</option>
                <option value="Trebuchet MS, sans-serif">Trebuchet MS</option>
                <option value="Microsoft YaHei, sans-serif">
                  {{ t('fontName.microsoftYaHei') }}
                </option>
                <option value="PingFang SC, sans-serif">{{ t('fontName.pingFang') }}</option>
              </optgroup>
              <optgroup :label="t('editor.fontOptions.serif')">
                <option value="Times New Roman, serif">Times New Roman</option>
                <option value="Georgia, serif">Georgia</option>
                <option value="Palatino, serif">Palatino</option>
                <option value="SimSun, serif">{{ t('fontName.simSun') }}</option>
                <option value="KaiTi, serif">{{ t('fontName.kaiTi') }}</option>
                <option value="FangSong, serif">{{ t('fontName.fangSong') }}</option>
              </optgroup>
              <optgroup :label="t('editor.fontOptions.monospace')">
                <option value="Courier New, monospace">Courier New</option>
                <option value="Consolas, monospace">Consolas</option>
                <option value="Monaco, monospace">Monaco</option>
                <option value="Source Code Pro, monospace">Source Code Pro</option>
              </optgroup>
              <optgroup :label="t('editor.fontOptions.artistic')">
                <option value="Comic Sans MS, cursive">Comic Sans MS</option>
                <option value="Impact, fantasy">Impact</option>
                <option value="Brush Script MT, cursive">Brush Script</option>
              </optgroup>
            </select>
          </Tooltip>

          <Tooltip :content="t('editor.toolbarTooltip.fontSize')" placement="bottom">
            <select
              v-model="fontSize"
              @change="setFontSize"
              :disabled="!editMode"
              class="h-8 px-2 text-sm border border-edge rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent ml-1 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              <option value="">{{ t('editor.fontOptions.defaultSize') }}</option>
              <option value="12px">12px</option>
              <option value="14px">14px</option>
              <option value="16px">16px</option>
              <option value="18px">18px</option>
              <option value="20px">20px</option>
              <option value="24px">24px</option>
              <option value="28px">28px</option>
              <option value="32px">32px</option>
              <option value="36px">36px</option>
              <option value="48px">48px</option>
              <option value="72px">72px</option>
            </select>
          </Tooltip>
        </div>

        <!-- 字体样式 -->
        <div class="toolbar-section">
          <div class="btn-group">
            <Tooltip :content="t('editor.toolbarTooltip.bold')" placement="bottom">
              <button
                :class="['toolbar-btn', { active: editor.isActive('bold') }]"
                @click="editor.chain().focus().toggleBold().run()"
                :disabled="!editMode || !editor.can().chain().focus().toggleBold().run()"
                :aria-label="t('editor.toolbarTooltip.bold')"
                :aria-pressed="editor.isActive('bold')"
              >
                <Bold class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip :content="t('editor.toolbarTooltip.italic')" placement="bottom">
              <button
                :class="['toolbar-btn', { active: editor.isActive('italic') }]"
                @click="editor.chain().focus().toggleItalic().run()"
                :disabled="!editMode || !editor.can().chain().focus().toggleItalic().run()"
                :aria-label="t('editor.toolbarTooltip.italic')"
                :aria-pressed="editor.isActive('italic')"
              >
                <Italic class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip :content="t('editor.toolbarTooltip.underline')" placement="bottom">
              <button
                :class="['toolbar-btn', { active: editor.isActive('underline') }]"
                @click="editor.chain().focus().toggleUnderline().run()"
                :disabled="!editMode"
                :aria-label="t('editor.toolbarTooltip.underline')"
                :aria-pressed="editor.isActive('underline')"
              >
                <UnderlineIcon class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip :content="t('editor.toolbarTooltip.strikethrough')" placement="bottom">
              <button
                :class="['toolbar-btn', { active: editor.isActive('strike') }]"
                @click="editor.chain().focus().toggleStrike().run()"
                :disabled="!editMode || !editor.can().chain().focus().toggleStrike().run()"
                :aria-label="t('editor.toolbarTooltip.strikethrough')"
                :aria-pressed="editor.isActive('strike')"
              >
                <Strikethrough class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip :content="t('editor.toolbarTooltip.superscript')" placement="bottom">
              <button
                :class="['toolbar-btn', { active: editor.isActive('superscript') }]"
                @click="editor.chain().focus().toggleSuperscript().run()"
                :disabled="!editMode"
                :aria-label="t('editor.toolbarTooltip.superscript')"
                :aria-pressed="editor.isActive('superscript')"
              >
                <Superscript class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip :content="t('editor.toolbarTooltip.subscript')" placement="bottom">
              <button
                :class="['toolbar-btn', { active: editor.isActive('subscript') }]"
                @click="editor.chain().focus().toggleSubscript().run()"
                :disabled="!editMode"
                :aria-label="t('editor.toolbarTooltip.subscript')"
                :aria-pressed="editor.isActive('subscript')"
              >
                <Subscript class="w-4 h-4" />
              </button>
            </Tooltip>
          </div>
        </div>

        <!-- 文本对齐 -->
        <div class="toolbar-section">
          <div class="btn-group">
            <Tooltip :content="t('editor.toolbarTooltip.alignLeft')" placement="bottom">
              <button
                :class="['toolbar-btn', { active: editor.isActive({ textAlign: 'left' }) }]"
                @click="editor.chain().focus().setTextAlign('left').run()"
                :disabled="!editMode"
                :aria-label="t('editor.toolbarTooltip.alignLeft')"
                :aria-pressed="editor.isActive({ textAlign: 'left' })"
              >
                <AlignLeft class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip :content="t('editor.toolbarTooltip.alignCenter')" placement="bottom">
              <button
                :class="['toolbar-btn', { active: editor.isActive({ textAlign: 'center' }) }]"
                @click="editor.chain().focus().setTextAlign('center').run()"
                :disabled="!editMode"
                :aria-label="t('editor.toolbarTooltip.alignCenter')"
                :aria-pressed="editor.isActive({ textAlign: 'center' })"
              >
                <AlignCenter class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip :content="t('editor.toolbarTooltip.alignRight')" placement="bottom">
              <button
                :class="['toolbar-btn', { active: editor.isActive({ textAlign: 'right' }) }]"
                @click="editor.chain().focus().setTextAlign('right').run()"
                :disabled="!editMode"
                :aria-label="t('editor.toolbarTooltip.alignRight')"
                :aria-pressed="editor.isActive({ textAlign: 'right' })"
              >
                <AlignRight class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip :content="t('editor.toolbarTooltip.alignJustify')" placement="bottom">
              <button
                :class="['toolbar-btn', { active: editor.isActive({ textAlign: 'justify' }) }]"
                @click="editor.chain().focus().setTextAlign('justify').run()"
                :disabled="!editMode"
                :aria-label="t('editor.toolbarTooltip.alignJustify')"
                :aria-pressed="editor.isActive({ textAlign: 'justify' })"
              >
                <AlignJustify class="w-4 h-4" />
              </button>
            </Tooltip>
          </div>
        </div>

        <!-- 列表 -->
        <div class="toolbar-section">
          <div class="btn-group">
            <Tooltip :content="t('editor.toolbarTooltip.bulletList')" placement="bottom">
              <button
                :class="['toolbar-btn', { active: editor.isActive('bulletList') }]"
                @click="editor.chain().focus().toggleBulletList().run()"
                :disabled="!editMode"
                :aria-label="t('editor.toolbarTooltip.bulletList')"
                :aria-pressed="editor.isActive('bulletList')"
              >
                <List class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip :content="t('editor.toolbarTooltip.orderedList')" placement="bottom">
              <button
                :class="['toolbar-btn', { active: editor.isActive('orderedList') }]"
                @click="editor.chain().focus().toggleOrderedList().run()"
                :disabled="!editMode"
                :aria-label="t('editor.toolbarTooltip.orderedList')"
                :aria-pressed="editor.isActive('orderedList')"
              >
                <ListOrdered class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip :content="t('editor.toolbarTooltip.taskList')" placement="bottom">
              <button
                :class="['toolbar-btn', { active: editor.isActive('taskList') }]"
                @click="editor.chain().focus().toggleTaskList().run()"
                :disabled="!editMode"
                :aria-label="t('editor.toolbarTooltip.taskList')"
                :aria-pressed="editor.isActive('taskList')"
              >
                <ListChecks class="w-4 h-4" />
              </button>
            </Tooltip>
          </div>
        </div>

        <!-- 缩进 -->
        <div class="toolbar-section">
          <div class="btn-group">
            <Tooltip :content="t('editor.toolbarTooltip.outdent')" placement="bottom">
              <button
                class="toolbar-btn"
                @click="editor.chain().focus().outdent().run()"
                :disabled="!editMode"
                :aria-label="t('editor.toolbarTooltip.outdent')"
              >
                <IndentDecrease class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip :content="t('editor.toolbarTooltip.indent')" placement="bottom">
              <button
                class="toolbar-btn"
                @click="editor.chain().focus().indent().run()"
                :disabled="!editMode"
                :aria-label="t('editor.toolbarTooltip.indent')"
              >
                <IndentIncrease class="w-4 h-4" />
              </button>
            </Tooltip>
          </div>
        </div>

        <!-- 引用和代码 -->
        <div class="toolbar-section">
          <div class="btn-group">
            <Tooltip :content="t('editor.toolbarTooltip.quote')" placement="bottom">
              <button
                :class="['toolbar-btn', { active: editor.isActive('blockquote') }]"
                @click="editor.chain().focus().toggleBlockquote().run()"
                :disabled="!editMode"
                :aria-label="t('editor.toolbarTooltip.quote')"
                :aria-pressed="editor.isActive('blockquote')"
              >
                <Quote class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip :content="t('editor.toolbarTooltip.codeBlock')" placement="bottom">
              <button
                :class="['toolbar-btn', { active: editor.isActive('codeBlock') }]"
                @click="editor.chain().focus().toggleCodeBlock().run()"
                :disabled="!editMode"
                :aria-label="t('editor.toolbarTooltip.codeBlock')"
                :aria-pressed="editor.isActive('codeBlock')"
              >
                <Code2 class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip :content="t('editor.toolbarTooltip.inlineCode')" placement="bottom">
              <button
                :class="['toolbar-btn', { active: editor.isActive('code') }]"
                @click="editor.chain().focus().toggleCode().run()"
                :disabled="!editMode"
                :aria-label="t('editor.toolbarTooltip.inlineCode')"
                :aria-pressed="editor.isActive('code')"
              >
                <Code class="w-4 h-4" />
              </button>
            </Tooltip>
          </div>
        </div>

        <!-- 链接和图片 -->
        <div class="toolbar-section">
          <div class="btn-group">
            <Tooltip :content="t('editor.toolbarTooltip.link')" placement="bottom">
              <button
                :class="['toolbar-btn', { active: editor.isActive('link') }]"
                @click="openLinkDialog"
                :disabled="!editMode"
                :aria-label="t('editor.toolbarTooltip.link')"
                :aria-pressed="editor.isActive('link')"
              >
                <LinkIcon class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip :content="t('editor.toolbarTooltip.unlink')" placement="bottom">
              <button
                class="toolbar-btn"
                @click="editor.chain().focus().unsetLink().run()"
                :disabled="!editMode || !editor.isActive('link')"
                :aria-label="t('editor.toolbarTooltip.unlink')"
              >
                <Unlink class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip :content="t('editor.toolbarTooltip.image')" placement="bottom">
              <button
                class="toolbar-btn"
                @click="openImageDialog"
                :disabled="!editMode"
                :aria-label="t('editor.toolbarTooltip.image')"
              >
                <ImageIcon class="w-4 h-4" />
              </button>
            </Tooltip>
          </div>
        </div>

        <!-- 表格 -->
        <div class="toolbar-section">
          <div class="btn-group">
            <Tooltip :content="t('editor.toolbarTooltip.insertTable')" placement="bottom">
              <button
                class="toolbar-btn"
                @click="insertTable"
                :disabled="!editMode"
                :aria-label="t('editor.toolbarTooltip.insertTable')"
              >
                <TableIcon class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip :content="t('editor.toolbarTooltip.addColumn')" placement="bottom">
              <button
                class="toolbar-btn"
                @click="editor.chain().focus().addColumnAfter().run()"
                :disabled="!editMode || !editor.can().addColumnAfter()"
                :aria-label="t('editor.toolbarTooltip.addColumn')"
              >
                <Columns3 class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip :content="t('editor.toolbarTooltip.addRow')" placement="bottom">
              <button
                class="toolbar-btn"
                @click="editor.chain().focus().addRowAfter().run()"
                :disabled="!editMode || !editor.can().addRowAfter()"
                :aria-label="t('editor.toolbarTooltip.addRow')"
              >
                <Rows3 class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip :content="t('editor.toolbarTooltip.deleteTable')" placement="bottom">
              <button
                class="toolbar-btn"
                @click="editor.chain().focus().deleteTable().run()"
                :disabled="!editMode || !editor.can().deleteTable()"
                :aria-label="t('editor.toolbarTooltip.deleteTable')"
              >
                <TableOff class="w-4 h-4" />
              </button>
            </Tooltip>
          </div>
        </div>

        <!-- 其他功能 -->
        <div class="toolbar-section">
          <div class="btn-group">
            <Tooltip :content="t('editor.toolbarTooltip.highlight')" placement="bottom">
              <button
                :class="['toolbar-btn', { active: editor.isActive('highlight') }]"
                @click="editor.chain().focus().toggleHighlight().run()"
                :disabled="!editMode"
                :aria-label="t('editor.toolbarTooltip.highlight')"
                :aria-pressed="editor.isActive('highlight')"
              >
                <Highlighter class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip :content="t('editor.toolbarTooltip.divider')" placement="bottom">
              <button
                class="toolbar-btn"
                @click="editor.chain().focus().setHorizontalRule().run()"
                :disabled="!editMode"
                :aria-label="t('editor.toolbarTooltip.divider')"
              >
                <Minus class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip :content="t('editor.toolbarTooltip.clearFormat')" placement="bottom">
              <button
                class="toolbar-btn"
                @click="editor.chain().focus().clearNodes().unsetAllMarks().run()"
                :disabled="!editMode"
                :aria-label="t('editor.toolbarTooltip.clearFormat')"
              >
                <RemoveFormatting class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip :content="t('editor.toolbarTooltip.findReplace')" placement="bottom">
              <button
                :class="['toolbar-btn', { active: searchDialogVisible }]"
                @click="toggleSearchDialog"
                :disabled="!editMode"
                :aria-label="t('editor.toolbarTooltip.findReplace')"
                :aria-pressed="searchDialogVisible"
              >
                <Search class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip :content="t('editor.toolbarTooltip.toc')" placement="bottom">
              <button
                :class="['toolbar-btn', { active: tocVisible }]"
                @click="emit('toggle-toc')"
                :aria-label="t('editor.toolbarTooltip.toc')"
                :aria-pressed="tocVisible"
              >
                <ListTree class="w-4 h-4" />
              </button>
            </Tooltip>
          </div>
        </div>

        <!-- 撤销和重做 -->
        <div class="toolbar-section">
          <div class="btn-group">
            <Tooltip :content="t('editor.toolbarTooltip.undo')" placement="bottom">
              <button
                class="toolbar-btn"
                @click="editor.chain().focus().undo().run()"
                :disabled="!editMode || !editor.can().chain().focus().undo().run()"
                :aria-label="t('editor.toolbarTooltip.undo')"
              >
                <Undo2 class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip :content="t('editor.toolbarTooltip.redo')" placement="bottom">
              <button
                class="toolbar-btn"
                @click="editor.chain().focus().redo().run()"
                :disabled="!editMode || !editor.can().chain().focus().redo().run()"
                :aria-label="t('editor.toolbarTooltip.redo')"
              >
                <Redo2 class="w-4 h-4" />
              </button>
            </Tooltip>
          </div>
        </div>

        <!-- 颜色选择器 -->
        <div class="toolbar-section">
          <Tooltip :content="t('editor.toolbarTooltip.textColor')" placement="bottom">
            <ColorPicker
              v-model="textColor"
              @change="setTextColor"
              :predefine="predefineColors"
              :disabled="!editMode"
            />
          </Tooltip>

          <Tooltip :content="t('editor.toolbarTooltip.backgroundColor')" placement="bottom">
            <ColorPicker
              v-model="highlightColor"
              @change="setHighlightColor"
              :predefine="predefineColors"
              :disabled="!editMode"
            />
          </Tooltip>
        </div>
      </template>

      <!-- Markdown 源码/预览切换（Markdown 模式显示） -->
      <div v-if="isMarkdownMode" class="toolbar-section">
        <div class="btn-group">
          <Tooltip
            :content="
              sourceMode
                ? t('editor.toolbarTooltip.sourceMode')
                : t('editor.toolbarTooltip.markdownSource')
            "
            placement="bottom"
          >
            <button
              :class="['toolbar-btn', { active: sourceMode }]"
              @click="emit('toggle-source-mode')"
              :disabled="!editMode"
              :aria-label="
                sourceMode
                  ? t('editor.toolbarTooltip.sourceMode')
                  : t('editor.toolbarTooltip.markdownSource')
              "
              :aria-pressed="sourceMode"
            >
              <FileCode v-if="!sourceMode" class="w-4 h-4" />
              <Eye v-else class="w-4 h-4" />
            </button>
          </Tooltip>

          <Tooltip :content="t('editor.toolbarTooltip.verticalLayout')" placement="bottom">
            <button
              :class="['toolbar-btn', { active: markdownLayout === MarkdownLayout.Vertical }]"
              @click="toggleLayout(MarkdownLayout.Vertical)"
              :disabled="!editMode"
              :aria-label="t('editor.toolbarTooltip.verticalLayout')"
              :aria-pressed="markdownLayout === MarkdownLayout.Vertical"
            >
              <PanelTop class="w-4 h-4" />
            </button>
          </Tooltip>

          <Tooltip :content="t('editor.toolbarTooltip.horizontalLayout')" placement="bottom">
            <button
              :class="['toolbar-btn', { active: markdownLayout === MarkdownLayout.Horizontal }]"
              @click="toggleLayout(MarkdownLayout.Horizontal)"
              :disabled="!editMode"
              :aria-label="t('editor.toolbarTooltip.horizontalLayout')"
              :aria-pressed="markdownLayout === MarkdownLayout.Horizontal"
            >
              <PanelLeft class="w-4 h-4" />
            </button>
          </Tooltip>
        </div>
      </div>
    </div>

    <!-- 右箭头 -->
    <button
      class="scroll-btn scroll-btn-right"
      @click="scrollRight"
      :disabled="!canScrollRight"
      :class="{ hidden: !canScrollRight }"
      aria-label="Scroll right"
    >
      <ChevronRight class="w-5 h-5" />
    </button>

    <!-- 右侧固定区域：操作按钮 -->
    <div class="toolbar-fixed toolbar-fixed-right">
      <ActionButtons
        :edit-mode="editMode"
        @edit="emit('edit')"
        @save="emit('save')"
        @cancel="emit('cancel')"
        @delete="emit('delete')"
        @settings="emit('settings')"
        @history="emit('history')"
        @export="emit('export')"
      />
    </div>
  </div>

  <!-- 链接弹窗 -->
  <LinkDialog v-model="linkDialogVisible" :editor="editor" />

  <!-- 图片弹窗 -->
  <ImageDialog v-model="imageDialogVisible" :editor="editor" />

  <!-- 查找替换弹窗 -->
  <SearchReplaceDialog v-model="searchDialogVisible" :editor="editor" />
</template>

<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted, onBeforeUnmount, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import type { Editor } from '@tiptap/vue-3'
import { ContentType, MarkdownLayout } from '../types'
import { Tooltip, ColorPicker } from './ui'
import ActionButtons from './toolbar/ActionButtons.vue'
import LinkDialog from './toolbar/LinkDialog.vue'
import ImageDialog from './toolbar/ImageDialog.vue'
import SearchReplaceDialog from './toolbar/SearchReplaceDialog.vue'
import {
  Bold,
  Italic,
  Underline as UnderlineIcon,
  Strikethrough,
  AlignLeft,
  AlignCenter,
  AlignRight,
  AlignJustify,
  List,
  ListOrdered,
  ListChecks,
  Quote,
  Code2,
  Code,
  Highlighter,
  Minus,
  RemoveFormatting,
  Undo2,
  Redo2,
  Link as LinkIcon,
  Unlink,
  Image as ImageIcon,
  Table as TableIcon,
  Columns3,
  Rows3,
  TableProperties as TableOff,
  ChevronLeft,
  ChevronRight,
  FileCode,
  Eye,
  PanelTop,
  PanelLeft,
  Subscript,
  Superscript,
  IndentDecrease,
  IndentIncrease,
  Search,
  ListTree,
} from 'lucide-vue-next'

const { t } = useI18n()

interface Props {
  editor: Editor | null
  sourceMode?: boolean
  contentType?: ContentType
  isNewNote?: boolean
  editMode?: boolean
  markdownLayout?: MarkdownLayout
  tocVisible?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  sourceMode: false,
  contentType: ContentType.Html,
  isNewNote: false,
  editMode: false,
  markdownLayout: MarkdownLayout.None,
  tocVisible: false,
})

const emit = defineEmits<{
  'toggle-source-mode': []
  'update:content-type': [contentType: ContentType]
  'update:markdown-layout': [layout: MarkdownLayout]
  'toggle-toc': []
  edit: []
  save: []
  cancel: []
  delete: []
  settings: []
  history: []
  export: []
}>()

const toggleLayout = (layout: MarkdownLayout) => {
  if (props.markdownLayout === layout) {
    emit('update:markdown-layout', MarkdownLayout.None)
  } else {
    emit('update:markdown-layout', layout)
  }
}

// 是否为 Markdown 模式
const isMarkdownMode = computed(() => props.contentType === ContentType.Markdown)

const headingLevel = ref('0')
const fontFamily = ref('')
const fontSize = ref('')
const textColor = ref('#000000')
const highlightColor = ref('#FFFF00')

// 弹窗状态
const linkDialogVisible = ref(false)
const imageDialogVisible = ref(false)
const searchDialogVisible = ref(false)

// 滚动相关
const toolbarRef = ref<HTMLElement | null>(null)
const canScrollLeft = ref(false)
const canScrollRight = ref(false)
const scrollAmount = 200

// 更新滚动状态
const updateScrollState = () => {
  if (!toolbarRef.value) return
  const { scrollLeft, scrollWidth, clientWidth } = toolbarRef.value
  canScrollLeft.value = scrollLeft > 0
  canScrollRight.value = scrollLeft + clientWidth < scrollWidth - 1
}

// 向左滚动
const scrollLeft = () => {
  if (!toolbarRef.value) return
  toolbarRef.value.scrollBy({ left: -scrollAmount, behavior: 'smooth' })
}

// 向右滚动
const scrollRight = () => {
  if (!toolbarRef.value) return
  toolbarRef.value.scrollBy({ left: scrollAmount, behavior: 'smooth' })
}

// 监听窗口大小变化
onMounted(() => {
  updateScrollState()
  window.addEventListener('resize', updateScrollState)
})

onUnmounted(() => {
  window.removeEventListener('resize', updateScrollState)
})

// 预定义颜色
const predefineColors = [
  '#000000',
  '#ffffff',
  '#ff4500',
  '#ff8c00',
  '#ffd700',
  '#90ee90',
  '#00ced1',
  '#1e90ff',
  '#c71585',
  '#8b4513',
]

// 设置标题
const setHeading = () => {
  if (!props.editor) return
  const level = parseInt(headingLevel.value)
  if (level === 0) {
    props.editor.chain().focus().setParagraph().run()
  } else {
    props.editor
      .chain()
      .focus()
      .toggleHeading({ level: level as 1 | 2 | 3 | 4 | 5 | 6 })
      .run()
  }
}

// 设置字体
const setFontFamily = () => {
  if (!props.editor) return
  if (fontFamily.value) {
    props.editor.chain().focus().setFontFamily(fontFamily.value).run()
  } else {
    props.editor.chain().focus().unsetFontFamily().run()
  }
}

// 设置字号
const setFontSize = () => {
  if (!props.editor) return
  if (fontSize.value) {
    props.editor.chain().focus().setFontSize(fontSize.value).run()
  } else {
    props.editor.chain().focus().unsetFontSize().run()
  }
}

// 设置文本颜色
const setTextColor = (color: string) => {
  if (!props.editor) return
  props.editor.chain().focus().setColor(color).run()
}

// 设置背景颜色
const setHighlightColor = (color: string) => {
  if (!props.editor) return
  props.editor.chain().focus().toggleHighlight({ color }).run()
}

// 打开链接弹窗
const openLinkDialog = () => {
  linkDialogVisible.value = true
}

// 打开图片弹窗
const openImageDialog = () => {
  imageDialogVisible.value = true
}

// 插入表格
const insertTable = () => {
  if (!props.editor) return
  props.editor.chain().focus().insertTable({ rows: 3, cols: 3, withHeaderRow: true }).run()
}

// 切换查找替换弹窗
const toggleSearchDialog = () => {
  searchDialogVisible.value = !searchDialogVisible.value
}

// 同步编辑器状态到工具栏控件（统一处理，避免多个 watch 各自触发）
const syncEditorState = () => {
  const editor = props.editor
  if (!editor) return

  // 标题级别
  if (!editor.isActive('heading')) {
    headingLevel.value = '0'
  } else {
    for (let i = 1; i <= 6; i++) {
      if (editor.isActive('heading', { level: i })) {
        headingLevel.value = i.toString()
        break
      }
    }
  }

  // 字体、字号
  const textStyle = editor.getAttributes('textStyle')
  fontFamily.value = textStyle?.fontFamily || ''
  fontSize.value = textStyle?.fontSize || ''
  if (textStyle?.color) textColor.value = textStyle.color

  // 高亮色
  const highlight = editor.getAttributes('highlight')
  if (highlight?.color) highlightColor.value = highlight.color
}

// 监听编辑器实例切换，绑定/解绑 selectionUpdate 事件
watch(
  () => props.editor,
  (editor, oldEditor) => {
    if (oldEditor) oldEditor.off('selectionUpdate', syncEditorState)
    if (editor) {
      editor.on('selectionUpdate', syncEditorState)
      syncEditorState()
    }
  },
  { immediate: true },
)

onBeforeUnmount(() => {
  if (props.editor) props.editor.off('selectionUpdate', syncEditorState)
})
</script>

<style scoped>
/* 工具栏外层容器 */
.tiptap-toolbar-wrapper {
  display: flex;
  align-items: center;
  border-bottom: 1px solid var(--color-border);
  background-color: var(--color-bg-secondary);
  position: relative;
}

/* 固定区域 */
.toolbar-fixed {
  display: flex;
  align-items: center;
  padding: 0.5rem;
  background-color: var(--color-bg-secondary);
  flex-shrink: 0;
  z-index: 5;
}

.toolbar-fixed-left {
  border-right: 1px solid var(--color-border);
  background: linear-gradient(to left, var(--color-bg-tertiary), var(--color-bg-secondary));
  padding-left: 0.75rem;
  padding-right: 0.75rem;
}

.toolbar-fixed-right {
  border-left: 1px solid var(--color-border);
  background: linear-gradient(to right, var(--color-bg-tertiary), var(--color-bg-secondary));
}

/* 滚动按钮 */
.scroll-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 100%;
  min-height: 48px;
  background-color: var(--color-bg-secondary);
  border: none;
  cursor: pointer;
  color: var(--color-text-secondary);
  transition: all var(--transition-fast) ease;
  flex-shrink: 0;
  z-index: 10;
}

.scroll-btn:hover:not(:disabled) {
  background-color: var(--color-border);
  color: var(--color-text-primary);
}

.scroll-btn:disabled {
  opacity: 0.3;
  cursor: not-allowed;
}

.scroll-btn.hidden {
  opacity: 0;
  pointer-events: none;
}

.scroll-btn-left {
  border-right: 1px solid var(--color-border);
}

.scroll-btn-right {
  border-left: 1px solid var(--color-border);
}

/* 工具栏内容区域 */
.tiptap-toolbar {
  display: flex;
  flex-wrap: nowrap;
  gap: 0.5rem;
  padding: 0.5rem 0.75rem;
  align-items: center;
  overflow-x: auto;
  flex: 1;
  scrollbar-width: none; /* Firefox */
  -ms-overflow-style: none; /* IE/Edge */
}

.tiptap-toolbar::-webkit-scrollbar {
  display: none; /* Chrome/Safari/Opera */
}

.toolbar-section {
  display: flex;
  align-items: center;
  flex-shrink: 0;
}

.btn-group {
  display: flex;
  border: 1px solid var(--color-border);
  border-radius: 0.5rem;
}

.btn-group .toolbar-btn {
  border-radius: 0;
  border: none;
  border-right: 1px solid var(--color-border);
}

.btn-group .toolbar-btn:last-child {
  border-right: none;
}

.toolbar-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border: 1px solid var(--color-border);
  background-color: var(--color-bg-primary);
  border-radius: 0.5rem;
  cursor: pointer;
  transition: all var(--transition-fast) ease;
  flex-shrink: 0;
}

.toolbar-btn:hover:not(:disabled) {
  background-color: var(--color-bg-tertiary);
}

.toolbar-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.toolbar-btn.active {
  background-color: var(--color-primary);
  color: white;
  border-color: var(--color-primary);
}

/* 响应式设计 */
@media (max-width: 768px) {
  .tiptap-toolbar {
    gap: 0.25rem;
    padding: 0.25rem 0.5rem;
  }

  .scroll-btn {
    width: 24px;
  }
}
</style>

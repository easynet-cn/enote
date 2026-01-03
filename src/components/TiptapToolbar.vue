<template>
  <div class="tiptap-toolbar-wrapper">
    <!-- 左侧固定区域：Content Type -->
    <div class="toolbar-fixed toolbar-fixed-left">
      <!-- 新建笔记时的模式选择 -->
      <div v-if="isNewNote" class="toolbar-section">
        <Tooltip content="内容格式（保存后不可更改）" placement="bottom">
          <select
            :value="contentType"
            @change="
              emit('update:content-type', Number(($event.target as HTMLSelectElement).value))
            "
            class="h-8 px-2 text-sm border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent font-medium"
            :class="
              isMarkdownMode
                ? 'bg-slate-800 text-white border-slate-700'
                : 'bg-indigo-50 text-indigo-700 border-indigo-300'
            "
          >
            <option :value="ContentType.Html">富文本</option>
            <option :value="ContentType.Markdown">Markdown</option>
          </select>
        </Tooltip>
      </div>

      <!-- 模式标识（已保存的笔记） -->
      <div v-else class="toolbar-section">
        <span
          class="h-8 px-3 text-sm rounded-lg flex items-center font-medium"
          :class="isMarkdownMode ? 'bg-slate-800 text-white' : 'bg-indigo-50 text-indigo-700'"
        >
          {{ isMarkdownMode ? 'Markdown' : '富文本' }}
        </span>
      </div>
    </div>

    <!-- 左箭头 -->
    <button
      class="scroll-btn scroll-btn-left"
      @click="scrollLeft"
      :disabled="!canScrollLeft"
      :class="{ hidden: !canScrollLeft }"
    >
      <ChevronLeft class="w-5 h-5" />
    </button>

    <!-- 中间可滚动区域：编辑工具 -->
    <div class="tiptap-toolbar" ref="toolbarRef" @scroll="updateScrollState">
      <!-- 富文本模式工具栏（始终显示，非编辑模式禁用） -->
      <template v-if="!isMarkdownMode && editor">
        <!-- 标题和字体 -->
        <div class="toolbar-section">
          <Tooltip content="标题级别" placement="bottom">
            <select
              v-model="headingLevel"
              @change="setHeading"
              :disabled="!editMode"
              class="h-8 px-2 text-sm border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent disabled:opacity-50 disabled:cursor-not-allowed"
            >
              <option value="0">正文</option>
              <option value="1">标题 1</option>
              <option value="2">标题 2</option>
              <option value="3">标题 3</option>
              <option value="4">标题 4</option>
              <option value="5">标题 5</option>
              <option value="6">标题 6</option>
            </select>
          </Tooltip>

          <Tooltip content="字体" placement="bottom">
            <select
              v-model="fontFamily"
              @change="setFontFamily"
              :disabled="!editMode"
              class="h-8 px-2 text-sm border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent ml-1 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              <option value="">默认字体</option>
              <optgroup label="无衬线字体">
                <option value="Arial, sans-serif">Arial</option>
                <option value="Helvetica, sans-serif">Helvetica</option>
                <option value="Verdana, sans-serif">Verdana</option>
                <option value="Tahoma, sans-serif">Tahoma</option>
                <option value="Trebuchet MS, sans-serif">Trebuchet MS</option>
                <option value="Microsoft YaHei, sans-serif">微软雅黑</option>
                <option value="PingFang SC, sans-serif">苹方</option>
              </optgroup>
              <optgroup label="衬线字体">
                <option value="Times New Roman, serif">Times New Roman</option>
                <option value="Georgia, serif">Georgia</option>
                <option value="Palatino, serif">Palatino</option>
                <option value="SimSun, serif">宋体</option>
                <option value="KaiTi, serif">楷体</option>
                <option value="FangSong, serif">仿宋</option>
              </optgroup>
              <optgroup label="等宽字体">
                <option value="Courier New, monospace">Courier New</option>
                <option value="Consolas, monospace">Consolas</option>
                <option value="Monaco, monospace">Monaco</option>
                <option value="Source Code Pro, monospace">Source Code Pro</option>
              </optgroup>
              <optgroup label="艺术字体">
                <option value="Comic Sans MS, cursive">Comic Sans MS</option>
                <option value="Impact, fantasy">Impact</option>
                <option value="Brush Script MT, cursive">Brush Script</option>
              </optgroup>
            </select>
          </Tooltip>

          <Tooltip content="字号" placement="bottom">
            <select
              v-model="fontSize"
              @change="setFontSize"
              :disabled="!editMode"
              class="h-8 px-2 text-sm border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent ml-1 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              <option value="">默认</option>
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
            <Tooltip content="粗体" placement="bottom">
              <button
                :class="['toolbar-btn', { active: editor.isActive('bold') }]"
                @click="editor.chain().focus().toggleBold().run()"
                :disabled="!editMode || !editor.can().chain().focus().toggleBold().run()"
              >
                <Bold class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip content="斜体" placement="bottom">
              <button
                :class="['toolbar-btn', { active: editor.isActive('italic') }]"
                @click="editor.chain().focus().toggleItalic().run()"
                :disabled="!editMode || !editor.can().chain().focus().toggleItalic().run()"
              >
                <Italic class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip content="下划线" placement="bottom">
              <button
                :class="['toolbar-btn', { active: editor.isActive('underline') }]"
                @click="editor.chain().focus().toggleUnderline().run()"
                :disabled="!editMode"
              >
                <UnderlineIcon class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip content="删除线" placement="bottom">
              <button
                :class="['toolbar-btn', { active: editor.isActive('strike') }]"
                @click="editor.chain().focus().toggleStrike().run()"
                :disabled="!editMode || !editor.can().chain().focus().toggleStrike().run()"
              >
                <Strikethrough class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip content="上标" placement="bottom">
              <button
                :class="['toolbar-btn', { active: editor.isActive('superscript') }]"
                @click="editor.chain().focus().toggleSuperscript().run()"
                :disabled="!editMode"
              >
                <Superscript class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip content="下标" placement="bottom">
              <button
                :class="['toolbar-btn', { active: editor.isActive('subscript') }]"
                @click="editor.chain().focus().toggleSubscript().run()"
                :disabled="!editMode"
              >
                <Subscript class="w-4 h-4" />
              </button>
            </Tooltip>
          </div>
        </div>

        <!-- 文本对齐 -->
        <div class="toolbar-section">
          <div class="btn-group">
            <Tooltip content="左对齐" placement="bottom">
              <button
                :class="['toolbar-btn', { active: editor.isActive({ textAlign: 'left' }) }]"
                @click="editor.chain().focus().setTextAlign('left').run()"
                :disabled="!editMode"
              >
                <AlignLeft class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip content="居中" placement="bottom">
              <button
                :class="['toolbar-btn', { active: editor.isActive({ textAlign: 'center' }) }]"
                @click="editor.chain().focus().setTextAlign('center').run()"
                :disabled="!editMode"
              >
                <AlignCenter class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip content="右对齐" placement="bottom">
              <button
                :class="['toolbar-btn', { active: editor.isActive({ textAlign: 'right' }) }]"
                @click="editor.chain().focus().setTextAlign('right').run()"
                :disabled="!editMode"
              >
                <AlignRight class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip content="两端对齐" placement="bottom">
              <button
                :class="['toolbar-btn', { active: editor.isActive({ textAlign: 'justify' }) }]"
                @click="editor.chain().focus().setTextAlign('justify').run()"
                :disabled="!editMode"
              >
                <AlignJustify class="w-4 h-4" />
              </button>
            </Tooltip>
          </div>
        </div>

        <!-- 列表 -->
        <div class="toolbar-section">
          <div class="btn-group">
            <Tooltip content="无序列表" placement="bottom">
              <button
                :class="['toolbar-btn', { active: editor.isActive('bulletList') }]"
                @click="editor.chain().focus().toggleBulletList().run()"
                :disabled="!editMode"
              >
                <List class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip content="有序列表" placement="bottom">
              <button
                :class="['toolbar-btn', { active: editor.isActive('orderedList') }]"
                @click="editor.chain().focus().toggleOrderedList().run()"
                :disabled="!editMode"
              >
                <ListOrdered class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip content="任务列表" placement="bottom">
              <button
                :class="['toolbar-btn', { active: editor.isActive('taskList') }]"
                @click="editor.chain().focus().toggleTaskList().run()"
                :disabled="!editMode"
              >
                <ListChecks class="w-4 h-4" />
              </button>
            </Tooltip>
          </div>
        </div>

        <!-- 缩进 -->
        <div class="toolbar-section">
          <div class="btn-group">
            <Tooltip content="减少缩进" placement="bottom">
              <button
                class="toolbar-btn"
                @click="editor.chain().focus().outdent().run()"
                :disabled="!editMode"
              >
                <IndentDecrease class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip content="增加缩进" placement="bottom">
              <button
                class="toolbar-btn"
                @click="editor.chain().focus().indent().run()"
                :disabled="!editMode"
              >
                <IndentIncrease class="w-4 h-4" />
              </button>
            </Tooltip>
          </div>
        </div>

        <!-- 引用和代码 -->
        <div class="toolbar-section">
          <div class="btn-group">
            <Tooltip content="引用" placement="bottom">
              <button
                :class="['toolbar-btn', { active: editor.isActive('blockquote') }]"
                @click="editor.chain().focus().toggleBlockquote().run()"
                :disabled="!editMode"
              >
                <Quote class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip content="代码块" placement="bottom">
              <button
                :class="['toolbar-btn', { active: editor.isActive('codeBlock') }]"
                @click="editor.chain().focus().toggleCodeBlock().run()"
                :disabled="!editMode"
              >
                <Code2 class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip content="行内代码" placement="bottom">
              <button
                :class="['toolbar-btn', { active: editor.isActive('code') }]"
                @click="editor.chain().focus().toggleCode().run()"
                :disabled="!editMode"
              >
                <Code class="w-4 h-4" />
              </button>
            </Tooltip>
          </div>
        </div>

        <!-- 链接和图片 -->
        <div class="toolbar-section">
          <div class="btn-group">
            <Tooltip content="链接" placement="bottom">
              <button
                :class="['toolbar-btn', { active: editor.isActive('link') }]"
                @click="openLinkDialog"
                :disabled="!editMode"
              >
                <LinkIcon class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip content="取消链接" placement="bottom">
              <button
                class="toolbar-btn"
                @click="editor.chain().focus().unsetLink().run()"
                :disabled="!editMode || !editor.isActive('link')"
              >
                <Unlink class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip content="图片" placement="bottom">
              <button class="toolbar-btn" @click="openImageDialog" :disabled="!editMode">
                <ImageIcon class="w-4 h-4" />
              </button>
            </Tooltip>
          </div>
        </div>

        <!-- 表格 -->
        <div class="toolbar-section">
          <div class="btn-group">
            <Tooltip content="插入表格" placement="bottom">
              <button class="toolbar-btn" @click="insertTable" :disabled="!editMode">
                <TableIcon class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip content="添加列" placement="bottom">
              <button
                class="toolbar-btn"
                @click="editor.chain().focus().addColumnAfter().run()"
                :disabled="!editMode || !editor.can().addColumnAfter()"
              >
                <Columns3 class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip content="添加行" placement="bottom">
              <button
                class="toolbar-btn"
                @click="editor.chain().focus().addRowAfter().run()"
                :disabled="!editMode || !editor.can().addRowAfter()"
              >
                <Rows3 class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip content="删除表格" placement="bottom">
              <button
                class="toolbar-btn"
                @click="editor.chain().focus().deleteTable().run()"
                :disabled="!editMode || !editor.can().deleteTable()"
              >
                <TableOff class="w-4 h-4" />
              </button>
            </Tooltip>
          </div>
        </div>

        <!-- 其他功能 -->
        <div class="toolbar-section">
          <div class="btn-group">
            <Tooltip content="高亮" placement="bottom">
              <button
                :class="['toolbar-btn', { active: editor.isActive('highlight') }]"
                @click="editor.chain().focus().toggleHighlight().run()"
                :disabled="!editMode"
              >
                <Highlighter class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip content="分隔线" placement="bottom">
              <button
                class="toolbar-btn"
                @click="editor.chain().focus().setHorizontalRule().run()"
                :disabled="!editMode"
              >
                <Minus class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip content="清除格式" placement="bottom">
              <button
                class="toolbar-btn"
                @click="editor.chain().focus().clearNodes().unsetAllMarks().run()"
                :disabled="!editMode"
              >
                <RemoveFormatting class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip content="查找替换" placement="bottom">
              <button
                :class="['toolbar-btn', { active: searchDialogVisible }]"
                @click="toggleSearchDialog"
                :disabled="!editMode"
              >
                <Search class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip content="目录" placement="bottom">
              <button :class="['toolbar-btn', { active: tocVisible }]" @click="emit('toggle-toc')">
                <ListTree class="w-4 h-4" />
              </button>
            </Tooltip>
          </div>
        </div>

        <!-- 撤销和重做 -->
        <div class="toolbar-section">
          <div class="btn-group">
            <Tooltip content="撤销" placement="bottom">
              <button
                class="toolbar-btn"
                @click="editor.chain().focus().undo().run()"
                :disabled="!editMode || !editor.can().chain().focus().undo().run()"
              >
                <Undo2 class="w-4 h-4" />
              </button>
            </Tooltip>

            <Tooltip content="重做" placement="bottom">
              <button
                class="toolbar-btn"
                @click="editor.chain().focus().redo().run()"
                :disabled="!editMode || !editor.can().chain().focus().redo().run()"
              >
                <Redo2 class="w-4 h-4" />
              </button>
            </Tooltip>
          </div>
        </div>

        <!-- 颜色选择器 -->
        <div class="toolbar-section">
          <Tooltip content="文本颜色" placement="bottom">
            <ColorPicker
              v-model="textColor"
              @change="setTextColor"
              :predefine="predefineColors"
              :disabled="!editMode"
            />
          </Tooltip>

          <Tooltip content="背景颜色" placement="bottom">
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
          <Tooltip :content="sourceMode ? '预览模式' : 'Markdown 源码'" placement="bottom">
            <button
              :class="['toolbar-btn', { active: sourceMode }]"
              @click="emit('toggle-source-mode')"
              :disabled="!editMode"
            >
              <FileCode v-if="!sourceMode" class="w-4 h-4" />
              <Eye v-else class="w-4 h-4" />
            </button>
          </Tooltip>

          <Tooltip content="上下布局" placement="bottom">
            <button
              :class="['toolbar-btn', { active: markdownLayout === MarkdownLayout.Vertical }]"
              @click="toggleLayout(MarkdownLayout.Vertical)"
              :disabled="!editMode"
            >
              <PanelTop class="w-4 h-4" />
            </button>
          </Tooltip>

          <Tooltip content="左右布局" placement="bottom">
            <button
              :class="['toolbar-btn', { active: markdownLayout === MarkdownLayout.Horizontal }]"
              @click="toggleLayout(MarkdownLayout.Horizontal)"
              :disabled="!editMode"
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
    >
      <ChevronRight class="w-5 h-5" />
    </button>

    <!-- 右侧固定区域：操作按钮 -->
    <div class="toolbar-fixed toolbar-fixed-right">
      <div class="toolbar-actions">
        <!-- 编辑按钮（非编辑模式显示） -->
        <Tooltip v-if="!editMode" content="编辑" placement="bottom">
          <button class="action-btn action-btn-primary" @click="emit('edit')">
            <Pencil class="w-4 h-4" />
          </button>
        </Tooltip>

        <!-- 保存按钮（编辑模式显示） -->
        <Tooltip v-if="editMode" content="保存" placement="bottom">
          <button class="action-btn action-btn-success" @click="emit('save')">
            <Check class="w-4 h-4" />
          </button>
        </Tooltip>

        <!-- 取消按钮（编辑模式显示） -->
        <Tooltip v-if="editMode" content="取消" placement="bottom">
          <button class="action-btn action-btn-secondary" @click="emit('cancel')">
            <X class="w-4 h-4" />
          </button>
        </Tooltip>

        <!-- 分隔线 -->
        <div class="action-divider"></div>

        <!-- 设置按钮（编辑模式显示） -->
        <Tooltip v-if="editMode" content="设置" placement="bottom">
          <button class="action-btn action-btn-ghost" @click="emit('settings')">
            <Settings class="w-4 h-4" />
          </button>
        </Tooltip>

        <!-- 导出按钮 -->
        <Tooltip content="导出" placement="bottom">
          <button class="action-btn action-btn-ghost" @click="emit('export')">
            <Download class="w-4 h-4" />
          </button>
        </Tooltip>

        <!-- 历史记录按钮 -->
        <Tooltip content="历史记录" placement="bottom">
          <button class="action-btn action-btn-ghost" @click="emit('history')">
            <History class="w-4 h-4" />
          </button>
        </Tooltip>

        <!-- 删除按钮 -->
        <Tooltip content="删除" placement="bottom">
          <button class="action-btn action-btn-danger" @click="emit('delete')">
            <Trash2 class="w-4 h-4" />
          </button>
        </Tooltip>
      </div>
    </div>
  </div>

  <!-- 链接弹窗 -->
  <Dialog v-model="linkDialogVisible" title="插入链接" :width="400">
    <div class="space-y-4">
      <div>
        <label class="block text-sm font-medium text-slate-700 mb-1">链接地址</label>
        <input
          v-model="linkUrl"
          type="text"
          placeholder="https://example.com"
          class="w-full px-3 py-2 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
        />
      </div>
    </div>
    <template #footer>
      <div class="flex justify-end gap-2">
        <button
          @click="linkDialogVisible = false"
          class="px-4 py-2 text-slate-700 bg-white border border-slate-300 rounded-lg hover:bg-slate-50 transition-colors"
        >
          取消
        </button>
        <button
          @click="setLink"
          class="px-4 py-2 text-white bg-indigo-600 rounded-lg hover:bg-indigo-700 transition-colors"
        >
          确定
        </button>
      </div>
    </template>
  </Dialog>

  <!-- 图片弹窗 -->
  <Dialog v-model="imageDialogVisible" title="插入图片" :width="400">
    <div class="space-y-4">
      <div>
        <label class="block text-sm font-medium text-slate-700 mb-1">图片地址</label>
        <input
          v-model="imageUrl"
          type="text"
          placeholder="https://example.com/image.png"
          class="w-full px-3 py-2 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
        />
      </div>
      <div class="text-center text-slate-500 text-sm">- 或 -</div>
      <div>
        <label class="block text-sm font-medium text-slate-700 mb-1">上传图片</label>
        <input
          type="file"
          accept="image/*"
          @change="handleImageUpload"
          class="w-full px-3 py-2 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
        />
      </div>
    </div>
    <template #footer>
      <div class="flex justify-end gap-2">
        <button
          @click="imageDialogVisible = false"
          class="px-4 py-2 text-slate-700 bg-white border border-slate-300 rounded-lg hover:bg-slate-50 transition-colors"
        >
          取消
        </button>
        <button
          @click="insertImage"
          class="px-4 py-2 text-white bg-indigo-600 rounded-lg hover:bg-indigo-700 transition-colors"
        >
          确定
        </button>
      </div>
    </template>
  </Dialog>

  <!-- 查找替换弹窗 -->
  <Dialog v-model="searchDialogVisible" title="查找替换" :width="400">
    <div class="space-y-4">
      <div>
        <label class="block text-sm font-medium text-slate-700 mb-1">查找</label>
        <input
          v-model="searchTerm"
          type="text"
          placeholder="输入要查找的文本"
          @input="handleSearchInput"
          @keydown.enter="findNext"
          class="w-full px-3 py-2 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
        />
      </div>
      <div>
        <label class="block text-sm font-medium text-slate-700 mb-1">替换为</label>
        <input
          v-model="replaceTerm"
          type="text"
          placeholder="输入替换文本"
          @input="handleReplaceInput"
          class="w-full px-3 py-2 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
        />
      </div>
      <div class="flex items-center gap-4">
        <label class="flex items-center gap-2 text-sm text-slate-600 cursor-pointer">
          <input
            type="checkbox"
            v-model="caseSensitive"
            @change="handleCaseSensitiveChange"
            class="w-4 h-4 rounded border-slate-300 text-indigo-600 focus:ring-indigo-500"
          />
          区分大小写
        </label>
        <span v-if="searchResultCount > 0" class="text-sm text-slate-500">
          {{ currentSearchIndex + 1 }} / {{ searchResultCount }}
        </span>
        <span v-else-if="searchTerm" class="text-sm text-slate-400"> 无结果 </span>
      </div>
    </div>
    <template #footer>
      <div class="flex justify-between">
        <div class="flex gap-2">
          <button
            @click="findPrevious"
            :disabled="searchResultCount === 0"
            class="px-3 py-2 text-slate-700 bg-white border border-slate-300 rounded-lg hover:bg-slate-50 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <ChevronUp class="w-4 h-4" />
          </button>
          <button
            @click="findNext"
            :disabled="searchResultCount === 0"
            class="px-3 py-2 text-slate-700 bg-white border border-slate-300 rounded-lg hover:bg-slate-50 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <ChevronDown class="w-4 h-4" />
          </button>
        </div>
        <div class="flex gap-2">
          <button
            @click="replaceOne"
            :disabled="searchResultCount === 0"
            class="px-4 py-2 text-slate-700 bg-white border border-slate-300 rounded-lg hover:bg-slate-50 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            替换
          </button>
          <button
            @click="replaceAll"
            :disabled="searchResultCount === 0"
            class="px-4 py-2 text-white bg-indigo-600 rounded-lg hover:bg-indigo-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            全部替换
          </button>
        </div>
      </div>
    </template>
  </Dialog>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted, computed } from 'vue'
import type { Editor } from '@tiptap/vue-3'
import { ContentType, MarkdownLayout } from '../types'
import { Tooltip, ColorPicker, Dialog } from './ui'
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
  ChevronUp,
  ChevronDown,
  FileCode,
  Eye,
  Pencil,
  Check,
  X,
  Trash2,
  Settings,
  History,
  Download,
  PanelTop,
  PanelLeft,
  Subscript,
  Superscript,
  IndentDecrease,
  IndentIncrease,
  Search,
  ListTree,
} from 'lucide-vue-next'

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

// 链接弹窗
const linkDialogVisible = ref(false)
const linkUrl = ref('')

// 图片弹窗
const imageDialogVisible = ref(false)
const imageUrl = ref('')

// 查找替换
const searchDialogVisible = ref(false)
const searchTerm = ref('')
const replaceTerm = ref('')
const caseSensitive = ref(false)
const searchResultCount = ref(0)
const currentSearchIndex = ref(0)

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
  if (!props.editor) return
  const previousUrl = props.editor.getAttributes('link').href
  linkUrl.value = previousUrl || ''
  linkDialogVisible.value = true
}

// 设置链接
const setLink = () => {
  if (!props.editor) return
  if (linkUrl.value) {
    props.editor.chain().focus().extendMarkRange('link').setLink({ href: linkUrl.value }).run()
  }
  linkDialogVisible.value = false
  linkUrl.value = ''
}

// 打开图片弹窗
const openImageDialog = () => {
  imageUrl.value = ''
  imageDialogVisible.value = true
}

// 处理图片上传
const handleImageUpload = (event: Event) => {
  const target = event.target as HTMLInputElement
  const file = target.files?.[0]
  if (file) {
    const reader = new FileReader()
    reader.onload = (e) => {
      imageUrl.value = e.target?.result as string
    }
    reader.readAsDataURL(file)
  }
}

// 插入图片
const insertImage = () => {
  if (!props.editor) return
  if (imageUrl.value) {
    props.editor.chain().focus().setImage({ src: imageUrl.value }).run()
  }
  imageDialogVisible.value = false
  imageUrl.value = ''
}

// 插入表格
const insertTable = () => {
  if (!props.editor) return
  props.editor.chain().focus().insertTable({ rows: 3, cols: 3, withHeaderRow: true }).run()
}

// 查找替换相关函数
const toggleSearchDialog = () => {
  searchDialogVisible.value = !searchDialogVisible.value
  if (!searchDialogVisible.value && props.editor) {
    props.editor.commands.clearSearch()
    searchTerm.value = ''
    replaceTerm.value = ''
    searchResultCount.value = 0
    currentSearchIndex.value = 0
  }
}

const handleSearchInput = () => {
  if (!props.editor) return
  props.editor.commands.setSearchTerm(searchTerm.value)
  updateSearchState()
}

const handleReplaceInput = () => {
  if (!props.editor) return
  props.editor.commands.setReplaceTerm(replaceTerm.value)
}

const handleCaseSensitiveChange = () => {
  if (!props.editor) return
  props.editor.commands.setCaseSensitive(caseSensitive.value)
  updateSearchState()
}

const updateSearchState = () => {
  if (!props.editor) return
  const storage = (props.editor.storage as unknown as Record<string, unknown>).searchAndReplace as
    | { results: unknown[]; currentIndex: number }
    | undefined
  if (storage) {
    searchResultCount.value = storage.results.length
    currentSearchIndex.value = storage.currentIndex
  }
}

const findNext = () => {
  if (!props.editor) return
  props.editor.commands.nextSearchResult()
  updateSearchState()
}

const findPrevious = () => {
  if (!props.editor) return
  props.editor.commands.previousSearchResult()
  updateSearchState()
}

const replaceOne = () => {
  if (!props.editor) return
  props.editor.commands.replaceCurrentResult()
  setTimeout(updateSearchState, 10)
}

const replaceAll = () => {
  if (!props.editor) return
  props.editor.commands.replaceAllResults()
  setTimeout(updateSearchState, 10)
}

// 监听编辑器活动状态更新选择框
watch(
  () => props.editor?.isActive('heading'),
  (isActive) => {
    if (!props.editor) return
    if (!isActive) {
      headingLevel.value = '0'
    } else {
      // 检查具体是哪个级别的标题
      for (let i = 1; i <= 6; i++) {
        if (props.editor.isActive('heading', { level: i })) {
          headingLevel.value = i.toString()
          break
        }
      }
    }
  },
)

// 监听字体变化
watch(
  () => props.editor?.getAttributes('textStyle')?.fontFamily,
  (font) => {
    fontFamily.value = font || ''
  },
)

// 监听字号变化
watch(
  () => props.editor?.getAttributes('textStyle')?.fontSize,
  (size) => {
    fontSize.value = size || ''
  },
)

// 监听文本颜色变化
watch(
  () => props.editor?.getAttributes('textStyle')?.color,
  (color) => {
    if (color) {
      textColor.value = color
    }
  },
)

// 监听高亮颜色变化
watch(
  () => props.editor?.getAttributes('highlight')?.color,
  (color) => {
    if (color) {
      highlightColor.value = color
    }
  },
)
</script>

<style scoped>
/* 工具栏外层容器 */
.tiptap-toolbar-wrapper {
  display: flex;
  align-items: center;
  border-bottom: 1px solid #e2e8f0;
  background-color: #f8fafc;
  position: relative;
}

/* 固定区域 */
.toolbar-fixed {
  display: flex;
  align-items: center;
  padding: 0.5rem;
  background-color: #f8fafc;
  flex-shrink: 0;
  z-index: 5;
}

.toolbar-fixed-left {
  border-right: 1px solid #e2e8f0;
  background: linear-gradient(to left, #f1f5f9, #f8fafc);
  padding-left: 0.75rem;
  padding-right: 0.75rem;
}

.toolbar-fixed-right {
  border-left: 1px solid #e2e8f0;
  background: linear-gradient(to right, #f1f5f9, #f8fafc);
}

/* 操作按钮区域 */
.toolbar-actions {
  display: flex;
  align-items: center;
  gap: 6px;
}

/* 操作按钮基础样式 */
.action-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  height: 32px;
  padding: 0 12px;
  border-radius: 8px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  border: none;
  outline: none;
}

/* 主要按钮 - 编辑 */
.action-btn-primary {
  background: #4f46e5;
  color: white;
  padding: 0 10px;
  box-shadow: 0 1px 2px rgba(79, 70, 229, 0.3);
}

.action-btn-primary:hover {
  background: #4338ca;
  box-shadow: 0 2px 4px rgba(79, 70, 229, 0.4);
  transform: translateY(-1px);
}

/* 成功按钮 - 保存 */
.action-btn-success {
  background: #4f46e5;
  color: white;
  padding: 0 10px;
  box-shadow: 0 1px 2px rgba(79, 70, 229, 0.3);
}

.action-btn-success:hover {
  background: #4338ca;
  box-shadow: 0 2px 4px rgba(79, 70, 229, 0.4);
  transform: translateY(-1px);
}

/* 次要按钮 - 取消 */
.action-btn-secondary {
  background: #e2e8f0;
  color: #334155;
  padding: 0 8px;
}

.action-btn-secondary:hover {
  background: #cbd5e1;
}

/* 幽灵按钮 - 设置/历史 */
.action-btn-ghost {
  background: transparent;
  color: #64748b;
  padding: 0 8px;
}

.action-btn-ghost:hover {
  background: #e2e8f0;
  color: #334155;
}

/* 危险按钮 - 删除 */
.action-btn-danger {
  background: transparent;
  color: #94a3b8;
  padding: 0 8px;
}

.action-btn-danger:hover {
  background: #fee2e2;
  color: #dc2626;
}

/* 操作分隔线 */
.action-divider {
  width: 1px;
  height: 20px;
  background: #cbd5e1;
  margin: 0 4px;
}

/* 滚动按钮 */
.scroll-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 100%;
  min-height: 48px;
  background-color: #f8fafc;
  border: none;
  cursor: pointer;
  color: #64748b;
  transition: all 0.15s ease;
  flex-shrink: 0;
  z-index: 10;
}

.scroll-btn:hover:not(:disabled) {
  background-color: #e2e8f0;
  color: #334155;
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
  border-right: 1px solid #e2e8f0;
}

.scroll-btn-right {
  border-left: 1px solid #e2e8f0;
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
  border: 1px solid #e2e8f0;
  border-radius: 0.5rem;
}

.btn-group .toolbar-btn {
  border-radius: 0;
  border: none;
  border-right: 1px solid #e2e8f0;
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
  border: 1px solid #e2e8f0;
  background-color: white;
  border-radius: 0.5rem;
  cursor: pointer;
  transition: all 0.15s ease;
  flex-shrink: 0;
}

.toolbar-btn:hover:not(:disabled) {
  background-color: #f1f5f9;
}

.toolbar-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.toolbar-btn.active {
  background-color: #4f46e5;
  color: white;
  border-color: #4f46e5;
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

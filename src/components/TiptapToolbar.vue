<template>
    <div class="tiptap-toolbar">
        <!-- 文本样式 -->
        <div class="toolbar-section">
            <el-select v-model="headingLevel" @change="setHeading" placeholder="正文" size="small" class="toolbar-select">
                <el-option label="正文" value="0" />
                <el-option label="标题 1" value="1" />
                <el-option label="标题 2" value="2" />
                <el-option label="标题 3" value="3" />
            </el-select>
        </div>

        <!-- 字体样式 -->
        <div class="toolbar-section">
            <el-button-group>
                <el-tooltip content="粗体" placement="bottom">
                    <el-button size="small" :type="editor.isActive('bold') ? 'primary' : 'default'"
                        @click="editor.chain().focus().toggleBold().run()"
                        :disabled="!editor.can().chain().focus().toggleBold().run()">
                        <i class="ri-bold"></i>
                    </el-button>
                </el-tooltip>

                <el-tooltip content="斜体" placement="bottom">
                    <el-button size="small" :type="editor.isActive('italic') ? 'primary' : 'default'"
                        @click="editor.chain().focus().toggleItalic().run()"
                        :disabled="!editor.can().chain().focus().toggleItalic().run()">
                        <i class="ri-italic"></i>
                    </el-button>
                </el-tooltip>

                <el-tooltip content="下划线" placement="bottom">
                    <el-button size="small" :type="editor.isActive('underline') ? 'primary' : 'default'"
                        @click="editor.chain().focus().toggleUnderline().run()">
                        <i class="ri-underline"></i>
                    </el-button>
                </el-tooltip>

                <el-tooltip content="删除线" placement="bottom">
                    <el-button size="small" :type="editor.isActive('strike') ? 'primary' : 'default'"
                        @click="editor.chain().focus().toggleStrike().run()"
                        :disabled="!editor.can().chain().focus().toggleStrike().run()">
                        <i class="ri-strikethrough"></i>
                    </el-button>
                </el-tooltip>
            </el-button-group>
        </div>

        <!-- 文本对齐 -->
        <div class="toolbar-section">
            <el-button-group>
                <el-tooltip content="左对齐" placement="bottom">
                    <el-button size="small" :type="editor.isActive({ textAlign: 'left' }) ? 'primary' : 'default'"
                        @click="editor.chain().focus().setTextAlign('left').run()">
                        <i class="ri-align-left"></i>
                    </el-button>
                </el-tooltip>

                <el-tooltip content="居中" placement="bottom">
                    <el-button size="small" :type="editor.isActive({ textAlign: 'center' }) ? 'primary' : 'default'"
                        @click="editor.chain().focus().setTextAlign('center').run()">
                        <i class="ri-align-center"></i>
                    </el-button>
                </el-tooltip>

                <el-tooltip content="右对齐" placement="bottom">
                    <el-button size="small" :type="editor.isActive({ textAlign: 'right' }) ? 'primary' : 'default'"
                        @click="editor.chain().focus().setTextAlign('right').run()">
                        <i class="ri-align-right"></i>
                    </el-button>
                </el-tooltip>

                <el-tooltip content="两端对齐" placement="bottom">
                    <el-button size="small" :type="editor.isActive({ textAlign: 'justify' }) ? 'primary' : 'default'"
                        @click="editor.chain().focus().setTextAlign('justify').run()">
                        <i class="ri-align-justify"></i>
                    </el-button>
                </el-tooltip>
            </el-button-group>
        </div>

        <!-- 列表 -->
        <div class="toolbar-section">
            <el-button-group>
                <el-tooltip content="无序列表" placement="bottom">
                    <el-button size="small" :type="editor.isActive('bulletList') ? 'primary' : 'default'"
                        @click="editor.chain().focus().toggleBulletList().run()">
                        <i class="ri-list-unordered"></i>
                    </el-button>
                </el-tooltip>

                <el-tooltip content="有序列表" placement="bottom">
                    <el-button size="small" :type="editor.isActive('orderedList') ? 'primary' : 'default'"
                        @click="editor.chain().focus().toggleOrderedList().run()">
                        <i class="ri-list-ordered"></i>
                    </el-button>
                </el-tooltip>

                <el-tooltip content="任务列表" placement="bottom">
                    <el-button size="small" :type="editor.isActive('taskList') ? 'primary' : 'default'"
                        @click="editor.chain().focus().toggleTaskList().run()">
                        <i class="ri-list-check-2"></i>
                    </el-button>
                </el-tooltip>
            </el-button-group>
        </div>

        <!-- 引用和代码 -->
        <div class="toolbar-section">
            <el-button-group>
                <el-tooltip content="引用" placement="bottom">
                    <el-button size="small" :type="editor.isActive('blockquote') ? 'primary' : 'default'"
                        @click="editor.chain().focus().toggleBlockquote().run()">
                        <i class="ri-double-quotes-l"></i>
                    </el-button>
                </el-tooltip>

                <el-tooltip content="代码块" placement="bottom">
                    <el-button size="small" :type="editor.isActive('codeBlock') ? 'primary' : 'default'"
                        @click="editor.chain().focus().toggleCodeBlock().run()">
                        <i class="ri-code-box-line"></i>
                    </el-button>
                </el-tooltip>

                <el-tooltip content="行内代码" placement="bottom">
                    <el-button size="small" :type="editor.isActive('code') ? 'primary' : 'default'"
                        @click="editor.chain().focus().toggleCode().run()">
                        <i class="ri-code-s-slash-line"></i>
                    </el-button>
                </el-tooltip>
            </el-button-group>
        </div>

        <!-- 其他功能 -->
        <div class="toolbar-section">
            <el-button-group>
                <el-tooltip content="高亮" placement="bottom">
                    <el-button size="small" :type="editor.isActive('highlight') ? 'primary' : 'default'"
                        @click="editor.chain().focus().toggleHighlight().run()">
                        <i class="ri-mark-pen-line"></i>
                    </el-button>
                </el-tooltip>

                <el-tooltip content="分隔线" placement="bottom">
                    <el-button size="small" @click="editor.chain().focus().setHorizontalRule().run()">
                        <i class="ri-separator"></i>
                    </el-button>
                </el-tooltip>

                <el-tooltip content="清除格式" placement="bottom">
                    <el-button size="small" @click="editor.chain().focus().clearNodes().unsetAllMarks().run()">
                        <i class="ri-format-clear"></i>
                    </el-button>
                </el-tooltip>
            </el-button-group>
        </div>

        <!-- 撤销和重做 -->
        <div class="toolbar-section">
            <el-button-group>
                <el-tooltip content="撤销" placement="bottom">
                    <el-button size="small" @click="editor.chain().focus().undo().run()"
                        :disabled="!editor.can().chain().focus().undo().run()">
                        <i class="ri-arrow-go-back-line"></i>
                    </el-button>
                </el-tooltip>

                <el-tooltip content="重做" placement="bottom">
                    <el-button size="small" @click="editor.chain().focus().redo().run()"
                        :disabled="!editor.can().chain().focus().redo().run()">
                        <i class="ri-arrow-go-forward-line"></i>
                    </el-button>
                </el-tooltip>
            </el-button-group>
        </div>

        <!-- 颜色选择器 -->
        <div class="toolbar-section">
            <el-tooltip content="文本颜色" placement="bottom">
                <el-color-picker v-model="textColor" @change="setTextColor" size="small" :predefine="predefineColors" />
            </el-tooltip>

            <el-tooltip content="背景颜色" placement="bottom">
                <el-color-picker v-model="highlightColor" @change="setHighlightColor" size="small"
                    :predefine="predefineColors" show-alpha />
            </el-tooltip>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import type { Editor } from '@tiptap/vue-3'

interface Props {
    editor: Editor
}

const props = defineProps<Props>()

const headingLevel = ref('0')
const textColor = ref('#000000')
const highlightColor = ref('#FFFF00')

// 预定义颜色
const predefineColors = ref([
    '#000000',
    '#ffffff',
    '#ff4500',
    '#ff8c00',
    '#ffd700',
    '#90ee90',
    '#00ced1',
    '#1e90ff',
    '#c71585',
    'rgba(255, 69, 0, 0.68)',
    'rgb(255, 120, 0)',
    'hsv(51, 100, 98)',
    'hsva(120, 40, 94, 0.5)',
    'hsl(181, 100%, 37%)',
    'hsla(209, 100%, 56%, 0.73)'
])

// 设置标题
const setHeading = () => {
    const level = parseInt(headingLevel.value)
    if (level === 0) {
        props.editor.chain().focus().setParagraph().run()
    } else {
        props.editor.chain().focus().toggleHeading({ level: level as any }).run()
    }
}

// 设置文本颜色
const setTextColor = (color: string) => {
    props.editor.chain().focus().setColor(color).run()
}

// 设置背景颜色
const setHighlightColor = (color: string) => {
    props.editor.chain().focus().toggleHighlight({ color }).run()
}

// 监听编辑器活动状态更新选择框
watch(
    () => props.editor?.isActive('heading'),
    (isActive) => {
        if (!isActive) {
            headingLevel.value = '0'
        }
    }
)

// 监听文本颜色变化
watch(
    () => props.editor?.getAttributes('textStyle')?.color,
    (color) => {
        if (color) {
            textColor.value = color
        }
    }
)

// 监听高亮颜色变化
watch(
    () => props.editor?.getAttributes('highlight')?.color,
    (color) => {
        if (color) {
            highlightColor.value = color
        }
    }
)
</script>

<style scoped>
.tiptap-toolbar {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    padding: 0.75rem;
    border-bottom: 1px solid #e5e7eb;
    background-color: #f9fafb;
    align-items: center;
}

.toolbar-section {
    display: flex;
    align-items: center;
}

.toolbar-select {
    width: 100px;
}

/* Remix Icon 样式调整 */
.toolbar-section i {
    font-size: 16px;
}

/* 调整按钮组间距 */
:deep(.el-button-group) {
    display: flex;
}

:deep(.el-button-group .el-button) {
    border-radius: 0;
}

:deep(.el-button-group .el-button:first-child) {
    border-top-left-radius: 4px;
    border-bottom-left-radius: 4px;
}

:deep(.el-button-group .el-button:last-child) {
    border-top-right-radius: 4px;
    border-bottom-right-radius: 4px;
}

/* 颜色选择器样式调整 */
:deep(.el-color-picker) {
    margin-right: 0.5rem;
}

:deep(.el-color-picker:last-child) {
    margin-right: 0;
}
</style>
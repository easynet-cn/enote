<template>
    <el-dialog v-model="visible" title="历史记录" fullscreen @open="$emit('open')">
        <div class="h-[88vh] overflow-hidden flex flex-col">
            <el-table :data="data" empty-text="没有数据">
                <el-table-column prop="id" label="ID" width="60" />
                <el-table-column prop="oldContent" label="旧内容" min-width="200">
                    <template #default="scope">
                        <div class="text-sm text-gray-500 mb-2 line-clamp-2" v-html="scope.row.oldContent"></div>
                    </template>
                </el-table-column>
                <el-table-column prop="newContent" label="新内容" min-width="200">
                    <template #default="scope">
                        <div class="text-sm text-gray-500 mb-2 line-clamp-2" v-html="scope.row.newContent"></div>
                    </template>
                </el-table-column>
                <el-table-column prop="extra" label="其他信息" width="120" />
                <el-table-column prop="operateType" label="操作类型" width="80" />
                <el-table-column prop="operateTime" label="操作时间" width="170" />
                <el-table-column label="操作" width="80" fixed="right">
                    <template #default="scope">
                        <el-button type="primary" size="small" @click="handleView(scope.row)">
                            查看
                        </el-button>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <template #footer>
            <div class="flex justify-end">
                <el-pagination v-model:current-page="currentPage" v-model:page-size="pageSize"
                    :page-sizes="[20, 50, 100, 200]" :default-page-size="50" layout="total, sizes, prev, pager, next"
                    :total="total" @size-change="handleSizeChange" @current-change="handleCurrentChange" />
            </div>
        </template>
    </el-dialog>

    <!-- 内容查看对话框 -->
    <el-dialog v-model="viewVisible" title="内容查看" width="90%" :fullscreen="false">
        <div class="h-[70vh] overflow-hidden flex">
            <!-- 旧内容区域 -->
            <div class="flex-1 border-r border-gray-200 pr-4">
                <div class="text-lg font-semibold mb-4">旧内容</div>
                <div class="h-full overflow-auto bg-gray-50 p-4 rounded border">
                    <TipTapEditor :model-value="viewOldContent" :editable="false" :show-toolbar="false"
                        class="h-full" />
                </div>
            </div>

            <!-- 新内容区域 -->
            <div class="flex-1 pl-4">
                <div class="text-lg font-semibold mb-4">新内容</div>
                <div class="h-full overflow-auto bg-green-50 p-4 rounded border">
                    <TipTapEditor :model-value="viewNewContent" :editable="false" :show-toolbar="false"
                        class="h-full" />
                </div>
            </div>
        </div>
    </el-dialog>
</template>
<script setup lang="ts">
import { ref } from 'vue';
import { NoteHistory } from '../types';
import TipTapEditor from './TipTapEditor.vue';

const visible = defineModel<boolean>("visible");
const data = defineModel<NoteHistory[]>("data");
const currentPage = defineModel<number>("currentPage");
const pageSize = defineModel<number>("pageSize");
const total = defineModel<number>("total");

// 查看对话框相关状态
const viewVisible = ref(false);
const viewOldContent = ref('');
const viewNewContent = ref('');

const emit = defineEmits<{
    sizeChange: [pageSize: number]
    currentChange: [currentPage: number]
    open: []
}>()

const handleSizeChange = (val: number) => {
    emit("sizeChange", val);
}

const handleCurrentChange = (val: number) => {
    emit("currentChange", val);
}

const handleView = (row: NoteHistory) => {
    const oldContent = row.oldContent || '';
    const newContent = row.newContent || '';

    // 直接使用原始HTML内容，TipTap编辑器会处理显示
    viewOldContent.value = oldContent;
    viewNewContent.value = newContent;
    viewVisible.value = true;
}
</script>
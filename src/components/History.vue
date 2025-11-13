<template>
    <el-dialog v-model="visible" title="历史记录" fullscreen @open="$emit('open')">
        <div class="h-[88vh] overflow-hidden flex flex-col">
            <el-table :data="data" show-overflow-tooltip="{raw-content:true}" empty-text="没有数据">
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
                        <el-button type="primary" size="small" @click="handlerDiff(scope.$index, scope.row)">
                            对比
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
</template>
<script setup lang="ts">
import { NoteHistory } from '../types';

const visible = defineModel<boolean>("visible");
const data = defineModel<NoteHistory[]>("data");
const currentPage = defineModel<number>("currentPage");
const pageSize = defineModel<number>("pageSize");
const total = defineModel<number>("total");

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

const handlerDiff = (index: number, row: History) => {
}
</script>
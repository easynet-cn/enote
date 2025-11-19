<template>
  <el-aside>
    <el-row class="p-2 border-b border-gray-200" justify="center">
      <el-col :span="12">
        <el-button
          type="success"
          size="large"
          @click="$emit('createNewNote')"
          :icon="Plus"
          round
        >
          创建新笔记
        </el-button>
      </el-col>
    </el-row>
    <el-row class="p-4 border-b border-gray-200">
      <el-col :span="22">
        <h2 class="text-sm font-semibold text-gray-500 mb-3">笔记本</h2>
      </el-col>
      <el-col :span="2">
        <div class="toolbar">
          <el-dropdown @command="handleNotebookCommand">
            <el-icon class="mt-1">
              <icon-menu />
            </el-icon>
            <template #dropdown>
              <el-dropdown-menu>
                <el-dropdown-item command="create">
                  <el-icon>
                    <plus />
                  </el-icon>
                  <span>添加</span>
                </el-dropdown-item>
                <el-dropdown-item
                  v-if="showNotebookEditAndDelete"
                  command="edit"
                >
                  <el-icon>
                    <edit />
                  </el-icon>
                  <span>编辑</span>
                </el-dropdown-item>
                <el-dropdown-item
                  v-if="showNotebookEditAndDelete"
                  command="delete"
                >
                  <el-icon>
                    <delete />
                  </el-icon>
                  <span>删除</span>
                </el-dropdown-item>
              </el-dropdown-menu>
            </template>
          </el-dropdown>
        </div>
      </el-col>
      <el-col :span="24">
        <ul>
          <li
            v-for="notebook in notebooks"
            :key="notebook.id"
            class="sidebar-item"
            :class="{ active: activeNotebook === notebook.id }"
            @click="$emit('setActiveNotebook', notebook.id)"
          >
            <div class="flex items-center">
              <span class="mr-3">{{ notebook.icon }}</span>
              <span class="flex-1">{{ notebook.name }}</span>
              <span class="text-xs text-gray-400">{{ notebook.count }}</span>
            </div>
          </li>
        </ul>
      </el-col>
    </el-row>
    <el-row class="p-4">
      <el-col :span="22">
        <h2 class="text-sm font-semibold text-gray-500 mb-3">标签</h2>
      </el-col>
      <el-col :span="2">
        <div class="toolbar">
          <el-dropdown @command="handleTagCommand">
            <el-icon class="mt-1">
              <icon-menu />
            </el-icon>
            <template #dropdown>
              <el-dropdown-menu>
                <el-dropdown-item command="create">
                  <el-icon>
                    <plus />
                  </el-icon>
                  <span>添加</span>
                </el-dropdown-item>
                <el-dropdown-item v-if="showTagEditAndDelete" command="edit">
                  <el-icon>
                    <edit />
                  </el-icon>
                  <span>编辑</span>
                </el-dropdown-item>
                <el-dropdown-item v-if="showTagEditAndDelete" command="delete">
                  <el-icon>
                    <delete />
                  </el-icon>
                  <span>删除</span>
                </el-dropdown-item>
              </el-dropdown-menu>
            </template>
          </el-dropdown>
        </div>
      </el-col>
      <el-col :span="24">
        <ul class="space-y-1">
          <li
            v-for="tag in tags"
            :key="tag.id"
            :class="['sidebar-item', { active: activeTag === tag.id }]"
            @click="$emit('setActiveTag', tag.id)"
          >
            <div class="flex items-center">
              <span :class="['mr-3', tag.cls]">●</span>
              <span>{{ tag.name }}</span>
            </div>
          </li>
        </ul>
      </el-col>
    </el-row>
  </el-aside>
  <el-dialog
    v-model="notebookDialog"
    :title="notebookDialogTitle"
    width="500"
    align-center
  >
    <el-form
      ref="notebookFormRef"
      :model="notebookForm"
      :rules="notebookRules"
      label-width="auto"
    >
      <el-form-item label="名称" prop="name">
        <el-input v-model="notebookForm.name" />
      </el-form-item>
      <el-form-item label="描述" prop="description">
        <el-input v-model="notebookForm.description" />
      </el-form-item>
      <el-form-item label="图标" prop="icon">
        <el-input v-model="notebookForm.icon" />
      </el-form-item>
      <el-form-item label="样式" prop="cls">
        <el-input v-model="notebookForm.cls" />
      </el-form-item>
      <el-form-item label="排序" prop="sortOrder">
        <el-input-number
          v-model="notebookForm.sortOrder"
          :min="0"
          :precision="0"
        />
      </el-form-item>
    </el-form>
    <template #footer>
      <div class="dialog-footer">
        <el-button @click="closeNotebookDialog">取消</el-button>
        <el-button type="primary" @click="submitNotebookForm(notebookFormRef)">
          保存
        </el-button>
      </div>
    </template>
  </el-dialog>
  <el-dialog
    v-model="tagDialog"
    :title="tagDialogTitle"
    width="500"
    align-center
  >
    <el-form
      ref="tagFormRef"
      :model="tagForm"
      :rules="tagRules"
      label-width="auto"
    >
      <el-form-item label="名称" prop="name">
        <el-input v-model="tagForm.name" />
      </el-form-item>
      <el-form-item label="图标" prop="icon">
        <el-input v-model="tagForm.icon" />
      </el-form-item>
      <el-form-item label="样式" prop="cls">
        <el-input v-model="tagForm.cls" />
      </el-form-item>
      <el-form-item label="排序" prop="sortOrder">
        <el-input-number v-model="tagForm.sortOrder" :min="0" :precision="0" />
      </el-form-item>
    </el-form>
    <template #footer>
      <div class="dialog-footer">
        <el-button @click="closeTagDialog">取消</el-button>
        <el-button type="primary" @click="submitTagForm(tagFormRef)">
          保存
        </el-button>
      </div>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { Plus, Edit, Delete, Menu as IconMenu } from "@element-plus/icons-vue";
import type { FormInstance, FormRules } from "element-plus";
import type { ShowNotebook, ShowTag } from "../types";
import { computed, reactive, ref } from "vue";

interface NotebookForm {
  id: string;
  name: string;
  description: string;
  icon: string;
  cls: string;
  sortOrder: number;
}

interface TagForm {
  id: string;
  name: string;
  icon: string;
  cls: string;
  sortOrder: number;
}

const notebookFormRef = ref<FormInstance>();
const notebookForm = reactive<NotebookForm>({
  id: "",
  name: "",
  description: "",
  icon: "",
  cls: "",
  sortOrder: 0,
});
const notebookRules = reactive<FormRules<NotebookForm>>({
  name: [
    {
      required: true,
      message: "请输入名称",
      trigger: "blur",
    },
  ],
});

const tagFormRef = ref<FormInstance>();
const tagForm = reactive<TagForm>({
  id: "",
  name: "",
  icon: "",
  cls: "",
  sortOrder: 0,
});
const tagRules = reactive<FormRules<TagForm>>({
  name: [
    {
      required: true,
      message: "请输入名称",
      trigger: "blur",
    },
  ],
});

const notebookDialog = ref(false);
const notebookDialogTitle = ref("添加笔记本");
const tagDialog = ref(false);
const tagDialogTitle = ref("添加标签");

const props = defineProps<{
  notebooks: ShowNotebook[];
  tags: ShowTag[];
  activeNotebook: string;
  activeTag: string;
}>();

const emit = defineEmits<{
  setActiveNotebook: [id: string];
  createNewNote: [];
  saveNotebook: [notebook: ShowNotebook];
  deleteNotebook: [id: string];
  setActiveTag: [id: string];
  saveTag: [tag: ShowTag];
  deleteTag: [id: string];
}>();

const showNotebookEditAndDelete = computed(() => {
  return (
    props.notebooks.length > 0 &&
    props.activeNotebook !== "" &&
    props.activeNotebook !== "0"
  );
});
const showTagEditAndDelete = computed(() => {
  return props.tags.length > 0 && props.activeTag !== "";
});

const submitNotebookForm = async (form: FormInstance | undefined) => {
  if (!form) {
    return;
  }

  await form.validate((valid) => {
    if (valid) {
      emit("saveNotebook", {
        id: notebookForm.id,
        name: notebookForm.name,
        description: notebookForm.description,
        icon: notebookForm.icon,
        cls: notebookForm.cls,
        sortOrder: notebookForm.sortOrder,
      });

      notebookDialog.value = false;
    }
  });
};

const submitTagForm = async (form: FormInstance | undefined) => {
  if (!form) {
    return;
  }

  await form.validate((valid) => {
    if (valid) {
      emit("saveTag", {
        id: tagForm.id,
        name: tagForm.name,
        icon: tagForm.icon,
        cls: tagForm.cls,
        sortOrder: tagForm.sortOrder,
      });

      tagDialog.value = false;
    }
  });
};

const closeNotebookDialog = () => {
  notebookFormRef.value?.resetFields();
  notebookDialog.value = false;
};

const closeTagDialog = () => {
  tagFormRef.value?.resetFields();
  tagDialog.value = false;
};

const resetNotebookForm = () => {
  notebookForm.id = "";
  notebookForm.name = "";
  notebookForm.description = "";
  notebookForm.icon = "";
  notebookForm.cls = "";
  notebookForm.sortOrder = 0;
};

const handleNotebookCommand = (command: string | number | object) => {
  if (command === "create") {
    resetNotebookForm();

    notebookDialogTitle.value = "添加笔记本";
    notebookDialog.value = true;
  } else if (command === "edit") {
    let notebook = props.notebooks.find((n) => n.id === props.activeNotebook);

    if (notebook) {
      notebookForm.id = notebook.id ?? "";
      notebookForm.name = notebook.name ?? "";
      notebookForm.description = notebook.description ?? "";
      notebookForm.icon = notebook.icon ?? "";
      notebookForm.cls = notebook.cls ?? "";
      notebookForm.sortOrder = notebook.sortOrder ?? 0;
    }

    notebookDialogTitle.value = "编辑笔记本";
    notebookDialog.value = true;
  } else if (command === "delete") {
    emit("deleteNotebook", props.activeNotebook);
  }
};

const resetTagForm = () => {
  tagForm.id = "";
  tagForm.name = "";
  tagForm.icon = "";
  tagForm.cls = "";
  tagForm.sortOrder = 0;
};

const handleTagCommand = (command: string | number | object) => {
  if (command === "create") {
    resetTagForm();

    tagDialogTitle.value = "添加标签";
    tagDialog.value = true;
  } else if (command === "edit") {
    let tag = props.tags.find((t) => t.id === props.activeTag);

    if (tag) {
      tagForm.id = tag.id ?? "";
      tagForm.name = tag.name ?? "";
      tagForm.icon = tag.icon ?? "";
      tagForm.cls = tag.cls ?? "";
      tagForm.sortOrder = tag.sortOrder ?? 0;
    }

    tagDialogTitle.value = "编辑标签";
    tagDialog.value = true;
  } else if (command === "delete") {
    emit("deleteTag", props.activeTag);
  }
};
</script>

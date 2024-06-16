<script lang="ts" setup>

import {ref} from 'vue'
import {InfoFilled, Tools} from "@element-plus/icons-vue";

import {ElMessage, ElNotification, UploadFile, UploadInstance, UploadRequestOptions} from 'element-plus'

import {invoke} from '@tauri-apps/api'
import {UploadRawFile} from "element-plus/es/components/upload/src/upload";

// 定义可以发射的事件
const emit = defineEmits(['onAddPlugin']);

function readFile(
    file: UploadRawFile,
    callback: (arrayBuffer: ArrayBuffer) => void
) {
  const reader = new FileReader();
  reader.onload = function (event) {
    const arrayBuffer = event.target!.result as ArrayBuffer;
    // arrayBuffer 包含了文件的字节流数据
    callback(arrayBuffer)
  };
  reader.onerror = function (error) {
    console.error('读取文件时发生错误:', error);
  };
  reader.readAsArrayBuffer(file);
}

const myUpload = (options: UploadRequestOptions) => {
  console.log("上传文件")

  console.log(options.file.name)
  readFile(options.file, (arrayBuffer) => {
    // console.log(arrayBuffer)
    const data = new Uint8Array(arrayBuffer);

    console.log()
    invoke('upload_file', {
      file: Array.from(data),
      filename: options.file.name
    })
  })

  return
}


const dialogVisible = ref(false);

// 一次只会上传一个文件~
const uploadDll = ref<UploadInstance>();
const hasDllFile = ref(false)
const onOpenDllSelector = () => {
  if (hasDllFile) {
    uploadDll.value!.clearFiles()
    // 避免显示出错
    hasDllFile.value = false
  }
}
const onRemoveDll = () => {
  hasDllFile.value = false
}


const uploadJson = ref<UploadInstance>();
const hasJsonFile = ref(false);
const onOpenJsonSelector = () => {
  if (hasJsonFile) {
    uploadJson.value!.clearFiles()
    hasJsonFile.value = false
  }
}
const onRemoveJson = () => {
  hasJsonFile.value = false
}


const beforeChangeDll = (file: UploadFile) => {

  if (file.name.endsWith('.dll')) {
    hasDllFile.value = true
  } else {
    hasDllFile.value = false
    uploadDll.value!.clearFiles()

    ElNotification({
      title: "Error",
      type: 'error',
      message: '只能上传 DLL 文件!'
    })
  }
  return;
};

const beforeChangeJson = (file: UploadFile) => {
  if (file.name.endsWith('.json')) {
    hasJsonFile.value = true
    console.log(hasJsonFile.value)
  } else {
    hasJsonFile.value = false
    uploadJson.value!.clearFiles()

    ElNotification({
      title: "Error",
      type: 'error',
      message: '只能上传 JSON 文件！!'
    })
  }
  return;
};


const uploadFiles = () => {
  if (!hasDllFile.value || !hasDllFile.value) {
    console.log(11)

    ElNotification({
      title: 'Warning',
      message: '请上传两个所需文件',
      type: 'warning',
    })
    return
  }

  uploadDll.value!.submit(); // 触发 DLL 文件上传
  uploadJson.value!.submit(); // 触发 JSON 文件上传
  dialogVisible.value = false; // 关闭对话框

  // 上传完毕清理文件
  clearAll()
  emit("onAddPlugin")

};

const onCancel = () => {

  clearAll()
}

const clearAll = () => {
  uploadDll.value!.clearFiles();
  uploadJson.value!.clearFiles();


  hasDllFile.value = false;
  hasJsonFile.value = false;

  dialogVisible.value = false
}

</script>


<template>

  <el-button type="primary" @click="dialogVisible = true">新增插件</el-button>

  <el-dialog
      v-model="dialogVisible"
      title="上传插件"
      align-center
  >
<!--    width="60%"-->
    <div class="flex-center">
      <el-upload
          ref="uploadDll"
          drag
          :auto-upload="false"
          action="#"
          :on-change="beforeChangeDll"
          :on-remove="onRemoveDll"
          @click="onOpenDllSelector"

          :http-request="myUpload"
      >
        <el-icon class="el-icon--upload">
          <Tools/>
        </el-icon>

        <div class="el-upload__text">
          <div v-if="hasDllFile">插件 Dll 文件准备添加</div>
          <div v-else>
            拖拽或 <em>点击此处</em> 上传 DLL 文件
          </div>
        </div>
      </el-upload>

      <el-upload
          ref="uploadJson"
          drag
          :auto-upload="false"
          action="#"
          :on-change="beforeChangeJson"
          :on-remove="onRemoveJson"
          @click="onOpenJsonSelector"

          :http-request="myUpload"

      >
        <el-icon class="el-icon--upload">
          <InfoFilled/>
        </el-icon>
        <div class="el-upload__text">
          <div v-if="hasJsonFile">
            插件 Json文件 准备添加
          </div>
          <div v-else>
            拖拽或 <em>点击此处</em> 上传 JSON 文件
          </div>
        </div>
      </el-upload>
    </div>


    <span slot="footer" class="float-right">
        <el-button @click="onCancel">取消</el-button>
        <el-button type="primary" @click="uploadFiles">确认上传</el-button>
      </span>
  </el-dialog>

</template>
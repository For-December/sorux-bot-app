<script lang="ts" setup>

import {ref} from 'vue'
import {InfoFilled, Tools} from "@element-plus/icons-vue";

import {ElNotification, UploadFile, UploadInstance, UploadRequestOptions} from 'element-plus'

import {invoke} from '@tauri-apps/api'
import {UploadRawFile} from "element-plus/es/components/upload/src/upload";

// 定义可以发射的事件
const emit = defineEmits(['onAddPlugin']);


const filesToUpload = new Map<string, UploadRawFile>()

const myUpload = (_: UploadRequestOptions) => {
  console.log("自定义上传方法")
  // console.log("临时存储文件")
  // if (options.file.name.endsWith(".json")) {
  //   filesToUpload.set("json", options.file)
  //
  // } else if (options.file.name.endsWith(".dll")) {
  //   filesToUpload.set("dll", options.file)
  // } else {
  //
  //   console.error("未知类型！")
  // }

  return


}


const beforeUpload = (rawFile: UploadRawFile) => {
  if (rawFile.name.endsWith(".json")) {
    filesToUpload.set("json", rawFile)

  } else if (rawFile.name.endsWith(".dll")) {
    filesToUpload.set("dll", rawFile)
  } else {
    console.error("未知类型！")
  }
}

const uploadPlugins = async (
    jsonFile: UploadRawFile,
    dllFile: UploadRawFile,
) => {
  console.log(jsonFile.name)
  if (!jsonFile || !dllFile) {
    ElNotification({
      title: "Error",
      type: "error",
      message: "文件为空？",
    })
    return
  }

  try {
    const jsonData = new Uint8Array(await jsonFile.arrayBuffer());
    const dllData = new Uint8Array(await dllFile.arrayBuffer());


    invoke('upload_plugin', {
      jsonFile: Array.from(jsonData),
      dllFile: Array.from(dllData),
      jsonFilename: jsonFile.name
    }).then((res) => {

      const msg = res as string;
      ElNotification({
        title: "Success",
        type: "success",
        message: msg,
      })

    }).catch((error) => {
      ElNotification({
        title: "Error",
        type: "error",
        message: error,
      })
    })


  } catch (error) {
    console.error('Error reading file:', error);
  }

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

  // 这里才真正上传插件
  // filesToUpload.value.forEach(uploadPlugins)
  console.log(filesToUpload)
  console.log(filesToUpload.get("json"))

  uploadPlugins(filesToUpload.get("json")!,
      filesToUpload.get("dll")!)
      .then(() => {
        filesToUpload.clear()
      })

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
          :before-upload="beforeUpload"
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
          :before-upload="beforeUpload"
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
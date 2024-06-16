<script lang="ts" setup>

import {computed, onMounted, ref} from "vue";
import PluginUpload from "@/components/PluginUpload.vue";
import {ElNotification} from "element-plus";
import {invoke} from "@tauri-apps/api";

const tableData = ref<Items.PluginItem[]>([])


const displayData = computed(() => {
  let count = 1
  return tableData.value.map(t => {
    t.num = count++
    return t
  })
})


//全局保存编辑的行号
const globalIndex = ref(-1)
const name = ref('')


//删除数据 从index位置开始，删除一行即可
const remove = (index: number) => {

  // 这里要向后端请求
  invoke('del_plugins',
      {filename: tableData.value[index].filename}
  ).then((res) => {
    if (res as String !== '') {
      tableData.value.splice(index, 1)
      ElNotification({
        title: "Success",
        type: "success",
        message: "插件删除成功！"
      })
      return
    }

    ElNotification({
      title: "Error",
      type: "error",
      message: "插件删除失败: " + res
    })

  })


}

onMounted(() => {
  fetchData()
})


const fetchData = () => {
  // 先后从端获取

  invoke('get_plugins', {}).then((data) => {
    console.log("d", data)
    tableData.value.length = 0;
    const plugins = data as Items.PluginItem[]
    plugins.forEach(p => {
      tableData.value.push(p)
    })
  })


}

const onAddPlugin = () => {
  setTimeout(() => {
    fetchData()
  }, 1000)


}

</script>
<template>

  <div style="padding: 10px">
    <PluginUpload @on-add-plugin="onAddPlugin"></PluginUpload>


    <div style="margin: 10px 0">
      <el-table :data="displayData" border style="width: 100%">
        <el-table-column prop="num" label="序号"/>
        <el-table-column prop="privilege" label="优先级"/>
        <el-table-column prop="name" label="插件名称"/>
        <el-table-column label="操作">
          <template #default="scope">
            <!--            <el-button link type="primary" size="small" @click="handleEdit(scope.row,scope.$index)">编辑-->
            <!--            </el-button>-->
            <el-button link type="danger" size="small" @click.prevent="remove(scope.$index)">删除</el-button>
          </template>
        </el-table-column>
      </el-table>
    </div>

  </div>
</template>

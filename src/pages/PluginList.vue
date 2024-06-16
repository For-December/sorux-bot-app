<script lang="ts" setup>

import {computed, onMounted, ref} from "vue";
import PluginUpload from "@/components/PluginUpload.vue";
import {ElNotification} from "element-plus";

const tableData = ref<Items.PluginItem[]>([])

let count = 1
const displayData = computed(() => {
  return tableData.value.map(t => {
    t.num = count++
    return t
  })
})


//全局保存编辑的行号
const globalIndex = ref(-1)
const name = ref('')


//删除数据 从index位置开始，删除一行即可
const remove = (index) => {

  // 先后从端获取

  // 这里要向后端请求
  tableData.value.splice(index, 1)
}

onMounted(() => {
  fetchData()
})


const fetchData = () => {
  tableData.value.push(
      {
        num: 0,
        name: 'example.txt',
        privilege: 1
      }
  )

}

const onAddPlugin = () => {
  fetchData()
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

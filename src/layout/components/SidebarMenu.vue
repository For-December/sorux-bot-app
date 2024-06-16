<script setup lang="ts">
import {computed, onMounted, ref} from 'vue'
import {
  Document,
  Menu as IconMenu,
  Location,
  Setting,
} from '@element-plus/icons-vue'
import {useRoute} from "vue-router";


const handleOpen = (key: string, keyPath: string[]) => {
  console.log(key, keyPath)
}
const handleClose = (key: string, keyPath: string[]) => {
  console.log(key, keyPath)
}
const route = useRoute()
const curPath = computed(() => {
  return route.path
})
</script>

<template>
  <el-menu router :default-active="curPath"
           class="el-menu-vertical-demo"
           text-color="#ffffff"
           background-color="#545c64"
           @open="handleOpen"
           @close="handleClose"
           :collapse="false"
  >
    <el-menu-item index="/home">
      <el-icon>
        <icon-menu/>
      </el-icon>
      <template #title>主页</template>
    </el-menu-item>

    <el-menu-item index="/plugin-list">
      <el-icon>
        <icon-menu/>
      </el-icon>
      <template #title>插件</template>
    </el-menu-item>


  </el-menu>
</template>

<style scoped>
.el-menu-vertical-demo:not(.el-menu--collapse) {
  user-select: none; /* 禁用用户选择*/
  width: 200px;
  height: 100vh;
  max-height: 100vh;
  min-height: 400px;
}
</style>
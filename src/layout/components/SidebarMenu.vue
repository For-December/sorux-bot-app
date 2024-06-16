<script setup lang="ts">
import {computed, onBeforeUnmount, onMounted, ref, shallowRef} from 'vue'
import {useRoute} from "vue-router";
import {routes} from "@/router"

const isCollapsed = ref(false);
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


function updateSidebarCollapse() {
  isCollapsed.value = window.innerWidth < 1024;
}

onMounted(() => {
  window.addEventListener('resize', updateSidebarCollapse);
  updateSidebarCollapse(); // Initial check on load
});

onBeforeUnmount(() => {
  window.removeEventListener('resize', updateSidebarCollapse);
});

// 引入外部组件时，用 shallowRef
const menuList = shallowRef(routes[1].children)

console.log(route.path)
console.log(menuList)
</script>

<template>
  <el-menu router :default-active="curPath"
           class="el-menu-vertical-demo"
           text-color="#ffffff"
           background-color="#545c64"
           @open="handleOpen"
           @close="handleClose"
           :collapse="isCollapsed"
  >
    <div v-for="subItem in menuList" :key="subItem.path">
      <!-- 有子菜单 -->
      <el-sub-menu v-if="subItem.children && subItem.children.length" :index="subItem.path">
        <el-icon>
          <component :is="subItem.meta!.icon"></component>
        </el-icon>

        <template #title>
          <span>{{ subItem.meta!.title }}</span>
        </template>
        <!--      <SubMenu :menuList="subItem.children" />-->
      </el-sub-menu>
      <!-- 无子菜单的一级菜单 -->
      <el-menu-item v-else :index="subItem.path">
        <!--      // @click="clickMenu(subItem)"-->
        <el-icon>
          <component :is="subItem.meta!.icon"></component>
        </el-icon>

        <template #title>

          <span>{{ subItem.meta!.title }}</span>
        </template>
      </el-menu-item>
    </div>
  </el-menu>
</template>

<style lang="scss" scoped>
//:root {
//  --el-menu-bg-color: "#545c64";
//  --el-menu-border-color: red;
//}

// 将element-plus内置的 sb 边框去掉，同时不脱离作用域
.el-menu {
  border-right: 0;
}

.el-menu-vertical-demo:not(.el-menu--collapse) {
  user-select: none; /* 禁用用户选择*/
  width: 10vw;
  height: 100vh;
  max-height: 100vh;
  min-height: 400px;

}

//:deep(.el-menu-vertical-demo):not(.el-menu--collapse) {
//
//}


</style>